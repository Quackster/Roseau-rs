use std::collections::HashMap;

use crate::messages::incoming::*;
use crate::protocol::ClientMessage;

pub struct MessageHandler {
    messages: HashMap<String, Box<dyn IncomingEvent>>,
}

impl MessageHandler {
    pub fn new() -> Self {
        let mut handler = Self {
            messages: HashMap::new(),
        };
        handler.register();
        handler
    }

    pub fn register(&mut self) {
        self.messages.clear();
        self.register_user_packets();
        self.register_handshake_packets();
        self.register_login_packets();
        self.register_register_packets();
        self.register_navigator_packets();
        self.register_room_packets();
        self.register_messenger_packets();
        self.register_item_packets();
        self.register_pool_handlers();
        self.register_moderation();
    }

    pub fn handle_request(
        &self,
        mut context: IncomingContext,
        message: &dyn ClientMessage,
    ) -> IncomingContext {
        if let Some(handler) = self.messages.get(message.get_header()) {
            handler.handle(&mut context, message);
        }

        context
    }

    pub fn contains_header(&self, header: &str) -> bool {
        self.messages.contains_key(header)
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn headers(&self) -> impl Iterator<Item = &str> {
        self.messages.keys().map(String::as_str)
    }

    fn register_message(&mut self, header: &str, event: impl IncomingEvent + 'static) {
        self.messages.insert(header.to_owned(), Box::new(event));
    }

    fn register_user_packets(&mut self) {
        self.register_message("UPDATE", Update);
    }

    fn register_handshake_packets(&mut self) {
        self.register_message("VERSIONCHECK", VersionCheck);
    }

    fn register_login_packets(&mut self) {
        self.register_message("LOGIN", Login);
        self.register_message("INFORETRIEVE", InfoRetrieve);
        self.register_message("GETCREDITS", GetCredits);
    }

    fn register_register_packets(&mut self) {
        self.register_message("APPROVENAME", ApproveName);
        self.register_message("FINDUSER", FindUser);
        self.register_message("REGISTER", Register);
    }

    fn register_navigator_packets(&mut self) {
        self.register_message("INITUNITLISTENER", InitUnitListener);
        self.register_message("GETUNITUSERS", GetUnitUsers);
        self.register_message("SEARCHFLATFORUSER", SearchFlatForUser);
        self.register_message("SEARCHBUSYFLATS", SearchBusyFlats);
        self.register_message("SETFLATINFO", SetFlatInfo);
        self.register_message("GETFLATINFO", GetFlatInfo);
        self.register_message("UPDATEFLAT", UpdateFlat);
        self.register_message("DELETEFLAT", DeleteFlat);
        self.register_message("TRYFLAT", TryFlat);
        self.register_message("GOTOFLAT", GoToFlat);
        self.register_message("CLOSE_UIMAKOPPI", CloseUimakoppi);
        self.register_message("LOOKTO", LookTo);
        self.register_message("SEARCHFLAT", SearchFlat);
    }

    fn register_room_packets(&mut self) {
        self.register_message("STATUSOK", StatusOk);
        self.register_message("Move", Move);
        self.register_message("Dance", Dance);
        self.register_message("STOP", Stop);
        self.register_message("CHAT", Talk);
        self.register_message("SHOUT", Talk);
        self.register_message("WHISPER", Talk);
        self.register_message("GOAWAY", GoAway);
        self.register_message("CREATEFLAT", CreateFlat);
        self.register_message("ASSIGNRIGHTS", AssignRights);
        self.register_message("REMOVERIGHTS", RemoveRights);
        self.register_message("KILLUSER", KillUser);
    }

    fn register_messenger_packets(&mut self) {
        self.register_message("MESSENGERINIT", MessengerInit);
        self.register_message("MESSENGER_SENDMSG", MessengerSendMessage);
        self.register_message("MESSENGER_REQUESTBUDDY", MessengerRequestBuddy);
        self.register_message("MESSENGER_ACCEPTBUDDY", MessengerAcceptBuddy);
        self.register_message("MESSENGER_DECLINEBUDDY", MessengerDeclineBuddy);
        self.register_message("MESSENGER_REMOVEBUDDY", MessengerRemoveBuddy);
        self.register_message("MESSENGER_MARKREAD", MessengerMarkRead);
        self.register_message("MESSENGER_ASSIGNPERSMSG", MessengerAssignPersonalMessage);
    }

    fn register_item_packets(&mut self) {
        self.register_message("GETORDERINFO", GetOrderInfo);
        self.register_message("GETSTRIP", GetStrip);
        self.register_message("PURCHASE", Purchase);
        self.register_message("PLACESTUFFFROMSTRIP", PlaceStuffFromStrip);
        self.register_message("PLACEITEMFROMSTRIP", PlaceItemFromStrip);
        self.register_message("MOVESTUFF", MoveStuff);
        self.register_message("FLATPROPERTYBYITEM", FlatPropertyByItem);
        self.register_message("ADDSTRIPITEM", AddStripItem);
        self.register_message("REMOVEITEM", RemoveItem);
        self.register_message("SETSTUFFDATA", SetStuffData);
        self.register_message("CarryItem", CarryItem);
        self.register_message("CarryDrink", CarryDrink);
        self.register_message("IntoDoor", IntoDoor);
        self.register_message("REMOVESTUFF", RemoveStuff);
        self.register_message("ADDITEM", AddItem);
        self.register_message("SETSTRIPITEMDATA", SetStripItemData);
        self.register_message("LETUSERIN", LetUserIn);
        self.register_message("SETITEMDATA", SetItemData);
    }

    fn register_pool_handlers(&mut self) {
        self.register_message("JUMPPERF", JumpPerformance);
        self.register_message("SPLASH_POSITION", SplashPosition);
        self.register_message("GIVE_TICKETS", GiveTickets);
        self.register_message("Sign", Sign);
    }

    fn register_moderation(&mut self) {
        self.register_message("CRYFORHELP", CryForHelp);
    }
}

impl Default for MessageHandler {
    fn default() -> Self {
        Self::new()
    }
}
