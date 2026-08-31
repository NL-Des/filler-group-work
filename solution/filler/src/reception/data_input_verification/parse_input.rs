use std::io::{self, BufRead};

#[cfg(test)]
#[path = "test.rs"]
mod test;

pub fn read_input() -> String {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.lock().read_line(&mut input).unwrap();

    input.trim().to_string()
}

pub fn verify_player_input(line: &str) -> bool {
    line.contains("p1") || line.contains("p2")
}

pub fn verify_board_header(line: &str) -> (usize, usize) {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() != 3 {
        return (0, 0);
    }

    if parts[0] != "Anfield" {
        return (0, 0);
    }

    let width = parts[1].parse::<usize>();
    let height = parts[2].trim_end_matches(':').parse::<usize>();

    (width.unwrap(), height.unwrap())

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
