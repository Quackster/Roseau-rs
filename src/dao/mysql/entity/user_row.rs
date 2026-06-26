use crate::dao::{mysql::SqlRow, DaoError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserRow {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub rank: i32,
    pub mission: String,
    pub figure: String,
    pub pool_figure: String,
    pub email: String,
    pub credits: i32,
    pub sex: String,
    pub country: String,
    pub badge: String,
    pub birthday: String,
    pub join_date: i64,
    pub last_online: i64,
    pub personal_greeting: String,
    pub tickets: i32,
}

impl UserRow {
    pub const TABLE: &'static str = "users";

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i32,
        username: impl Into<String>,
        password: impl Into<String>,
        rank: i32,
        mission: impl Into<String>,
        figure: impl Into<String>,
        pool_figure: impl Into<String>,
        email: impl Into<String>,
        credits: i32,
        sex: impl Into<String>,
        country: impl Into<String>,
        badge: impl Into<String>,
        birthday: impl Into<String>,
        join_date: i64,
        last_online: i64,
        personal_greeting: impl Into<String>,
        tickets: i32,
    ) -> Self {
        Self {
            id,
            username: username.into(),
            password: password.into(),
            rank,
            mission: mission.into(),
            figure: figure.into(),
            pool_figure: pool_figure.into(),
            email: email.into(),
            credits,
            sex: sex.into(),
            country: country.into(),
            badge: badge.into(),
            birthday: birthday.into(),
            join_date,
            last_online,
            personal_greeting: personal_greeting.into(),
            tickets,
        }
    }
}

impl TryFrom<&SqlRow> for UserRow {
    type Error = DaoError;

    fn try_from(row: &SqlRow) -> Result<Self, Self::Error> {
        Ok(Self::new(
            row.required_i32("id")?,
            row.required_string("username")?,
            row.optional_string("password")?.unwrap_or_default(),
            row.required_i32("rank")?,
            row.required_string("mission")?,
            row.required_string("figure")?,
            row.required_string("pool_figure")?,
            row.optional_string("email")?.unwrap_or_default(),
            row.required_i32("credits")?,
            row.required_string("sex")?,
            row.required_string("country")?,
            row.required_string("badge")?,
            row.required_string("birthday")?,
            row.optional_i64("join_date")?.unwrap_or_default(),
            row.optional_i64("last_online")?.unwrap_or_default(),
            row.required_string("personal_greeting")?,
            row.required_i32("tickets")?,
        ))
    }
}

#[cfg(test)]
#[path = "user_row_tests.rs"]
mod tests;
