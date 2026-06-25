#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigatorRequest {
    PrivateRooms,
    PopularRooms,
    SearchRooms,
}
