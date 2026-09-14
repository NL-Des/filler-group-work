#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::data_read_and_simplify::Player;
    use crate::piece::Piece;

    use super::super::render_turn;

    #[test]
    fn render_turn_displays_board_piece_and_player_colours() {
        let mut board = Board::new(2, 2);
        board.set(0, 0, '@');
        board.set(1, 1, '$');

        let mut piece = Piece::new(2, 1);
        piece.set(0, 0, 'O');

        let player = Player {
            me: '@',
            me_last: 'a',
            enemy: '$',
            enemy_last: 's',
        };

        let screen = render_turn(&board, &piece, &player, 3);

        assert!(screen.starts_with("\x1b[2J\x1b[H"));
        assert!(screen.contains("Tour 3"));
        assert!(screen.contains("Plateau (2x2)"));
        assert!(screen.contains("Pièce (2x1)"));
        assert!(screen.contains("\x1b[36m@\x1b[0m"));
        assert!(screen.contains("\x1b[31m$\x1b[0m"));
        assert!(screen.contains("\x1b[32mO\x1b[0m"));
    }
}
