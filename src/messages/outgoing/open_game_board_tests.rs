use super::open_game_board::*;

#[test]
fn composes_open_game_board_packet() {
    let mut response = OpenGameBoard::new("chess", "game-1", 10, 20, 3, 4).compose();

    assert_eq!(
        response.get(),
        "#OPEN_GAMEBOARD\rgame-1;chess; \t10 20 3 4##"
    );
}
