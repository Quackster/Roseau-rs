use std::sync::atomic::{AtomicI32, Ordering};

pub const CHAT_FLOOD_SECONDS: i32 = 4;
pub const CHAT_FLOOD_WAIT: i32 = 10;
pub const MAX_CHAT_BEFORE_FLOOD: i32 = 4;

pub const MAX_ROOMS_PER_ACCOUNT: i32 = 40;

pub const MAX_RANK: i32 = 10;
pub const MAX_PETS_PER_ROOM: i32 = 15;
pub const MAX_USERBOTS_PER_ROOM: i32 = 2;
pub const MAX_ROLLERS_PER_ROOM: i32 = 35;
pub const MAX_USER_INROOM: i32 = 200;
pub const MAX_FAVORITES_ROOMS: i32 = 30;
pub const MAX_INVENTORY_ITEMS_COUNT: i32 = 1450;
pub const MAX_DISCOUNT_VALUE: i32 = 50;
pub const MAX_FRIENDS_DEFAULT: i32 = 300;
pub const MAX_FRIENDS_BASIC: i32 = 800;
pub const MAX_FRIENDS_VIP: i32 = 1000;
pub const MAX_FRIENDS_STAFF: i32 = 2500;
pub const MAX_FURNI_SELECTION: i32 = 10;

static ITEM_ID_COUNTER: AtomicI32 = AtomicI32::new(0);

pub fn next_item_id() -> i32 {
    ITEM_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1
}

pub fn item_id_counter() -> i32 {
    ITEM_ID_COUNTER.load(Ordering::Relaxed)
}

pub fn reset_item_id_counter(value: i32) {
    ITEM_ID_COUNTER.store(value, Ordering::Relaxed);
}
