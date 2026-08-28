#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_player_input() {
        assert!(verify_player_input("p1"));
        assert!(verify_player_input("p2"));
    }

    #[test]
    fn invalid_player_input() {
        assert!(!verify_player_input("p3"));
        assert!(!verify_player_input(""));
    }

    #[test]
    fn valid_board_header() {
        assert!(verify_board_header("Anfield 10 20:"));
    }

    #[test]
    fn invalid_board_header() {
        assert!(!verify_board_header("Something 10 20:"));
        assert!(!verify_board_header("Anfield"));
        assert!(!verify_board_header("Anfield abc 20:"));
    }

    #[test]
    fn valid_piece_header() {
        assert!(verify_piece_header("Piece 3 2:"));
    }

    #[test]
    fn invalid_piece_header() {
        assert!(!verify_piece_header("Something 3 2:"));
        assert!(!verify_piece_header("Piece abc 2:"));
    }

    #[test]
    fn valid_dimensions() {
        assert!(verify_dimensions(5, 5));
    }

    #[test]
    fn invalid_dimensions() {
        assert!(!verify_dimensions(0, 5));
        assert!(!verify_dimensions(5, 0));
    }
}
