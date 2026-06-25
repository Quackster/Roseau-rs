use mysql::prelude::Queryable;
use mysql::{Opts, OptsBuilder, Params, Pool, Row, Value};

use crate::dao::mysql::{
    DatabaseEngine, SqlDriver, SqlExecutionKind, SqlExecutionPlan, SqlExecutionResult,
    SqlParameter, SqlRow, SqlValue, Storage,
};
use crate::dao::DaoError;

#[derive(Debug, Clone)]
pub struct MySqlDriver {
    pool: Pool,
}

impl MySqlDriver {
    pub fn connect(connection_url: &str) -> Result<Self, DaoError> {
        Pool::new(connection_url)
            .map(|pool| Self { pool })
            .map_err(driver_error)
    }

    pub fn connect_storage(storage: &Storage) -> Result<Self, DaoError> {
        if storage.engine() != DatabaseEngine::MySql {
            return Err(DaoError::new(format!(
                "MySQL driver cannot connect {} storage",
                storage.engine().config_prefix()
            )));
        }

        let opts = Opts::from_url(storage.connection_url())
            .map_err(|error| DaoError::new(format!("Invalid MySQL connection URL: {error}")))?;
        let mut builder = OptsBuilder::from_opts(opts);

        if !storage.username().trim().is_empty() {
            builder = builder.user(Some(storage.username()));
        }
        if !storage.password().trim().is_empty() {
            builder = builder.pass(Some(storage.password()));
        }

        Pool::new(builder)
            .map(|pool| Self { pool })
            .map_err(driver_error)
    }

    pub fn pool(&self) -> &Pool {
        &self.pool
    }
}

impl SqlDriver for MySqlDriver {
    fn execute_plan(&self, plan: &SqlExecutionPlan) -> Result<SqlExecutionResult, DaoError> {
        let mut connection = self.pool.get_conn().map_err(driver_error)?;
        let params = positional_params(plan.parameters());
        let result = connection
            .exec_iter(plan.sql(), params)
            .map_err(driver_error)?;

        match plan.kind() {
            SqlExecutionKind::ReadRows => result
                .collect::<Result<Vec<Row>, _>>()
                .map(|rows| SqlExecutionResult::rows(rows.into_iter().map(sql_row_from_mysql_row)))
                .map_err(driver_error),
            SqlExecutionKind::Execute => {
                Ok(SqlExecutionResult::affected_rows(result.affected_rows()))
            }
            SqlExecutionKind::InsertReturningId => {
                let id = result
                    .last_insert_id()
                    .ok_or_else(|| DaoError::new("MySQL insert did not return a generated id"))?;
                let id = i64::try_from(id)
                    .map_err(|_| DaoError::new(format!("MySQL insert id {id} exceeds i64")))?;
                Ok(SqlExecutionResult::insert_id(id))
            }
        }
    }
}

fn positional_params(parameters: &[SqlParameter]) -> Params {
    if parameters.is_empty() {
        Params::Empty
    } else {
        Params::Positional(parameters.iter().map(mysql_value_from_parameter).collect())
    }
}

fn mysql_value_from_parameter(parameter: &SqlParameter) -> Value {
    match parameter {
        SqlParameter::Null => Value::NULL,
        SqlParameter::Bool(value) => Value::Int(i64::from(*value)),
        SqlParameter::Integer(value) => Value::Int(i64::from(*value)),
        SqlParameter::Long(value) => Value::Int(*value),
        SqlParameter::Float(value) => Value::Double(*value),
        SqlParameter::Text(value) => Value::Bytes(value.clone().into_bytes()),
    }
}

fn sql_row_from_mysql_row(row: Row) -> SqlRow {
    let values = row.columns_ref().iter().enumerate().map(|(index, column)| {
        let column_name = column.name_str().into_owned();
        let value = row
            .as_ref(index)
            .map(sql_value_from_mysql_value)
            .unwrap_or(SqlValue::Null);
        (column_name, value)
    });

    SqlRow::new(values)
}

