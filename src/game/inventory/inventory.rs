use std::collections::HashMap;

use crate::game::item::Item;
use crate::settings::MAX_ITEMS_PER_PAGE;

#[derive(Debug, Clone, PartialEq)]
pub struct Inventory {
    items: Vec<Item>,
    paginated_items: HashMap<usize, Vec<Item>>,
    cursor: usize,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            paginated_items: HashMap::new(),
            cursor: 0,
        }
    }

    pub fn with_items(items: impl IntoIterator<Item = Item>) -> Self {
        let mut inventory = Self {
            items: items.into_iter().collect(),
            paginated_items: HashMap::new(),
            cursor: 0,
        };
        inventory.refresh_pagination();
        inventory
    }

    pub fn refresh_pagination(&mut self) {
        self.paginated_items.clear();

        let mut page_id = 0;
        let mut counter = 0;

        for item in &self.items {
            if counter > (MAX_ITEMS_PER_PAGE - 1) {
                page_id += 1;
                counter = 0;
            } else {
                counter += 1;
            }

            self.paginated_items
                .entry(page_id)
                .or_default()
                .push(item.clone());
        }
    }

    pub fn get_item(&self, id: i32) -> Option<&Item> {
        self.items.iter().find(|item| item.id() == id)
    }

    pub fn remove_item_by_id(&mut self, id: i32, refresh_pagination: bool) -> Option<Item> {
        let index = self.items.iter().position(|item| item.id() == id)?;
        let item = self.items.remove(index);

        if refresh_pagination {
            self.refresh_pagination();
        }

        Some(item)
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
        self.refresh_pagination();
    }

    pub fn refresh(&mut self, mode: &str) -> InventoryRefresh<'_> {
        if self.paginated_items.is_empty() {
            return InventoryRefresh::Empty;
        }

        match mode {
            "last" => self.cursor = self.paginated_items.len().saturating_sub(1),
            "new" => self.cursor = 0,
            "next" => self.cursor += 1,
            _ => {}
        }

        if !self.paginated_items.contains_key(&self.cursor) {
            self.cursor = 0;
        }

        self.paginated_items
            .get(&self.cursor)
            .map(|items| InventoryRefresh::Page {
                cursor: self.cursor,
                items,
            })
            .unwrap_or(InventoryRefresh::Empty)
    }

    pub fn dispose(&mut self) {
        self.items.clear();
        self.paginated_items.clear();
        self.cursor = 0;
    }

    pub fn items(&self) -> &[Item] {
        &self.items
    }

    pub fn paginated_items(&self) -> &HashMap<usize, Vec<Item>> {
        &self.paginated_items
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InventoryRefresh<'a> {
    Page { cursor: usize, items: &'a [Item] },
    Empty,
}
