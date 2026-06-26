#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncomingCommand {
    SetRoomStatus {
        key: String,
        value: String,
        visible: bool,
        timeout: i32,
    },
    RemoveRoomStatus {
        key: String,
    },
    MarkRoomNeedsUpdate,
    ResetAfkTimer,
    SendAlert {
        message: String,
    },
    CloseUserConnections,
    ClosePublicRoomConnections,
    RefreshInventory {
        category: String,
    },
    MarkMessengerMessageRead {
        message_id: i32,
    },
    SendTickets,
    AssignPersonalMessage {
        message: String,
    },
    GoAway,
    RequestBuddy {
        username: String,
    },
    AcceptBuddy {
        username: String,
    },
    DeclineBuddy {
        username: String,
    },
    RemoveBuddy {
        username: String,
    },
    SendMessengerMessage {
        receiver_ids: Vec<i32>,
        message: String,
    },
    WalkTo {
        x: i32,
        y: i32,
    },
    LookTo {
        x: i32,
        y: i32,
    },
    EnterDoor {
        item_id: i32,
    },
    LetUserIn {
        username: String,
    },
    RemoveItem {
        item_id: i32,
    },
    AssignRights {
        username: String,
    },
    RemoveRights {
        username: String,
    },
    KickUser {
        username: String,
    },
    AddWallItem {
        sprite: String,
        wall_position: String,
        extra_data: String,
    },
    ReturnItemToInventory {
        item_id: i32,
    },
    CreateFlat {
        floor: String,
        room_name: String,
        room_model: String,
        state: i32,
        show_owner_name: bool,
    },
    DeleteFlat {
        room_id: i32,
    },
    GetFlatInfo {
        room_id: i32,
    },
    GetOrderInfo {
        call_id: String,
    },
    GetUnitUsers {
        room_name: String,
    },
    GoToFlat,
    InitUnitListener,
    JumpPerformance {
        data: String,
    },
    ClosePoolChangeBooth,
    RetrieveUserInfo,
    GetCredits,
    InitMessenger,
    MoveStuff {
        item_id: i32,
        x: i32,
        y: i32,
        rotation: Option<i32>,
    },
    CryForHelp {
        message: String,
    },
    FindUser {
        username: String,
    },
    ApplyDecoration {
        decoration: String,
        item_id: i32,
    },
    SetItemData {
        item_id: i32,
        data: String,
    },
    SetStuffData {
        item_id: i32,
        data_class: String,
        custom_data: String,
    },
    UseStripItem {
        item_id: i32,
    },
    SplashPosition {
        position: String,
    },
    Login {
        username: String,
        password: String,
        room_login: bool,
    },
    RegisterPlayer {
        name: String,
        password: String,
        email: String,
        mission: String,
        figure: String,
        sex: String,
        birthday: String,
    },
    SearchBusyFlats {
        multiplier: i32,
    },
    EmptySearchBusyFlats,
    SearchFlat {
        query: String,
    },
    SearchFlatForUser {
        username: String,
    },
    TryFlat {
        room_id: i32,
        password: String,
    },
    PlaceWallItemFromInventory {
        item_id: i32,
        wall_position: String,
    },
    PlaceFloorItemFromInventory {
        item_id: i32,
        x: i32,
        y: i32,
        rotation: i32,
    },
    Purchase {
        call_id: String,
    },
    SetFlatInfo {
        room_id: i32,
        description: String,
        password: String,
        all_super_user: bool,
    },
    Talk {
        mode: String,
        message: String,
    },
    UpdatePoolFigure {
        pool_figure: String,
    },
    UpdateProfile {
        password: String,
        email: String,
        figure: String,
        mission: String,
        sex: String,
    },
    UpdateFlat {
        room_id: i32,
        room_name: String,
        state: i32,
        show_owner_name: bool,
    },
}
