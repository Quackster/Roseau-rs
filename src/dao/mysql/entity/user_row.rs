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
            row.required_string("password")?,
            row.required_i32("rank")?,
            row.required_string("mission")?,
            row.required_string("figure")?,
            row.required_string("pool_figure")?,
            row.required_string("email")?,
            row.required_i32("credits")?,
            row.required_string("sex")?,
            row.required_string("country")?,
            row.required_string("badge")?,
            row.required_string("birthday")?,
            row.required_i64("join_date")?,
            row.required_i64("last_online")?,
            row.required_string("personal_greeting")?,
            row.required_i32("tickets")?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::SqlValue;

    #[test]
    fn builds_user_row_from_sql_row() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(7)),
            ("username", SqlValue::Text("alice".to_owned())),
            ("password", SqlValue::Text("hash".to_owned())),
            ("rank", SqlValue::Integer(4)),
            ("mission", SqlValue::Text("hello".to_owned())),
            ("figure", SqlValue::Text("hd-100".to_owned())),
            ("pool_figure", SqlValue::Text("pool".to_owned())),
            ("email", SqlValue::Text("alice@example.test".to_owned())),
            ("credits", SqlValue::Integer(55)),
            ("sex", SqlValue::Text("F".to_owned())),
            ("country", SqlValue::Text("UK".to_owned())),
            ("badge", SqlValue::Text("ADM".to_owned())),
            ("birthday", SqlValue::Text("1990-01-01".to_owned())),
            ("join_date", SqlValue::Long(1000)),
            ("last_online", SqlValue::Long(2000)),
            ("personal_greeting", SqlValue::Text("welcome".to_owned())),
            ("tickets", SqlValue::Integer(8)),
        ]);

        assert_eq!(
            UserRow::try_from(&row).unwrap(),
            UserRow::new(
                7,
                "alice",
                "hash",
                4,
                "hello",
                "hd-100",
                "pool",
                "alice@example.test",
                55,
                "F",
                "UK",
                "ADM",
                "1990-01-01",
                1000,
                2000,
                "welcome",
                8,
            )
        );
    }

    #[test]
    fn reports_invalid_user_columns() {
        let row = SqlRow::new([
            ("id", SqlValue::Integer(7)),
            ("username", SqlValue::Text("alice".to_owned())),
        ]);

        assert_eq!(
            UserRow::try_from(&row).unwrap_err().message(),
            "Missing or invalid SQL column `password` as String"
        );
    }
}
