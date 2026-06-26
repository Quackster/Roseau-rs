use std::collections::HashMap;

use crate::dao::mysql::entity::{
    RoomBotRow, RoomModelRow, RoomPublicConnectionRow, RoomRightRow, RoomRow,
};
use crate::dao::mysql::mapper::{
    room_connections_from_row, room_data_from_row, room_model_from_row,
};
use crate::dao::mysql::SqlExecutionResult;
use crate::dao::{DaoError, PublicRoomDescriptor};
use crate::game::player::Bot;
use crate::game::room::model::{Position, RoomModel};
use crate::game::room::RoomSummary;
use crate::game::room::{RoomConnection, RoomData};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoomResultMapper;

impl RoomResultMapper {
    pub fn rooms(result: SqlExecutionResult, owner_name: &str) -> Result<Vec<RoomData>, DaoError> {
        result.map_rows(|row| {
            let room_row = RoomRow::try_from(row)?;
            Ok(room_data_from_row(&room_row, owner_name))
        })
    }

    pub fn room_summaries(
        result: SqlExecutionResult,
        owner_name: &str,
    ) -> Result<Vec<RoomSummary>, DaoError> {
        result.map_rows(|row| {
            let room_row = RoomRow::try_from(row)?;
            Ok(room_summary_from_row(&room_row, owner_name))
        })
    }

    pub fn optional_room(
        result: SqlExecutionResult,
        owner_name: &str,
    ) -> Result<Option<RoomData>, DaoError> {
        result
            .optional_first_row()?
            .as_ref()
            .map(|row| {
                let room_row = RoomRow::try_from(row)?;
                Ok(room_data_from_row(&room_row, owner_name))
            })
            .transpose()
    }

    pub fn public_room_descriptors(
        result: SqlExecutionResult,
    ) -> Result<Vec<PublicRoomDescriptor>, DaoError> {
        result.map_rows(|row| {
            Ok(PublicRoomDescriptor::new(
                row.required_i32("id")?,
                row.required_string("name")?,
            ))
        })
    }

    pub fn room_rights(result: SqlExecutionResult) -> Result<Vec<i32>, DaoError> {
        result.map_rows(|row| {
            let right_row = RoomRightRow::try_from(row)?;
            Ok(right_row.user_id)
        })
    }

    pub fn room_connections(result: SqlExecutionResult) -> Result<Vec<RoomConnection>, DaoError> {
        Ok(result
            .map_rows(|row| {
                let connection_row = RoomPublicConnectionRow::try_from(row)?;
                Ok(room_connections_from_row(&connection_row))
            })?
            .into_iter()
            .flatten()
            .collect())
    }

    pub fn room_models(result: SqlExecutionResult) -> Result<HashMap<String, RoomModel>, DaoError> {
        Ok(result
            .map_rows(|row| {
                let model_row = RoomModelRow::try_from(row)?;
                let model = room_model_from_row(&model_row)
                    .map_err(|error| DaoError::new(format!("Invalid room model row: {error}")))?;
                Ok((model.name().to_owned(), model))
            })?
            .into_iter()
            .collect())
    }

    pub fn bots(result: SqlExecutionResult) -> Result<Vec<Bot>, DaoError> {
        result.map_rows(|row| {
            let bot_row = RoomBotRow::try_from(row)?;
            bot_from_row(&bot_row)
        })
    }

    pub fn created_room_id(result: SqlExecutionResult) -> Result<i32, DaoError> {
        result.require_i32_insert_id("room id")
    }
}

fn room_summary_from_row(row: &RoomRow, owner_name: &str) -> RoomSummary {
    let mut summary = RoomSummary::new(room_data_from_row(row, owner_name));
    summary.set_order_id(row.order_id);
    summary.set_player_count(usize::try_from(row.users_now).unwrap_or(0));
    summary
}

fn bot_from_row(row: &RoomBotRow) -> Result<Bot, DaoError> {
    let positions = parse_walk_targets(&row.walk_to)?;
    let responses = split_java_list(&row.responses, '|');
    let triggers = split_java_list(&row.triggers, ',');
    let start_position = Position::with_rotation(
        row.start_x,
        row.start_y,
        f64::from(row.start_z),
        row.start_rotation,
    );

    let mut bot = Bot::new(start_position, positions, responses, triggers);
    bot.details_mut().fill_full(
        row.id,
        &row.name,
        &row.motto,
        &row.figure,
        "",
        "",
        0,
        0,
        "Male",
        "",
        "",
        "",
        0,
        "",
        0,
    );
    Ok(bot)
}

fn parse_walk_targets(value: &str) -> Result<Vec<(i32, i32)>, DaoError> {
    if value.is_empty() {
        return Ok(Vec::new());
    }

    value
        .split_whitespace()
        .map(|target| {
            let position = Position::parse(target)
                .map_err(|error| DaoError::new(format!("Invalid bot walk target: {error}")))?;
            Ok((position.x(), position.y()))
        })
        .collect()
}

fn split_java_list(value: &str, delimiter: char) -> Vec<String> {
    if value.contains(delimiter) {
        value.split(delimiter).map(ToOwned::to_owned).collect()
    } else {
        vec![value.to_owned()]
    }
}
