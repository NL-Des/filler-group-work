use crate::{board::Board, piece::Piece};

// Lecture et mise en tableau en 2D des données. Pour le robot et faire les vérifications des règles de placement.
pub fn read_data_game(board: Board, piece: Piece) -> (Vec<Vec<char>>, Vec<Vec<char>>) {

    // Reçoit les dimensions du plateau de jeu (Anfield).
    let table_height = board.height;
    let table_width = board.width;
    let table_board = board.cells;

    // Reçoit les dimensions de la pièce à placer.
    let piece_height = piece.height;
    let piece_width = piece.width;
    let piece_cells = piece.cells;

    // Un tableau en deux dimensions pour le plateau de jeu (Anfield).
    // Un tableau en deux dimensions pour la pièce à placer.
    let mut board_2d: Vec<Vec<char>> = vec![vec!['.'; table_width]; table_height];
    let mut piece_2d: Vec<Vec<char>> = vec![vec!['.'; piece_width]; piece_height];

    // Une boucle qui lit les lignes du plateau de jeu et les stockes dans le tableau du plateau.
    for i in 0..table_height {
        for j in 0..table_width {
            board_2d[i][j] = table_board[i * table_width + j];
        }
    }

    // Une boucle qui lit les lignes de la pièce et les stocke dans le tableau de la pièce.
    for i in 0..piece_height {
        for j in 0..piece_width {
            piece_2d[i][j] = piece_cells[i * piece_width + j];
        }
    }

    (board_2d, piece_2d)
}

// Vérifie si la proposition de placement du robot respecte les règles.
pub fn if_placement_valid(board_2d: &[Vec<char>],piece_2d: &[Vec<char>],top_left: (usize, usize),me: char,me_last: char,enemy: char,enemy_last: char,) -> bool {

    let board_height = board_2d.len(); // Hauteur du plateau.
    let board_width = if board_height == 0 { // Largeur du plateau.
            0 
        } else { 
            board_2d[0].len() };
        // Le if avec 0 est pour éviter un panic si le plateau est vide.

    // Coordonnée du coin supérieur gauche de la pièce sur le plateau.
    let (top_left_x, top_left_y) = top_left; 

    // Compteurs de cases alliées ou ennemies qui sont recouvertes par la suggestion de placement.
    let mut ally_overlaps = 0;
    let mut enemy_overlaps = 0;

    for (py, row) in piece_2d.iter().enumerate() { // py = position y de la pièce.
        for (px, &cell) in row.iter().enumerate() { // px = position x de la pièce.
            if cell != 'O' {
                continue;
            }

            // Vérification que la pièce ne dépasse pas du plateau.
            let board_x = top_left_x + px;
            let board_y = top_left_y + py;
            if board_x >= board_width || board_y >= board_height {
                return false;
            }

            // COmptabilisation des cases alliées et ennemies recouvertes par la proposition de placement.
            let board_cell = board_2d[board_y][board_x];
            if board_cell == me || board_cell == me_last {
                ally_overlaps += 1;
            } else if board_cell == enemy || board_cell == enemy_last {
                enemy_overlaps += 1;
            }
        }
    }

    // Si la toutes les règles sont respectées, alors la proposition est validée.
    // Notamment ne recouvrir qu'une seule case alliée et aucune case ennemie.
    ally_overlaps == 1 && enemy_overlaps == 0
}

// Applique les règles de "if_placement_valid" pour déterminer les placements valides d'une pièce.
pub fn apply_rules_to_place_piece(board_2d: &[Vec<char>],piece_2d: &[Vec<char>],me: char,me_last: char,enemy: char,enemy_last: char) -> Vec<(usize, usize)> {

    let board_height = board_2d.len(); // Hauteur du plateau.
    let board_width = if board_height == 0 { // Largeur du plateau.
            0 
        } else { 
            board_2d[0].len() };
        // Le if avec 0 est pour éviter un panic si le plateau est vide.

    // Stocke les placements valides de la pièce sur le plateau.    
    let mut valid_placements = Vec::new();

    for y in 0..board_height {
        for x in 0..board_width {
            if if_placement_valid(board_2d, piece_2d, (x, y), me, me_last, enemy, enemy_last) {
                valid_placements.push((x, y));
            }
        }
    }

    valid_placements
}
