use crate::game::messenger::MessengerEffect;
use crate::messages::outgoing::SystemBroadcast;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerEffect {
    SendAlert(SystemBroadcast),
    UpdateLastLogin { user_id: i32 },
    CloseConnection { connection_id: i32 },
    CloseUserConnections { user_id: i32 },
    DisposeOwnedRooms { user_id: i32 },
    DisposeInventory { user_id: i32 },
    LeaveCurrentRoom { connection_id: i32 },
    Messenger(MessengerEffect),
}