fn sql_value_from_mysql_value(value: &Value) -> SqlValue {
    match value {
        Value::NULL => SqlValue::Null,
        Value::Bytes(bytes) => SqlValue::Text(String::from_utf8_lossy(bytes).into_owned()),
        Value::Int(value) => integer_sql_value(*value),
        Value::UInt(value) => unsigned_sql_value(*value),
        Value::Float(value) => SqlValue::Float(f64::from(*value)),
        Value::Double(value) => SqlValue::Float(*value),
        Value::Date(year, month, day, hour, minute, second, micros) => SqlValue::Text(format!(
            "{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}.{micros:06}"
        )),
        Value::Time(negative, days, hour, minute, second, micros) => {
            let sign = if *negative { "-" } else { "" };
            SqlValue::Text(format!(
                "{sign}{days} {hour:02}:{minute:02}:{second:02}.{micros:06}"
            ))
        }
    }
}

fn integer_sql_value(value: i64) -> SqlValue {
    i32::try_from(value)
        .map(SqlValue::Integer)
        .unwrap_or(SqlValue::Long(value))
}

fn unsigned_sql_value(value: u64) -> SqlValue {
    i32::try_from(value)
        .map(SqlValue::Integer)
        .or_else(|_| i64::try_from(value).map(SqlValue::Long))
        .unwrap_or_else(|_| SqlValue::Text(value.to_string()))
}

fn driver_error(error: mysql::Error) -> DaoError {
    DaoError::new(format!("MySQL driver error: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dao::mysql::DatabaseEngine;

    #[test]
    fn converts_sql_parameters_to_mysql_params() {
        assert_eq!(positional_params(&[]), Params::Empty);
        assert_eq!(
            positional_params(&[
                SqlParameter::Null,
                SqlParameter::Bool(true),
                SqlParameter::Integer(7),
                SqlParameter::Long(8),
                SqlParameter::Float(1.5),
                SqlParameter::Text("alice".to_owned()),
            ]),
            Params::Positional(vec![
                Value::NULL,
                Value::Int(1),
                Value::Int(7),
                Value::Int(8),
                Value::Double(1.5),
                Value::Bytes(b"alice".to_vec()),
            ])
        );
    }

    #[test]
    fn converts_mysql_values_to_driver_neutral_values() {
        assert_eq!(sql_value_from_mysql_value(&Value::NULL), SqlValue::Null);
        assert_eq!(
            sql_value_from_mysql_value(&Value::Bytes(b"alice".to_vec())),
            SqlValue::Text("alice".to_owned())
        );
        assert_eq!(
            sql_value_from_mysql_value(&Value::Int(7)),
            SqlValue::Integer(7)
        );
        assert_eq!(
            sql_value_from_mysql_value(&Value::Int(i64::from(i32::MAX) + 1)),
            SqlValue::Long(i64::from(i32::MAX) + 1)
        );
        assert_eq!(
            sql_value_from_mysql_value(&Value::UInt(u64::MAX)),
            SqlValue::Text(u64::MAX.to_string())
        );
        assert_eq!(
            sql_value_from_mysql_value(&Value::Float(1.25)),
            SqlValue::Float(1.25)
        );
        assert_eq!(
            sql_value_from_mysql_value(&Value::Double(1.5)),
            SqlValue::Float(1.5)
        );
        assert_eq!(
            sql_value_from_mysql_value(&Value::Date(2024, 5, 6, 7, 8, 9, 10)),
            SqlValue::Text("2024-05-06 07:08:09.000010".to_owned())
        );
        assert_eq!(
            sql_value_from_mysql_value(&Value::Time(true, 1, 2, 3, 4, 5)),
            SqlValue::Text("-1 02:03:04.000005".to_owned())
        );
    }

    #[test]
    fn rejects_non_mysql_storage_urls() {
        let storage = Storage::new(
            DatabaseEngine::Postgres,
            "db",
            5432,
            "roseau",
            "secret",
            "hotel",
            "",
            "",
        );

        assert_eq!(
            MySqlDriver::connect_storage(&storage)
                .unwrap_err()
                .message(),
            "MySQL driver cannot connect postgres storage"
        );
    }
}
