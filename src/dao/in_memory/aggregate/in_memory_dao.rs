use std::rc::Rc;

use crate::dao::in_memory::{
    InMemoryCatalogueDao, InMemoryInventoryDao, InMemoryItemDao, InMemoryMessengerDao,
    InMemoryPlayerDao, InMemoryRoomDao,
};

#[derive(Debug, Default)]
pub struct InMemoryDao {
    catalogue: InMemoryCatalogueDao,
    inventory: InMemoryInventoryDao,
    item: Rc<InMemoryItemDao>,
    messenger: InMemoryMessengerDao,
    player: InMemoryPlayerDao,
    room: InMemoryRoomDao,
    connected: bool,
}

impl InMemoryDao {
    pub fn new(player: InMemoryPlayerDao) -> Self {
        let item = Rc::new(InMemoryItemDao::new());
        Self {
            catalogue: InMemoryCatalogueDao::new(),
            inventory: InMemoryInventoryDao::shared(Rc::clone(&item)),
            item,
            messenger: InMemoryMessengerDao::new(),
            player,
            room: InMemoryRoomDao::new(),
            connected: true,
        }
    }

    pub fn with_daos(
        catalogue: InMemoryCatalogueDao,
        item: Rc<InMemoryItemDao>,
        messenger: InMemoryMessengerDao,
        player: InMemoryPlayerDao,
        room: InMemoryRoomDao,
    ) -> Self {
        Self {
            catalogue,
            inventory: InMemoryInventoryDao::shared(Rc::clone(&item)),
            item,
            messenger,
            player,
            room,
            connected: true,
        }
    }

    pub fn connect(&mut self) -> bool {
        self.connected = true;
        true
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn player(&self) -> &InMemoryPlayerDao {
        &self.player
    }

    pub fn catalogue(&self) -> &InMemoryCatalogueDao {
        &self.catalogue
    }

    pub fn inventory(&self) -> &InMemoryInventoryDao {
        &self.inventory
    }

    pub fn item(&self) -> &InMemoryItemDao {
        &self.item
    }

    pub fn messenger(&self) -> &InMemoryMessengerDao {
        &self.messenger
    }

    pub fn room(&self) -> &InMemoryRoomDao {
        &self.room
    }
}

#[cfg(test)]
#[path = "in_memory_dao_tests.rs"]
mod tests;
