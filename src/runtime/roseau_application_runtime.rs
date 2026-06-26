use crate::dao::PublicRoomDescriptor;
use crate::game::catalogue::{CatalogueDeal, CatalogueItem, CatalogueManager};
use crate::game::room::{RoomData, RoomSummary};
use crate::game::Game;
use crate::runtime::RoseauStartupRuntime;
use crate::runtime::{BootstrapError, RoseauBootstrap, RoseauRuntime, RoseauStartupPlan};
use crate::server::ServerSocketBinder;

pub struct RoseauApplicationRuntime {
    pub(super) runtime: RoseauRuntime,
    pub(super) game: Game,
    pub(super) startup_runtime: RoseauStartupRuntime,
    pub(super) startup_log_lines: Vec<String>,
    pub(super) resolved_config_ip: Option<String>,
}

impl RoseauApplicationRuntime {
    pub fn prepare<B: ServerSocketBinder>(
        bootstrap: &RoseauBootstrap,
        binder: &B,
        public_rooms: impl IntoIterator<Item = PublicRoomDescriptor>,
        first_connection_id: i32,
        resolved_config_ip: Option<&str>,
    ) -> Result<Self, BootstrapError> {
        let runtime = RoseauRuntime::load(bootstrap)?;
        let mut game = Game::new();
        game.load(runtime.hotel_config())?;
        let server_plan = bootstrap.server_plan(runtime.main_config(), public_rooms)?;
        let startup_plan = RoseauStartupPlan::from_server_plan(server_plan)?;
        let startup_runtime =
            RoseauStartupRuntime::prepare(&runtime, startup_plan, binder, first_connection_id)?;
        let startup_log_lines = startup_runtime.startup_log_lines(resolved_config_ip);

        Ok(Self {
            runtime,
            game,
            startup_runtime,
            startup_log_lines,
            resolved_config_ip: resolved_config_ip.map(str::to_owned),
        })
    }

    pub fn write_pending_server_logs(&mut self) {
        for line in self.drain_pending_server_log_lines() {
            println!("{line}");
        }
    }

    pub fn drain_pending_server_log_lines(&mut self) -> Vec<String> {
        let logger = self.runtime.logger().clone();
        let lines = self.startup_runtime.drain_pending_logs();
        for line in &lines {
            let _ = logger.write_output_line(&line);
        }
        lines
    }

    pub fn load_public_rooms(&mut self, public_rooms: impl IntoIterator<Item = RoomData>) {
        for (order_id, room) in public_rooms.into_iter().enumerate() {
            let mut summary = RoomSummary::new(room);
            summary.set_order_id(i32::try_from(order_id).unwrap_or(i32::MAX));
            self.game.room_manager_mut().add(summary);
        }
    }

    pub fn load_catalogue(
        &mut self,
        items: impl IntoIterator<Item = CatalogueItem>,
        deals: impl IntoIterator<Item = CatalogueDeal>,
    ) {
        *self.game.catalogue_manager_mut() = CatalogueManager::with_items_and_deals(items, deals);
    }
}
