#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigatorRequest {
    PrivateRooms,
    PopularRooms,
    SearchRooms,
}

#[cfg(test)]
#[path = "navigator_request_tests.rs"]
mod tests;
