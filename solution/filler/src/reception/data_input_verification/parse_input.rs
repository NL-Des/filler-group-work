use std::str::FromStr;

pub fn verify_player_input(line: &str) -> bool {
    line.contains("p1") || line.contains("p2")
}

pub fn verify_board_header(line: &str) -> bool {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() != 3 {
        return false;
    }

    if parts[0] != "Anfield" {
        return false;
    }

    let width = parts[1].parse::<usize>();
    let height = parts[2].trim_end_matches(':').parse::<usize>();

    width.is_ok() && height.is_ok()
}

pub fn verify_piece_header(line: &str) -> bool {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() != 3 {
        return false;
    }

    if parts[0] != "Piece" {
        return false;
    }

    let width = parts[1].parse::<usize>();
    let height = parts[2].trim_end_matches(':').parse::<usize>();

    width.is_ok() && height.is_ok()
}

pub fn verify_dimensions(width: usize, height: usize) -> bool {
    width > 0 && height > 0
}
