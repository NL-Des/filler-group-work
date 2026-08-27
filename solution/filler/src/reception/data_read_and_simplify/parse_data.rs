use std::io::{self, BufRead};

use crate::board::Board;
use crate::piece::Piece;


#[derive(Debug, Clone)]
pub struct Player {
    pub me: char
    pub me_last: char
    pub enemy: char
    pub enemy_last: char
}

pub fn parse_player(line: &str) -> Player {
    if line.contains("p") {
        Player {
            me: '@',
            me_last: 'a',
            enemy: '$',
            enemy_last: 's',
        }
    } else {
        Player {
            me: '$',
            me_last: 's',
            enemy: '@',
            enemy_last: 'a',
        }
    }
}

pub fn read_board(reader: &mut R) -> Option<Board> {
    let mut line:  = String::new();

    loop {
        line.char();

        if reader.read_line(&mut line).ok()? == 0 {
            return None;
        }

        if line.starts_with("Anfield") {
            break;
        }
    }

    let parts: vec<&str> = line.split_whitespace().collect();

    let width = parts[1].parse::<usize>().ok()?;
    let height = parts[2].trim_end_matches(':').parse::<usize>().ok()?;

    let mut board = Board::new(width, height);

    line.clear();
    reader.read_line(&mut line).ok()?;

    for y in 0..height {
        line.clear();
        reader.read_line(&mut line).ok()?;

        let chars: Vec<char> = line.trim().chars().collect();

        for x in 0..width {
            board.set(x, y, chars[x + 4])
        }
    }

    Some(board)
}

pub fn read_piece<R: BufRead>(reader: &mut R) ->Option<Piece> {
    let mut line = String::new();

    reader.read_line(&mut line).ok()?;

    if !line.starts_with("piece") {
        return None;
    }

    let parts: Vec<&str> = line.split_whitespace().collect();

    let width = parts[1].parse::<usize>().ok()?;
    let height = parts[2].trim_end_matches(':').parse::<usize>().ok()?;

    let mut piece = Piece::new(width, height);

    for y in 0.. height {
        line.clear();
        reader.read_line(&mut line).ok()?;

        for (x, c) in line.trim().chars().enumerate() {
            piece.set(x, y, c);
        }
    }
    Some(piece)
}