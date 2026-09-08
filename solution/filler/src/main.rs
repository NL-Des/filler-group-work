use std::io;
use std::io::BufRead;

use crate::data_input_verification::verify_player_input;
use crate::data_read_and_simplify::parse_player;
use crate::data_read_and_simplify::read_board;
use crate::data_read_and_simplify::read_piece;

mod board;
mod piece;

#[path = "reception/data_input_verification/parse_input.rs"]
mod data_input_verification;

#[path = "reception/data_read_and_simplify/parse_data.rs"]
mod data_read_and_simplify;

mod actions_to_place_pieces;
mod our_robot_logic;
mod send_answer;
mod visualisation;

fn main() {
    // creation d'un reader pour le stdin
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut player_line = String::new();

    if reader.read_line(&mut player_line).unwrap() == 0 {
        return;
    }

    // Vérification de la validité des données d'entré pour le joueur
    if !verify_player_input(&player_line) {
        println!("Invalid player input: {}", player_line);
        return;
    }

    // Transformation de "p1" / "p2" en Player.
    let player = match parse_player(&player_line) {
        Some(player) => player,
        None => return,
    };

    //boucle de jeu
    // Lecture du plateau, validation et placement des piece sur le board
    loop {
        let board = match read_board(&mut reader) {
            Some(board) => board,
            None => break,
        };

        // Lecture de la pièce et validation.
        let piece = match read_piece(&mut reader) {
            Some(piece) => piece,
            None => break,
        };

        // Lecture des données et mise en 2D du tableau de jeu et de la pièce.
        let (board_2d, piece_2d) = our_robot_logic::rules::read_data_game(board, piece);

        // Lancement de la stratégie du robot pour déterminer le meilleur placement de la pièce.
        let result = our_robot_logic::strategy::read_game(board_2d, piece_2d, player.clone());

        // reponse au game engine si placement trouvé ou non
        match result {
            Some((x, y)) => send_answer::print_placement(x, y),
            // OLD -- None => println!("No valid placement found."),
            // le sujet attend un une sortie au format (x, y)
            None => send_answer::print_placement(0, 0),
        }
    }
}
