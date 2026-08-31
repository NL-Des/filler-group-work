use std::io;

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

mod our_robot_logic;
mod send_answer;
mod actions_to_place_pieces;
mod visualisation;

fn main() {
    // Importation de la ligne joueur.
    let input = data_input_verification::read_input();

    // Vérification de la validité des données d'entrée.
    if !verify_player_input(&input) {
        println!("Invalid player input: {}", input);
        return;
    }

    // Mise en forme des données d'entrée.
    let player = parse_player(&input);
    if player.is_none() {
        println!("Invalid player input: {}", input);
        return;
    }
    let player = player.unwrap();

    // On continue de lire le même flux stdin là où read_input() s'est arrêté,
    // pour ne pas perdre les lignes du plateau et de la pièce.
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let board = read_board(&mut reader);
    if board.is_none() {
        println!("Invalid board input: {}", input);
        return;
    }
    let board = board.unwrap();

    let piece = read_piece(&mut reader);
    if piece.is_none() {
        println!("Invalid piece input: {}", input);
        return;
    }
    let piece = piece.unwrap();

    // TODO: brancher our_robot_logic / actions_to_place_pieces / send_answer.
    let _ = (player, board, piece);
}
