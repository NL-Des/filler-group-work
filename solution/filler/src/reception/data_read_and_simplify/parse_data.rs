use std::io::{self, BufRead};

use crate::board::Board;
use crate::piece::Piece;

use crate::data_input_verification::{verify_board_header, verify_piece_header};

#[derive(Debug, Clone)]
pub struct Player {
    pub me: char,
    pub me_last: char,
    pub enemy: char,
    pub enemy_last: char,
}

pub fn parse_player(line: &str) -> Option<Player> {
    if !line.contains("p1") && !line.contains("p2") {
        return None;
    }

    if line.contains("p1") {
        Some(Player {
            me: '@',
            me_last: 'a',
            enemy: '$',
            enemy_last: 's',
        })
    } else {
        Some(Player {
            me: '$',
            me_last: 's',
            enemy: '@',
            enemy_last: 'a',
        })
    }
}

pub fn read_board<R: BufRead>(reader: &mut R) -> Option<Board> {
    let mut line = String::new();

    loop {
        line.clear();

        if reader.read_line(&mut line).ok()? == 0 {
            return None;
        }

        if line.starts_with("Anfield") {
            break;
        }
    }

    if !verify_board_header(&line) {
        return None;
    }

    let parts: Vec<&str> = line.split_whitespace().collect();

    let width = parts[1].parse::<usize>().ok()?;
    let height = parts[2].trim_end_matches(':').parse::<usize>().ok()?;

    let mut board = Board::new(width, height);

    line.clear();
    reader.read_line(&mut line).ok()?;

    for y in 0..height {
        line.clear();

        if reader.read_line(&mut line).ok()? == 0 {
            return None;
        }

        let chars: Vec<char> = line.trim().chars().collect();

        if chars.len() < width + 4 {
            return None;
        }

        for x in 0..width {
            board.set(x, y, chars[x + 4]);
        }
    }

    Some(board)
}

pub fn read_piece<R: BufRead>(reader: &mut R) -> Option<Piece> {
    let mut line = String::new();

    reader.read_line(&mut line).ok()?;

    if !verify_piece_header(&line) {
        return None;
    }

    let parts: Vec<&str> = line.split_whitespace().collect();

    let width = parts[1].parse::<usize>().ok()?;
    let height = parts[2].trim_end_matches(':').parse::<usize>().ok()?;

    let mut piece = Piece::new(width, height);

    for y in 0..height {
        line.clear();

        if reader.read_line(&mut line).ok()? == 0 {
            return None;
        }

        let chars: Vec<char> = line.trim().chars().collect();

        if chars.len() < width {
            return None;
        }

        for x in 0..width {
            piece.set(x, y, chars[x]);
        }
    }

    Some(piece)
}
