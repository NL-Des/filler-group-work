use crate::data_input_verification::verify_player_input;
use crate::data_input_verification::verify_board_header;
use crate::data_input_verification::verify_piece_header;
use crate::data_input_verification::verify_dimensions;

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
    // Importation des données d'entrée.
    let input = data_input_verification::read_input();

    // Vérification de la validité des données d'entrée.
    if !verify_player_input(&input) {
        println!("Invalid player input: {}", input);
        return;
    }
    let (width, height) = verify_board_header(&input);

    if !verify_piece_header(&input) {
        println!("Invalid piece header: {}", input);
        return;
    }
    if !verify_dimensions(width, height) {
        println!("Invalid board dimensions: {}x{}", width, height);
        return;
    }

    // Mise en forme des données d'entrée.
    
}
