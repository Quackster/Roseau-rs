use crate::game::room::settings::RoomType;
use crate::messages::outgoing::SystemBroadcast;
use crate::messages::OutgoingMessage;
use crate::runtime::RoseauApplicationRuntime;
use crate::server::PlayerNetworkEffect;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{Attribute, Print, SetAttribute};
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};
use std::cmp::min;
use std::collections::VecDeque;
use std::io::{self, IsTerminal, Stdout, Write};
use std::time::Duration;

const INPUT_POLL_TIMEOUT: Duration = Duration::from_millis(0);
const MAX_OBSERVED_LOG_LINES: usize = 300;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConsoleTab {
    Rooms,
    Users,
    Stats,
    Logs,
    Alert,
}

impl ConsoleTab {
    fn title(self) -> &'static str {
        match self {
            Self::Rooms => "Rooms",
            Self::Users => "Users",
            Self::Stats => "Stats",
            Self::Logs => "Logs",
            Self::Alert => "Alert",
        }
    }

    fn next(self) -> Self {
        match self {
            Self::Rooms => Self::Users,
            Self::Users => Self::Stats,
            Self::Stats => Self::Logs,
            Self::Logs => Self::Alert,
            Self::Alert => Self::Rooms,
        }
    }

    fn previous(self) -> Self {
        match self {
            Self::Rooms => Self::Alert,
            Self::Users => Self::Rooms,
            Self::Stats => Self::Users,
            Self::Logs => Self::Stats,
            Self::Alert => Self::Logs,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConsoleRoom {
    id: i32,
    name: String,
    owner: String,
    room_type: RoomType,
    state: String,
    server_port: i32,
    users_max: i32,
    users: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConsoleUser {
    connection_id: i32,
    user_id: i32,
    username: String,
    rank: i32,
    credits: i32,
    tickets: i32,
    server_port: i32,
    room_id: Option<i32>,
    room_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConsoleSnapshot {
    rooms: Vec<ConsoleRoom>,
    users: Vec<ConsoleUser>,
    active_connections: usize,
}

impl ConsoleSnapshot {
    fn capture(application: &RoseauApplicationRuntime) -> Self {
        let base_port = i32::from(
            application
                .startup_runtime()
                .startup_plan()
                .server_plan()
                .server_port(),
        );
        let private_port = i32::from(
            application
                .startup_runtime()
                .startup_plan()
                .server_plan()
                .private_server_port(),
        );
        let players = application.game().player_manager().players();

        let mut rooms = application
            .game()
            .room_manager()
            .loaded_rooms()
            .values()
            .map(|room| {
                let data = room.data();
                let server_port = match data.room_type() {
                    RoomType::Public => data.server_port(base_port),
                    RoomType::Private => private_port,
                };
                let mut users = players
                    .values()
                    .filter(|session| {
                        session.room_user().is_some_and(|room_user| {
                            room_user.room_id() == data.id()
                                && (data.room_type() == RoomType::Private
                                    || session.server_port() == server_port)
                        })
                    })
                    .map(|session| session.details().username().to_owned())
                    .collect::<Vec<_>>();
                users.sort_by_key(|name| name.to_ascii_lowercase());

                ConsoleRoom {
                    id: data.id(),
                    name: data.name().to_owned(),
                    owner: data.owner_name().to_owned(),
                    room_type: data.room_type(),
                    state: data.state().to_string(),
                    server_port,
                    users_max: data.users_max(),
                    users,
                }
            })
            .collect::<Vec<_>>();
        rooms.sort_by(|left, right| {
            right.users.len().cmp(&left.users.len()).then_with(|| {
                left.name
                    .to_ascii_lowercase()
                    .cmp(&right.name.to_ascii_lowercase())
            })
        });

        let mut users = players
            .values()
            .map(|session| {
                let room_id = session.room_user().map(|room_user| room_user.room_id());
                let room_name = room_id.and_then(|id| {
                    application
                        .game()
                        .room_manager()
                        .get_room_by_id(id)
                        .map(|room| room.data().name().to_owned())
                });

                ConsoleUser {
                    connection_id: session.connection_id(),
                    user_id: session.details().id(),
                    username: session.details().username().to_owned(),
                    rank: session.details().rank(),
                    credits: session.details().credits(),
                    tickets: session.details().tickets(),
                    server_port: session.server_port(),
                    room_id,
                    room_name,
                }
            })
            .collect::<Vec<_>>();
        users.sort_by_key(|user| user.username.to_ascii_lowercase());

        let active_connections = application
            .startup_runtime()
            .tcp_runtime()
            .map(|runtime| runtime.connections().len())
            .unwrap_or_default();

        Self {
            rooms,
            users,
            active_connections,
        }
    }
}

#[derive(Debug)]
pub struct RoseauConsole {
    enabled: bool,
    tab: ConsoleTab,
    selected: usize,
    alert_input: String,
    observed_logs: VecDeque<String>,
    notice: Option<String>,
}

impl RoseauConsole {
    pub fn start() -> Self {
        if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
            return Self::disabled();
        }

        let mut stdout = io::stdout();
        match terminal::enable_raw_mode()
            .and_then(|_| execute!(stdout, EnterAlternateScreen, Hide, Clear(ClearType::All)))
        {
            Ok(()) => Self {
                enabled: true,
                tab: ConsoleTab::Rooms,
                selected: 0,
                alert_input: String::new(),
                observed_logs: VecDeque::new(),
                notice: Some("Console ready. Left/right tabs, up/down selection, l logs, a alert, q normal output.".to_owned()),
            },
            Err(_) => {
                let _ = terminal::disable_raw_mode();
                Self::disabled()
            }
        }
    }

    pub fn disabled() -> Self {
        Self {
            enabled: false,
            tab: ConsoleTab::Rooms,
            selected: 0,
            alert_input: String::new(),
            observed_logs: VecDeque::new(),
            notice: None,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn observe_logs(&mut self, lines: impl IntoIterator<Item = String>) {
        for line in lines {
            self.observed_logs.push_back(line);
        }
        while self.observed_logs.len() > MAX_OBSERVED_LOG_LINES {
            self.observed_logs.pop_front();
        }
    }

    pub fn tick(&mut self, application: &mut RoseauApplicationRuntime) {
        if !self.enabled {
            return;
        }

        let snapshot = ConsoleSnapshot::capture(application);
        while event::poll(INPUT_POLL_TIMEOUT).unwrap_or(false) {
            match event::read() {
                Ok(Event::Key(key)) => self.handle_key(key, application, &snapshot),
                Ok(Event::Resize(_, _)) => {}
                Ok(_) => {}
                Err(_) => break,
            }
        }
        let snapshot = ConsoleSnapshot::capture(application);
        self.clamp_selection(&snapshot);
        if let Err(error) = self.render(&snapshot) {
            self.notice = Some(format!("console render failed: {error}"));
        }
    }

    fn handle_key(
        &mut self,
        key: KeyEvent,
        application: &mut RoseauApplicationRuntime,
        snapshot: &ConsoleSnapshot,
    ) {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.stop();
            return;
        }

        match self.tab {
            ConsoleTab::Alert => self.handle_alert_key(key, application),
            _ => self.handle_navigation_key(key, snapshot),
        }
    }

    fn handle_navigation_key(&mut self, key: KeyEvent, snapshot: &ConsoleSnapshot) {
        match key.code {
            KeyCode::Left => self.select_tab(self.tab.previous()),
            KeyCode::Right | KeyCode::Tab => self.select_tab(self.tab.next()),
            KeyCode::Up => self.selected = self.selected.saturating_sub(1),
            KeyCode::Down => {
                let max = self.current_len(snapshot).saturating_sub(1);
                self.selected = min(self.selected.saturating_add(1), max);
            }
            KeyCode::Home => self.selected = 0,
            KeyCode::End => self.selected = self.current_len(snapshot).saturating_sub(1),
            KeyCode::Char('a') => self.select_tab(ConsoleTab::Alert),
            KeyCode::Char('l') => self.select_tab(ConsoleTab::Logs),
            KeyCode::Char('q') | KeyCode::Esc => self.stop(),
            _ => {}
        }
    }

    fn handle_alert_key(&mut self, key: KeyEvent, application: &mut RoseauApplicationRuntime) {
        match key.code {
            KeyCode::Esc => {
                self.alert_input.clear();
                self.select_tab(ConsoleTab::Rooms);
            }
            KeyCode::Left if self.alert_input.is_empty() => self.select_tab(self.tab.previous()),
            KeyCode::Right | KeyCode::Tab if self.alert_input.is_empty() => {
                self.select_tab(self.tab.next())
            }
            KeyCode::Enter => self.send_alert(application),
            KeyCode::Backspace => {
                self.alert_input.pop();
            }
            KeyCode::Char('q') if self.alert_input.is_empty() => self.stop(),
            KeyCode::Char(c) => {
                if !key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.alert_input.push(c);
                }
            }
            _ => {}
        }
    }

    fn send_alert(&mut self, application: &mut RoseauApplicationRuntime) {
        let message = self.alert_input.trim();
        if message.is_empty() {
            self.notice = Some("Alert message is empty.".to_owned());
            return;
        }

        let packet = SystemBroadcast::new(message).compose().get();
        let effects = application
            .game()
            .player_manager()
            .players()
            .values()
            .map(|session| PlayerNetworkEffect::WriteResponse {
                connection_id: session.connection_id(),
                packet: packet.clone(),
            })
            .collect::<Vec<_>>();
        let attempted = effects.len();
        let unapplied = application
            .startup_runtime_mut()
            .apply_network_effects(effects);
        let delivered = attempted.saturating_sub(unapplied.len());
        self.notice = Some(format!(
            "Sent alert to {delivered}/{attempted} online sessions."
        ));
        self.alert_input.clear();
    }

    fn select_tab(&mut self, tab: ConsoleTab) {
        self.tab = tab;
        self.selected = 0;
    }

    fn current_len(&self, snapshot: &ConsoleSnapshot) -> usize {
        match self.tab {
            ConsoleTab::Rooms => snapshot.rooms.len(),
            ConsoleTab::Users => snapshot.users.len(),
            ConsoleTab::Logs => self.observed_logs.len(),
            ConsoleTab::Stats | ConsoleTab::Alert => 1,
        }
    }

    fn clamp_selection(&mut self, snapshot: &ConsoleSnapshot) {
        self.selected = min(self.selected, self.current_len(snapshot).saturating_sub(1));
    }

    fn render(&mut self, snapshot: &ConsoleSnapshot) -> io::Result<()> {
        let mut stdout = io::stdout();
        let (cols, rows) = terminal::size().unwrap_or((100, 30));
        queue!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;
        self.render_header(&mut stdout, snapshot)?;
        match self.tab {
            ConsoleTab::Rooms => self.render_rooms(&mut stdout, snapshot, cols, rows)?,
            ConsoleTab::Users => self.render_users(&mut stdout, snapshot, cols, rows)?,
            ConsoleTab::Stats => self.render_stats(&mut stdout, snapshot, cols, rows)?,
            ConsoleTab::Logs => self.render_logs(&mut stdout, cols, rows)?,
            ConsoleTab::Alert => self.render_alert(&mut stdout, snapshot, cols, rows)?,
        }
        self.render_footer(&mut stdout, rows)?;
        stdout.flush()
    }

    fn render_header(&self, stdout: &mut Stdout, snapshot: &ConsoleSnapshot) -> io::Result<()> {
        queue!(
            stdout,
            SetAttribute(Attribute::Bold),
            Print("Roseau Console"),
            SetAttribute(Attribute::Reset),
            Print(format!(
                "  sessions:{} rooms:{} connections:{}\r\n",
                snapshot.users.len(),
                snapshot.rooms.len(),
                snapshot.active_connections
            ))
        )?;

        for tab in [
            ConsoleTab::Rooms,
            ConsoleTab::Users,
            ConsoleTab::Stats,
            ConsoleTab::Logs,
            ConsoleTab::Alert,
        ] {
            if tab == self.tab {
                queue!(stdout, SetAttribute(Attribute::Reverse))?;
            }
            queue!(stdout, Print(format!(" {} ", tab.title())))?;
            if tab == self.tab {
                queue!(stdout, SetAttribute(Attribute::Reset))?;
            }
            queue!(stdout, Print(" "))?;
        }
        queue!(stdout, Print("\r\n\r\n"))
    }

    fn render_rooms(
        &self,
        stdout: &mut Stdout,
        snapshot: &ConsoleSnapshot,
        cols: u16,
        rows: u16,
    ) -> io::Result<()> {
        queue!(stdout, Print("Loaded rooms\r\n"))?;
        let list_height = rows.saturating_sub(10) as usize;
        for (index, room) in snapshot.rooms.iter().take(list_height).enumerate() {
            self.render_marker(stdout, index)?;
            queue!(
                stdout,
                Print(format!(
                    "#{:<5} {:<28} {:<7} users {:>2}/{:<2} port {}\r\n",
                    room.id,
                    truncate(&room.name, 28),
                    room_type_label(room.room_type),
                    room.users.len(),
                    room.users_max,
                    room.server_port
                ))
            )?;
        }
        if snapshot.rooms.is_empty() {
            queue!(stdout, Print("No loaded rooms.\r\n"))?;
            return Ok(());
        }

        let selected = &snapshot.rooms[self.selected];
        queue!(stdout, Print("\r\n"))?;
        queue!(
            stdout,
            SetAttribute(Attribute::Bold),
            Print(truncate(&selected.name, cols.saturating_sub(2) as usize)),
            SetAttribute(Attribute::Reset),
            Print(format!(
                "\r\nid:{} owner:{} state:{} port:{}\r\n",
                selected.id, selected.owner, selected.state, selected.server_port
            ))
        )?;
        if selected.users.is_empty() {
            queue!(stdout, Print("Users inside: none\r\n"))?;
        } else {
            queue!(stdout, Print("Users inside: "))?;
            queue!(
                stdout,
                Print(truncate(
                    &selected.users.join(", "),
                    cols.saturating_sub(15) as usize
                ))
            )?;
            queue!(stdout, Print("\r\n"))?;
        }
        Ok(())
    }

    fn render_users(
        &self,
        stdout: &mut Stdout,
        snapshot: &ConsoleSnapshot,
        _cols: u16,
        rows: u16,
    ) -> io::Result<()> {
        queue!(stdout, Print("Current online users\r\n"))?;
        let list_height = rows.saturating_sub(7) as usize;
        for (index, user) in snapshot.users.iter().take(list_height).enumerate() {
            self.render_marker(stdout, index)?;
            queue!(
                stdout,
                Print(format!(
                    "#{:<4} {:<20} rank {:<3} cr {:<6} tk {:<5} {}\r\n",
                    user.user_id,
                    truncate(&user.username, 20),
                    user.rank,
                    user.credits,
                    user.tickets,
                    user.room_name.as_deref().unwrap_or("hotel view")
                ))
            )?;
        }
        if snapshot.users.is_empty() {
            queue!(stdout, Print("No online users.\r\n"))?;
        }
        Ok(())
    }

    fn render_stats(
        &self,
        stdout: &mut Stdout,
        snapshot: &ConsoleSnapshot,
        _cols: u16,
        _rows: u16,
    ) -> io::Result<()> {
        let occupied_rooms = snapshot
            .rooms
            .iter()
            .filter(|room| !room.users.is_empty())
            .count();
        let public_rooms = snapshot
            .rooms
            .iter()
            .filter(|room| room.room_type == RoomType::Public)
            .count();
        let private_rooms = snapshot.rooms.len().saturating_sub(public_rooms);
        let hotel_view_users = snapshot
            .users
            .iter()
            .filter(|user| user.room_id.is_none())
            .count();

        queue!(
            stdout,
            Print("User stats\r\n"),
            Print(format!(
                "Online sessions:       {}\r\n",
                snapshot.users.len()
            )),
            Print(format!(
                "Active connections:    {}\r\n",
                snapshot.active_connections
            )),
            Print(format!("Hotel view users:      {}\r\n", hotel_view_users)),
            Print(format!(
                "Loaded rooms:          {}\r\n",
                snapshot.rooms.len()
            )),
            Print(format!("Occupied rooms:        {}\r\n", occupied_rooms)),
            Print(format!("Public rooms:          {}\r\n", public_rooms)),
            Print(format!("Private rooms:         {}\r\n", private_rooms))
        )
    }

    fn render_logs(&self, stdout: &mut Stdout, cols: u16, rows: u16) -> io::Result<()> {
        queue!(stdout, Print("Server log\r\n"))?;
        if self.observed_logs.is_empty() {
            queue!(
                stdout,
                Print("No server log lines observed since the console opened.\r\n")
            )?;
            return Ok(());
        }

        let height = rows.saturating_sub(7) as usize;
        let start = self.observed_logs.len().saturating_sub(height);
        for line in self.observed_logs.iter().skip(start) {
            queue!(
                stdout,
                Print(truncate(line, cols.saturating_sub(1) as usize)),
                Print("\r\n")
            )?;
        }
        Ok(())
    }

    fn render_alert(
        &self,
        stdout: &mut Stdout,
        snapshot: &ConsoleSnapshot,
        cols: u16,
        _rows: u16,
    ) -> io::Result<()> {
        queue!(
            stdout,
            Print("Send hotel alert\r\n"),
            Print(format!(
                "Recipients: {} online sessions\r\n\r\n",
                snapshot.users.len()
            )),
            Print("> "),
            SetAttribute(Attribute::Bold),
            Print(truncate(&self.alert_input, cols.saturating_sub(3) as usize)),
            SetAttribute(Attribute::Reset),
            Print("\r\n\r\nPress Enter to send. Esc cancels the draft.\r\n")
        )
    }

    fn render_marker(&self, stdout: &mut Stdout, index: usize) -> io::Result<()> {
        if index == self.selected {
            queue!(
                stdout,
                SetAttribute(Attribute::Reverse),
                Print(">"),
                SetAttribute(Attribute::Reset)
            )
        } else {
            queue!(stdout, Print(" "))
        }
    }

    fn render_footer(&self, stdout: &mut Stdout, rows: u16) -> io::Result<()> {
        let y = rows.saturating_sub(2);
        queue!(stdout, MoveTo(0, y), Clear(ClearType::CurrentLine))?;
        if let Some(notice) = &self.notice {
            queue!(stdout, Print(notice), Print("\r\n"))?;
        }
        queue!(
            stdout,
            Clear(ClearType::CurrentLine),
            Print("Left/Right tabs  Up/Down select  l logs  a alert  q normal output")
        )
    }

    fn stop(&mut self) {
        self.enabled = false;
        let mut stdout = io::stdout();
        let _ = execute!(stdout, Show, LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}

impl Drop for RoseauConsole {
    fn drop(&mut self) {
        if self.enabled {
            self.stop();
        }
    }
}

fn room_type_label(room_type: RoomType) -> &'static str {
    match room_type {
        RoomType::Public => "public",
        RoomType::Private => "private",
    }
}

fn truncate(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_owned();
    }

    let mut truncated = value
        .chars()
        .take(max_chars.saturating_sub(1))
        .collect::<String>();
    truncated.push('.');
    truncated
}
