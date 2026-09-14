use std::env;
use std::fs::OpenOptions;
use std::io::Write;

use crate::board::Board;
use crate::data_read_and_simplify::Player;
use crate::piece::Piece;

const RESET: &str = "\x1b[0m";
const OWN_COLOUR: &str = "\x1b[36m";
const ENEMY_COLOUR: &str = "\x1b[31m";
const PIECE_COLOUR: &str = "\x1b[32m";

/// Indique si le visualiseur terminal est activé.
pub fn is_enabled() -> bool {
    matches!(
        env::var("FILLER_VISUALIZE").as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE")
    )
}

/// Construit l'écran ANSI affiché avant chaque coup du robot.
pub fn render_turn(board: &Board, piece: &Piece, player: &Player, turn: usize) -> String {
    let mut screen = format!(
        "\x1b[2J\x1b[HFiller — visualiseur\nTour {turn}\n\nPlateau ({}x{})\n",
        board.width, board.height
    );

    for y in 0..board.height {
        for x in 0..board.width {
            screen.push_str(&colour_board_cell(board.get(x, y), player));
        }
        screen.push('\n');
    }

    screen.push_str(&format!("\nPièce ({}x{})\n", piece.width, piece.height));
    for y in 0..piece.height {
        for x in 0..piece.width {
            screen.push_str(&colour_piece_cell(piece.get(x, y)));
        }
        screen.push('\n');
    }

    screen.push_str(&format!(
        "\n{OWN_COLOUR}{}{} votre territoire   {ENEMY_COLOUR}{}{} adversaire   {PIECE_COLOUR}O{} pièce\n",
        player.me, RESET, player.enemy, RESET, RESET
    ));
    screen
}

/// Affiche le visualiseur hors des flux capturés par le game engine.
///
/// Le moteur lit stdout pour les coordonnées et capture stderr. Dans un
/// conteneur interactif, `/dev/tty` permet donc d'afficher directement le
/// plateau dans le terminal de l'utilisateur.
pub fn display_turn(board: &Board, piece: &Piece, player: &Player, turn: usize) {
    let screen = render_turn(board, piece, player, turn);

    if let Ok(mut terminal) = OpenOptions::new().write(true).open("/dev/tty") {
        let _ = terminal.write_all(screen.as_bytes());
        let _ = terminal.flush();
    } else {
        eprint!("{screen}");
    }
}

fn colour_board_cell(cell: char, player: &Player) -> String {
    if cell == player.me || cell == player.me_last {
        format!("{OWN_COLOUR}{cell}{RESET}")
    } else if cell == player.enemy || cell == player.enemy_last {
        format!("{ENEMY_COLOUR}{cell}{RESET}")
    } else {
        cell.to_string()
    }
}

fn colour_piece_cell(cell: char) -> String {
    if cell == 'O' || cell == '#' {
        format!("{PIECE_COLOUR}{cell}{RESET}")
    } else {
        cell.to_string()
    }
}

#[cfg(test)]
#[path = "test.rs"]
mod test;
