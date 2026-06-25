use crate::dao::mysql::entity::{UserPermissionRow, UserRow};
use crate::dao::mysql::{SqlParameter, SqlQuery};
use crate::dao::CreatePlayer;
use crate::game::player::PlayerDetails;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerQueries;

impl PlayerQueries {
    pub fn create_player(
        player: &CreatePlayer,
        password_hash: &str,
        default_credits: i32,
        messenger_greeting: &str,
        now: i64,
    ) -> SqlQuery {
        SqlQuery::new(
            "INSERT INTO users (username, password, email, mission, figure, credits, sex, birthday, join_date, last_online, personal_greeting) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            [
                SqlParameter::Text(player.username.clone()),
                SqlParameter::Text(password_hash.to_owned()),
                SqlParameter::Text(player.email.clone()),
                SqlParameter::Text(player.mission.clone()),
                SqlParameter::Text(player.figure.clone()),
                SqlParameter::Integer(default_credits),
                SqlParameter::Text(player.sex.clone()),
                SqlParameter::Text(player.birthday.clone()),
                SqlParameter::Long(now),
                SqlParameter::Long(now),
                SqlParameter::Text(messenger_greeting.to_owned()),
            ],
        )
    }

    pub fn details_by_id(user_id: i32) -> SqlQuery {
        SqlQuery::new(
            "SELECT * FROM users WHERE id = ? LIMIT 1",
            [SqlParameter::Integer(user_id)],
        )
    }

    pub fn details_by_username(username: &str) -> SqlQuery {
        SqlQuery::new(
            "SELECT * FROM users WHERE username = ? LIMIT 1",
            [SqlParameter::Text(username.to_owned())],
        )
    }

    pub fn login(username: &str) -> SqlQuery {
        Self::details_by_username(username)
    }

    pub fn id_by_username(username: &str) -> SqlQuery {
        SqlQuery::new(
            "SELECT id FROM users WHERE username = ? LIMIT 1",
            [SqlParameter::Text(username.to_owned())],
        )
    }

    pub fn is_name_taken(name: &str) -> SqlQuery {
        SqlQuery::new(
            "SELECT id FROM users WHERE username = ? LIMIT 1",
            [SqlParameter::Text(name.to_owned())],
        )
    }

    pub fn update_player(details: &PlayerDetails, password_hash: &str) -> SqlQuery {
        SqlQuery::new(
            "UPDATE users SET password = ?, figure = ?, credits = ?, mission = ?, pool_figure = ?, sex = ?, email = ?, personal_greeting = ?, tickets = ? WHERE id = ?",
            [
                SqlParameter::Text(password_hash.to_owned()),
                SqlParameter::Text(details.figure().to_owned()),
                SqlParameter::Integer(details.credits()),
                SqlParameter::Text(details.mission().to_owned()),
                SqlParameter::Text(details.pool_figure().to_owned()),
                SqlParameter::Text(details.sex().to_owned()),
                SqlParameter::Text(details.email().to_owned()),
                SqlParameter::Text(details.personal_greeting().to_owned()),
                SqlParameter::Integer(details.tickets()),
                SqlParameter::Integer(details.id()),
            ],
        )
    }

    pub fn update_credits(user_id: i32, credits: i32) -> SqlQuery {
        SqlQuery::new(
            "UPDATE users SET credits = ? WHERE id = ?",
            [
                SqlParameter::Integer(credits),
                SqlParameter::Integer(user_id),
            ],
        )
    }

    pub fn update_tickets(user_id: i32, tickets: i32) -> SqlQuery {
        SqlQuery::new(
            "UPDATE users SET tickets = ? WHERE id = ?",
            [
                SqlParameter::Integer(tickets),
                SqlParameter::Integer(user_id),
            ],
        )
    }

    pub fn update_personal_greeting(
        user_id: i32,
        personal_greeting: impl Into<String>,
    ) -> SqlQuery {
        SqlQuery::new(
            "UPDATE users SET personal_greeting = ? WHERE id = ?",
            [
                SqlParameter::Text(personal_greeting.into()),
                SqlParameter::Integer(user_id),
            ],
        )
    }

    pub fn update_pool_figure(user_id: i32, pool_figure: impl Into<String>) -> SqlQuery {
        SqlQuery::new(
            "UPDATE users SET pool_figure = ? WHERE id = ?",
            [
                SqlParameter::Text(pool_figure.into()),
                SqlParameter::Integer(user_id),
            ],
        )
    }

    pub fn update_last_login(user_id: i32, now: i64) -> SqlQuery {
        SqlQuery::new(
            "UPDATE users SET last_online = ? WHERE id = ?",
            [SqlParameter::Long(now), SqlParameter::Integer(user_id)],
        )
    }

    pub fn permissions() -> SqlQuery {
        SqlQuery::select_all(UserPermissionRow::TABLE)
    }

    pub fn user_table() -> &'static str {
        UserRow::TABLE
    }

    pub fn permission_table() -> &'static str {
        UserPermissionRow::TABLE
    }
}
