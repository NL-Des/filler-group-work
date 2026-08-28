use std::io::Cursor;

use crate::data_read_and_simplify::{parse_player, read_board, read_piece};

#[test]
fn test_parse_player_p1() {
    let player = parse_player("p1").expect("p1 should be valid");

    assert_eq!(player.me, '@');
    assert_eq!(player.me_last, 'a');
    assert_eq!(player.enemy, '$');
    assert_eq!(player.enemy_last, 's');
}

#[test]
fn test_parse_player_p2() {
    let player = parse_player("p2").expect("p2 should be valid");

    assert_eq!(player.me, '$');
    assert_eq!(player.me_last, 's');
    assert_eq!(player.enemy, '@');
    assert_eq!(player.enemy_last, 'a');
}

#[test]
fn test_parse_player_invalid() {
    let player = parse_player("invalid");

    assert!(player.is_none());
}

#[test]
fn test_read_board() {
    let input = "\
something before
Anfield 5 3:
    01234
0000.....
0001..@..
0002.$...
";

    let mut reader = Cursor::new(input);

    let board = read_board(&mut reader).expect("The board should be successfully read");

    assert_eq!(board.width, 5);
    assert_eq!(board.height, 3);

    assert_eq!(board.get(0, 0), '.');
    assert_eq!(board.get(1, 0), '.');
    assert_eq!(board.get(2, 0), '.');
    assert_eq!(board.get(3, 0), '.');
    assert_eq!(board.get(4, 0), '.');

    assert_eq!(board.get(2, 1), '@');
    assert_eq!(board.get(3, 2), '$');
}

#[test]
fn test_read_board_no_header() {
    let input = "\
something
another line
";

    let mut reader = Cursor::new(input);

    let board = read_board(&mut reader);

    assert!(board.is_none());
}

#[test]
fn test_read_board_invalid_dimensions() {
    let input = "\
Anfield abc 3:
    01234
0000.....
0001.....
0002.....
";

    let mut reader = Cursor::new(input);

    let board = read_board(&mut reader);

    assert!(board.is_none());
}

#[test]
fn test_read_piece() {
    let input = "\
Piece 3 2:
***
.*.
";

    let mut reader = Cursor::new(input);

    let piece = read_piece(&mut reader).expect("The piece should be successfully read");

    assert_eq!(piece.width, 3);
    assert_eq!(piece.height, 2);

    assert_eq!(piece.get(0, 0), '*');
    assert_eq!(piece.get(1, 0), '*');
    assert_eq!(piece.get(2, 0), '*');

    assert_eq!(piece.get(0, 1), '.');
    assert_eq!(piece.get(1, 1), '*');
    assert_eq!(piece.get(2, 1), '.');
}

#[test]
fn test_read_piece_invalid_header() {
    let input = "\
Something 3 2:
***
.*.
";

    let mut reader = Cursor::new(input);

    let piece = read_piece(&mut reader);

    assert!(piece.is_none());
}

#[test]
fn test_read_piece_invalid_dimensions() {
    let input = "\
Piece abc 2:
***
.*.
";

    let mut reader = Cursor::new(input);

    let piece = read_piece(&mut reader);

    assert!(piece.is_none());
}

#[test]
fn test_read_piece_empty_input() {
    let input = "";

    let mut reader = Cursor::new(input);

    let piece = read_piece(&mut reader);

    assert!(piece.is_none());
}
