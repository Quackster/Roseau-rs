use crate::dao::mysql::entity::{
    CatalogueDealRow, CatalogueRow, ItemDefinitionRow, ItemRow, MessengerMessageRow, RoomModelRow,
    RoomPublicConnectionRow, RoomPublicItemRow, RoomRow, UserPermissionRow, UserRow,
};
use crate::game::catalogue::{CatalogueDeal, CatalogueItem};
use crate::game::item::{Item, ItemDefinition, ParseItemError};
use crate::game::messenger::MessengerMessage;
use crate::game::player::{Permission, PlayerDetails};
use crate::game::room::model::{ParseRoomModelError, Position, RoomModel};
use crate::game::room::settings::RoomType;
use crate::game::room::{RoomConnection, RoomData};

pub fn catalogue_item_from_row(row: &CatalogueRow) -> CatalogueItem {
    CatalogueItem::new(&row.call_id, row.definition_id, row.credits)
}

pub fn catalogue_deal_from_row(row: &CatalogueDealRow) -> CatalogueDeal {
    CatalogueDeal::new(
        &row.call_id,
        row.products
            .split(',')
            .filter(|product| !product.is_empty())
            .map(str::trim),
        row.cost,
    )
}

pub fn item_definition_from_row(row: &ItemDefinitionRow) -> ItemDefinition {
    ItemDefinition::new(
        row.id,
        &row.sprite,
        &row.color,
        row.length,
        row.width,
        row.height,
        &row.behaviour,
        &row.name,
        &row.description,
        &row.data_class,
    )
}

pub fn item_from_row(row: &ItemRow, definition: ItemDefinition) -> Result<Item, ParseItemError> {
    Item::new(
        row.id,
        row.room_id,
        row.user_id,
        &row.x,
        row.y,
        row.z,
        row.rotation,
        definition,
        "",
        Some(row.extra_data.clone()),
    )
}

pub fn public_item_from_row(
    row: &RoomPublicItemRow,
    room_id: i32,
    definition: ItemDefinition,
) -> Result<Item, ParseItemError> {
    Item::new(
        row.id,
        room_id,
        -1,
        &row.x,
        row.y,
        row.z,
        row.rotation,
        definition,
        &row.object,
        Some(row.data.clone()),
    )
}

pub fn room_model_from_row(row: &RoomModelRow) -> Result<RoomModel, ParseRoomModelError> {
    RoomModel::new(
        &row.id,
        &row.heightmap,
        row.door_x,
        row.door_y,
        row.door_z,
        row.door_dir,
        row.has_pool,
        row.disable_height_check,
    )
}

pub fn messenger_message_from_row(row: &MessengerMessageRow) -> MessengerMessage {
    MessengerMessage::new(row.id, row.to_id, row.from_id, row.time_sent, &row.message)
}

pub fn player_details_from_row(row: &UserRow) -> PlayerDetails {
    let mut details = PlayerDetails::new();
    details.fill_full(
        row.id,
        &row.username,
        &row.mission,
        &row.figure,
        &row.pool_figure,
        &row.email,
        row.rank,
        row.credits,
        &row.sex,
        &row.country,
        &row.badge,
        &row.birthday,
        row.last_online,
        &row.personal_greeting,
        row.tickets,
    );
    details.set_password(&row.password);
    details
}

pub fn permission_from_row(row: &UserPermissionRow) -> Permission {
    Permission::new(&row.permission, row.inheritable, row.rank)
}

pub fn room_data_from_row(row: &RoomRow, owner_name: impl Into<String>) -> RoomData {
    let room_type = RoomType::from_code(row.room_type);
    RoomData::new(
        row.id,
        row.hidden,
        room_type,
        row.owner_id,
        owner_name,
        &row.name,
        row.state,
        &row.password,
        row.users_max,
        &row.description,
        &row.model,
        &row.cct,
        &row.wallpaper,
        &row.floor,
        row.all_super_user,
        row.show_owner_name,
    )
}

pub fn room_connection_from_row(row: &RoomPublicConnectionRow) -> RoomConnection {
    let door_position =
        Position::with_rotation(row.door_x, row.door_y, row.door_z as f64, row.door_rotation);
    RoomConnection::new(row.room_id, row.to_id, door_position)
}

pub fn room_connections_from_row(row: &RoomPublicConnectionRow) -> Vec<RoomConnection> {
    let door_position =
        Position::with_rotation(row.door_x, row.door_y, row.door_z as f64, row.door_rotation);
    row.coordinates
        .split_whitespace()
        .filter_map(|coordinate| Position::parse(coordinate).ok())
        .map(|source| RoomConnection::with_source(row.room_id, row.to_id, source, door_position))
        .collect()
}

#[cfg(test)]
#[path = "mapper_tests.rs"]
mod tests;
