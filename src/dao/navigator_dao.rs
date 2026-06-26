use crate::dao::DaoError;
use crate::game::room::RoomData;

pub trait NavigatorDao {
    fn rooms_by_like_name(&self, name: &str) -> Result<Vec<RoomData>, DaoError>;
}
