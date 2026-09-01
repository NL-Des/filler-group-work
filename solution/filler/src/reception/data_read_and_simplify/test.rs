#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::super::{parse_player, read_board, read_piece};

    // ============================================================
    // Tests de parse_player()
    // ============================================================

    #[test]
    fn test_parse_player_p1() {
        let player = parse_player("$$$ exec p1 : [robots/bender]");

        assert!(player.is_some());

        let player = player.unwrap();

        assert_eq!(player.me, '@');
        assert_eq!(player.me_last, 'a');
        assert_eq!(player.enemy, '$');
        assert_eq!(player.enemy_last, 's');
    }

    #[test]
    fn test_parse_player_p2() {
        let player = parse_player("$$$ exec p2 : [robots/bender]");

        assert!(player.is_some());

        let player = player.unwrap();

        assert_eq!(player.me, '$');
        assert_eq!(player.me_last, 's');
        assert_eq!(player.enemy, '@');
        assert_eq!(player.enemy_last, 'a');
    }

    #[test]
    fn test_parse_player_invalid() {
        assert!(parse_player("$$$ exec p3 : [robots/bender]").is_none());
        assert!(parse_player("hello").is_none());
        assert!(parse_player("").is_none());
    }

    // ============================================================
    // Tests de read_board()
    // ============================================================

    #[test]
    fn test_read_board_valid() {
        let input = "\
Anfield 5 4:
    01234
000 .....
001 ..@..
002 .....
003 .....
";

        let mut reader = Cursor::new(input.as_bytes());

        let board = read_board(&mut reader);

        assert!(board.is_some());

        let board = board.unwrap();

        // Vérification des dimensions.
        assert_eq!(board.width, 5);
        assert_eq!(board.height, 4);

        // Vérification des cellules.
        assert_eq!(board.get(0, 0), '.');
        assert_eq!(board.get(1, 0), '.');
        assert_eq!(board.get(2, 1), '@');
        assert_eq!(board.get(4, 3), '.');
    }

    #[test]
    fn test_read_board_with_player_positions() {
        let input = "\
Anfield 6 4:
    012345
000 ......
001 ..@...
002 ...$..
003 ......
";

        let mut reader = Cursor::new(input.as_bytes());

        let board = read_board(&mut reader);

        assert!(board.is_some());

        let board = board.unwrap();

        assert_eq!(board.get(2, 1), '@');
        assert_eq!(board.get(3, 2), '$');
    }

    #[test]
    fn test_read_board_invalid_header() {
        let input = "\
Wrong 5 4:
    01234
000 .....
001 .....
002 .....
003 .....
";

        let mut reader = Cursor::new(input.as_bytes());

        assert!(read_board(&mut reader).is_none());
    }

    #[test]
    fn test_read_board_missing_rows() {
        let input = "\
Anfield 5 4:
    01234
000 .....
001 .....
";

        let mut reader = Cursor::new(input.as_bytes());

        assert!(read_board(&mut reader).is_none());
    }

    #[test]
    fn test_read_board_invalid_dimensions() {
        let input = "\
Anfield 0 4:
    0123
000 ....
001 ....
002 ....
003 ....
";

        let mut reader = Cursor::new(input.as_bytes());

        assert!(read_board(&mut reader).is_none());
    }

    // ============================================================
    // Tests de read_piece()
    // ============================================================

    #[test]
    fn test_read_piece_valid() {
        let input = "\
Piece 4 2:
.##.
.##.
";

        let mut reader = Cursor::new(input.as_bytes());

        let piece = read_piece(&mut reader);

        assert!(piece.is_some());

        let piece = piece.unwrap();

        // Vérification des dimensions.
        assert_eq!(piece.width, 4);
        assert_eq!(piece.height, 2);

        // Vérification du contenu.
        assert_eq!(piece.get(0, 0), '.');
        assert_eq!(piece.get(1, 0), '#');
        assert_eq!(piece.get(2, 0), '#');
        assert_eq!(piece.get(3, 0), '.');

        assert_eq!(piece.get(0, 1), '.');
        assert_eq!(piece.get(1, 1), '#');
        assert_eq!(piece.get(2, 1), '#');
        assert_eq!(piece.get(3, 1), '.');
    }

    #[test]
    fn test_read_piece_different_shape() {
        let input = "\
Piece 5 4:
.##..
.##..
..#..
...#.
";

        let mut reader = Cursor::new(input.as_bytes());

        let piece = read_piece(&mut reader);

        assert!(piece.is_some());

        let piece = piece.unwrap();

        assert_eq!(piece.width, 5);
        assert_eq!(piece.height, 4);

        assert_eq!(piece.get(1, 0), '#');
        assert_eq!(piece.get(2, 0), '#');

        assert_eq!(piece.get(1, 1), '#');
        assert_eq!(piece.get(2, 1), '#');

        assert_eq!(piece.get(2, 2), '#');
        assert_eq!(piece.get(3, 3), '#');
    }

    #[test]
    fn test_read_piece_invalid_header() {
        let input = "\
Wrong 4 2:
.##.
.##.
";

        let mut reader = Cursor::new(input.as_bytes());

        assert!(read_piece(&mut reader).is_none());
    }

    #[test]
    fn test_read_piece_missing_rows() {
        let input = "\
Piece 4 2:
.##.
";

        let mut reader = Cursor::new(input.as_bytes());

        assert!(read_piece(&mut reader).is_none());
    }

    #[test]
    fn test_read_piece_invalid_dimensions() {
        let input = "\
Piece 0 2:
..
..
";

        let mut reader = Cursor::new(input.as_bytes());

        assert!(read_piece(&mut reader).is_none());
    }
}
