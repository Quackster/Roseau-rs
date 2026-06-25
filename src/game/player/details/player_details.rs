use crate::messages::outgoing::{PhTickets, WalletBalance};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerDetails {
    id: i32,
    pub(super) username: String,
    pub(super) mission: String,
    pub(super) figure: String,
    pub(super) email: String,
    rank: i32,
    credits: i32,
    pub(super) sex: String,
    pub(super) country: String,
    pub(super) badge: String,
    pub(super) birthday: String,
    pool_figure: String,
    password: String,
    personal_greeting: String,
    authenticated: bool,
    last_online: i64,
    tickets: i32,
}

impl PlayerDetails {
    pub fn new() -> Self {
        Self {
            id: -1,
            username: String::new(),
            mission: String::new(),
            figure: String::new(),
            email: String::new(),
            rank: 0,
            credits: 0,
            sex: String::new(),
            country: String::new(),
            badge: String::new(),
            birthday: String::new(),
            pool_figure: String::new(),
            password: String::new(),
            personal_greeting: String::new(),
            authenticated: false,
            last_online: 0,
            tickets: 0,
        }
    }

    pub fn fill_basic(
        &mut self,
        id: i32,
        username: impl Into<String>,
        mission: impl Into<String>,
        figure: impl Into<String>,
    ) {
        self.id = id;
        self.username = username.into();
        self.mission = mission.into();
        self.figure = figure.into();
    }

    #[allow(clippy::too_many_arguments)]
    pub fn fill_full(
        &mut self,
        id: i32,
        username: impl Into<String>,
        mission: impl Into<String>,
        figure: impl Into<String>,
        pool_figure: impl Into<String>,
        email: impl Into<String>,
        rank: i32,
        credits: i32,
        sex: impl Into<String>,
        country: impl Into<String>,
        badge: impl Into<String>,
        birthday: impl Into<String>,
        last_online: i64,
        personal_greeting: impl Into<String>,
        tickets: i32,
    ) {
        self.id = id;
        self.username = username.into();
        self.mission = mission.into();
        self.figure = figure.into();
        self.pool_figure = pool_figure.into();
        self.email = email.into();
        self.rank = rank;
        self.credits = credits;
        self.sex = sex.into();
        self.country = country.into();
        self.badge = badge.into();
        self.birthday = birthday.into();
        self.last_online = last_online;
        self.personal_greeting = personal_greeting.into();
        self.tickets = tickets;
    }

    pub fn wallet_balance(&self) -> WalletBalance {
        WalletBalance::new(self.credits)
    }

    pub fn ph_tickets(&self) -> PhTickets {
        PhTickets::new(self.tickets)
    }

    pub fn has_fuse(&self, _fuse: &str) -> bool {
        false
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn is_authenticated(&self) -> bool {
        self.authenticated
    }

    pub fn set_authenticated(&mut self, authenticated: bool) {
        self.authenticated = authenticated;
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn set_username(&mut self, username: impl Into<String>) {
        self.username = username.into();
    }

    pub fn mission(&self) -> &str {
        &self.mission
    }

    pub fn set_mission(&mut self, mission: impl Into<String>) {
        self.mission = mission.into();
    }

    pub fn figure(&self) -> &str {
        &self.figure
    }

    pub fn set_figure(&mut self, figure: impl Into<String>) {
        self.figure = figure.into();
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn set_email(&mut self, email: impl Into<String>) {
        self.email = email.into();
    }

    pub fn rank(&self) -> i32 {
        self.rank
    }

    pub fn credits(&self) -> i32 {
        self.credits
    }

    pub fn set_credits(&mut self, new_total: i32) {
        self.credits = new_total;
    }

    pub fn sex(&self) -> &str {
        &self.sex
    }

    pub fn set_sex(&mut self, sex: impl Into<String>) {
        self.sex = sex.into();
    }

    pub fn country(&self) -> &str {
        &self.country
    }

    pub fn badge(&self) -> &str {
        &self.badge
    }

    pub fn birthday(&self) -> &str {
        &self.birthday
    }

    pub fn pool_figure(&self) -> &str {
        &self.pool_figure
    }

    pub fn set_pool_figure(&mut self, pool_figure: impl Into<String>) {
        self.pool_figure = pool_figure.into();
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn set_password(&mut self, password: impl Into<String>) {
        self.password = password.into();
    }

    pub fn last_online(&self) -> i64 {
        self.last_online
    }

    pub fn personal_greeting(&self) -> &str {
        &self.personal_greeting
    }

    pub fn set_personal_greeting(&mut self, personal_greeting: impl Into<String>) {
        self.personal_greeting = personal_greeting.into();
    }

    pub fn tickets(&self) -> i32 {
        self.tickets
    }

    pub fn set_tickets(&mut self, new_total: i32) {
        self.tickets = new_total;
    }
}

impl Default for PlayerDetails {
    fn default() -> Self {
        Self::new()
    }
}
