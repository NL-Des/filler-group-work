// Données en entrée :
// - Pièce (taille de la pièces(Hauteur / Largeur) et le dessin de la pièce)
// - Dimensions + Grille du plateau (Anfield)
// - Numéro de joueur (au 1er tour) : p1 (@ / a) ou p2 ($ / s)

// Règles de placement pour une coordonnée (X, Y) testée :
// 1. Territoire allié : EXACTEMENT 1 case active (#) doit recouvrir une case alliée (@/a ou $/s).
// 2. Territoire adverse : 0 case active (#) ne doit recouvrir l'adversaire.
// 3. Bordures : Toutes les cases actives (#) doivent être dans la grille.
//    (Les cases vides '.' de la pièce peuvent dépasser du plateau ou recouvrir n'importe quoi).

// Données de sortie :
// - Coordonnées de la pièce à placer sur le plateau de jeu. (coordonnées X Y\n du coin supérieur gauche de la boîte de la pièce.)

use crate::{board::Board, data_read_and_simplify::Player, piece::{self, Piece}};

pub fn read_data_game(board: Board, piece: Piece) {
    // reçoit les dimensions du plateau de jeu (Anfield).
    // reçoit les dimensions de la pièce à placer.
    let table_height = board.height;
    let table_width = board.width;
    let table_board = board.cells;
    let piece_height = piece.height;
    let piece_width = piece.width;
    let piece_cells = piece.cells;

    // Un tableau en deux dimensions pour le plateau de jeu (Anfield).
    // Un tableau en deux dimensionspour la pièce à placer.
    let mut board_2d: Vec<Vec<char>> = vec![vec!['.'; table_width]; table_height];
    let mut piece_2d: Vec<Vec<char>> = vec![vec!['.'; piece_width]; piece_height];

    // Une boucle qui lit les lignes du plateau de jeu et les stocke dans le tableau du plateau.
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

    // faire une boucle qui lit chaque case et détecte si :
    // Elle est vide.
    // Si elle est occupée par le joueur 1.
    // Si elle est occupée par le joueur 2.
    // Renvoi les deux tableaux.
}

pub fn apply_rules_to_place_piece() {
    // reçoit la réponse du robot.
    // reçoit les tableaux du plateau et de la pièce.
    // Vérification si la coordonnée de placement est valide selon les règles de placement.
    // Si la coordonnée est valide, renvoyer les coordonnées.
}

pub fn if_placement_valid() {
    // reçoit les coordonnées de placement.
    // reçoit le joueur (p1 ou p2).
    // Transformme les pièces du joueurs en @ ou $
    // Envoi de la réponse.
}
