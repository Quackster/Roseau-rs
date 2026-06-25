use crate::messages::OutgoingMessage;
use crate::protocol::NettyResponse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenGameBoard {
    game: String,
    game_id: String,
    item_id: i32,
    sprite_id: i32,
    x: i32,
    y: i32,
}

impl OpenGameBoard {
    pub fn new(
        game: impl Into<String>,
        game_id: impl Into<String>,
        item_id: i32,
        sprite_id: i32,
        x: i32,
        y: i32,
    ) -> Self {
        Self {
            game: game.into(),
            game_id: game_id.into(),
            item_id,
            sprite_id,
            x,
            y,
        }
    }
}

impl OutgoingMessage for OpenGameBoard {
    fn write(&self, response: &mut NettyResponse) {
        response.init("OPEN_GAMEBOARD");
        response.append_new_argument(&self.game_id);
        response.append_argument_with(&self.game, ';');
        response.append_argument_with(" ", ';');
        response.append_tab_argument(self.item_id);
        response.append_argument(self.sprite_id);
        response.append_argument(self.x);
        response.append_argument(self.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_open_game_board_packet() {
        let mut response = OpenGameBoard::new("chess", "game-1", 10, 20, 3, 4).compose();

        assert_eq!(
            response.get(),
            "#OPEN_GAMEBOARD\rgame-1;chess; \t10 20 3 4##"
        );
    }
}
