#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigatorRequest {
    PrivateRooms,
    PopularRooms,
    SearchRooms,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_java_navigator_request_variants() {
        let requests = [
            NavigatorRequest::PrivateRooms,
            NavigatorRequest::PopularRooms,
            NavigatorRequest::SearchRooms,
        ];

        assert_eq!(requests.len(), 3);
    }
}
