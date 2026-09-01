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

use crate::{board::Board, data_read_and_simplify::Player, piece::Piece};

pub fn read_data_game(player: Player, board: Board, piece: Piece) {
    let table_board = board.cells();

    // reçoit les dimensions du plateau de jeu (Anfield).
    // reçoit les dimensions de la pièce à placer.
// faire un tableau en deux dimensions pour le plateau de jeu (Anfield).
// faire un tableau en deux dimensionspour la pièce à placer.
// faire une boucle qui lit les lignes du plateau de jeu et les stocke dans le tableau du plateau.
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