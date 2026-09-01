#[cfg(test)]
mod tests {
    use super::super::{
        verify_board_header,
        verify_dimensions,
        verify_piece_header,
        verify_player_input,
    };

    // ============================================================
    // Tests de verify_player_input()
    // ============================================================

    #[test]
    fn test_verify_player_input_p1() {
        assert!(verify_player_input(
            "$$$ exec p1 : [robots/bender]"
        ));
    }

    #[test]
    fn test_verify_player_input_p2() {
        assert!(verify_player_input(
            "$$$ exec p2 : [robots/bender]"
        ));
    }

    #[test]
    fn test_verify_player_input_invalid() {
        assert!(!verify_player_input(
            "$$$ exec p3 : [robots/bender]"
        ));

        assert!(!verify_player_input("hello"));
        assert!(!verify_player_input(""));
    }

    // ============================================================
    // Tests de verify_board_header()
    // ============================================================

    #[test]
    fn test_verify_board_header_valid() {
        assert!(verify_board_header("Anfield 20 15:"));
        assert!(verify_board_header("Anfield 5 5:"));
        assert!(verify_board_header("Anfield 1 1:"));
    }

    #[test]
    fn test_verify_board_header_invalid_name() {
        assert!(!verify_board_header("Board 20 15:"));
        assert!(!verify_board_header("anfield 20 15:"));
    }

    #[test]
    fn test_verify_board_header_invalid_dimensions() {
        assert!(!verify_board_header("Anfield abc 15:"));
        assert!(!verify_board_header("Anfield 20 abc:"));
    }

    #[test]
    fn test_verify_board_header_invalid_number_of_elements() {
        assert!(!verify_board_header("Anfield 20:"));
        assert!(!verify_board_header("Anfield 20 15: extra"));
        assert!(!verify_board_header(""));
    }

    // ============================================================
    // Tests de verify_piece_header()
    // ============================================================

    #[test]
    fn test_verify_piece_header_valid() {
        assert!(verify_piece_header("Piece 4 1:"));
        assert!(verify_piece_header("Piece 2 2:"));
        assert!(verify_piece_header("Piece 10 5:"));
    }

    #[test]
    fn test_verify_piece_header_invalid_name() {
        assert!(!verify_piece_header("Anfield 4 1:"));
        assert!(!verify_piece_header("piece 4 1:"));
    }

    #[test]
    fn test_verify_piece_header_invalid_dimensions() {
        assert!(!verify_piece_header("Piece abc 1:"));
        assert!(!verify_piece_header("Piece 4 abc:"));
    }

    #[test]
    fn test_verify_piece_header_invalid_number_of_elements() {
        assert!(!verify_piece_header("Piece 4:"));
        assert!(!verify_piece_header("Piece 4 1: extra"));
        assert!(!verify_piece_header(""));
    }

    // ============================================================
    // Tests de verify_dimensions()
    // ============================================================

    #[test]
    fn test_verify_dimensions_valid() {
        assert!(verify_dimensions(1, 1));
        assert!(verify_dimensions(20, 15));
        assert!(verify_dimensions(100, 100));
    }

    #[test]
    fn test_verify_dimensions_zero_width() {
        assert!(!verify_dimensions(0, 10));
    }

    #[test]
    fn test_verify_dimensions_zero_height() {
        assert!(!verify_dimensions(10, 0));
    }

    #[test]
    fn test_verify_dimensions_both_zero() {
        assert!(!verify_dimensions(0, 0));
    }
}
