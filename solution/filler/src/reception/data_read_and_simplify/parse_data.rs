use std::io::BufRead;

use crate::board::Board;
use crate::piece::Piece;

use crate::data_input_verification::{verify_board_header, verify_dimensions, verify_piece_header};

/// Représente le joueur contrôlé par notre programme.
///
/// Les caractères utilisés par Filler changent selon le joueur.
///
/// Pour le joueur 1 :
///
///     notre territoire       = '@'
///     notre dernière pièce   = 'a'
///     territoire adverse     = '$'
///     dernière pièce adverse = 's'
///
/// Pour le joueur 2 :
///
///     notre territoire       = '$'
///     notre dernière pièce   = 's'
///     territoire adverse     = '@'
///     dernière pièce adverse = 'a'
#[derive(Debug, Clone)]
pub struct Player {
    /// Caractère représentant notre territoire.
    pub me: char,

    /// Caractère représentant notre dernière pièce.
    pub me_last: char,

    /// Caractère représentant le territoire adverse.
    pub enemy: char,

    /// Caractère représentant la dernière pièce adverse.
    pub enemy_last: char,
}

/// Transforme la ligne envoyée par le game engine
/// en structure Player.
///
/// Retour :
///
///     Some(Player) -> joueur identifié
///     None         -> ligne invalide
pub fn parse_player(line: &str) -> Option<Player> {
    // On vérifie d'abord si la ligne correspond au joueur 1.
    //
    // On teste p1 avant p2 afin de déterminer clairement
    // les caractères à utiliser.
    if line.contains("p1") {
        return Some(Player {
            // Notre territoire permanent.
            me: '@',

            // Notre dernière pièce.
            me_last: 'a',

            // Territoire adverse.
            enemy: '$',

            // Dernière pièce adverse.
            enemy_last: 's',
        });
    }

    // Si ce n'est pas p1, on regarde si c'est p2.
    if line.contains("p2") {
        return Some(Player {
            // Notre territoire permanent.
            me: '$',

            // Notre dernière pièce.
            me_last: 's',

            // Territoire adverse.
            enemy: '@',

            // Dernière pièce adverse.
            enemy_last: 'a',
        });
    }

    // La ligne ne contient ni p1 ni p2.
    None
}

/// Lit un Anfield complet depuis le flux d'entrée.
///
/// Le game engine envoie quelque chose comme :
///
///     Anfield 20 15:
///         01234567890123456789
///     000 ....................
///     001 ....................
///     002 .........@..........
///     ...
///
/// Cette fonction :
///
/// 1. cherche "Anfield";
/// 2. vérifie son header;
/// 3. récupère largeur et hauteur;
/// 4. crée un Board;
/// 5. saute la ligne des indices;
/// 6. lit toutes les lignes de la grille;
/// 7. remplit le Board.
///
/// Retour :
///
///     Some(Board) -> Anfield correctement lu
///     None        -> entrée invalide
pub fn read_board<R: BufRead>(reader: &mut R) -> Option<Board> {
    // Buffer utilisé pour chaque ligne lue.
    let mut line = String::new();

    // Verification de la presence du Header "Anfield"
    // le game engine peut envoyer d'autre data vant le header, alors on continue jusqu'a ce que l'on lise "Anfiled"
    loop {
        // On réutilise la même String.
        line.clear();

        // Lecture d'une ligne.
        //
        // read_line() retourne 0 lorsqu'il n'y a plus rien
        // à lire dans stdin.
        if reader.read_line(&mut line).ok()? == 0 {
            return None;
        }

        // Le header de l'Anfield commence par "Anfield".
        if line.starts_with("Anfield") {
            break;
        }
    }

    // ---------------------------------------------------------
    // Vérification du header
    // ---------------------------------------------------------
    if !verify_board_header(&line) {
        return None;
    }

    // ---------------------------------------------------------
    // Extraction des dimensions
    // ---------------------------------------------------------
    //
    // sortie :
    //
    //     ["Anfield", "20", "15:"]
    let parts: Vec<&str> = line.split_whitespace().collect();

    // Récupération de la largeur.
    let width = parts[1].parse::<usize>().ok()?;

    // Récupération de la hauteur.
    //
    // "15:" devient "15".
    let height = parts[2].trim_end_matches(':').parse::<usize>().ok()?;

    // Vérification que les dimensions sont valides.
    if !verify_dimensions(width, height) {
        return None;
    }

    // ---------------------------------------------------------
    // Création du Board
    // ---------------------------------------------------------
    //
    // width  = nombre de colonnes
    // height = nombre de lignes
    let mut board = Board::new(width, height);

    // ---------------------------------------------------------
    // Lecture de la ligne des indices
    // ---------------------------------------------------------
    //
    // Exemple : 01234567890123456789

    // Cette ligne sert uniquement à afficher les coordonnées.
    // Elle ne fait pas partie de l'Anfield.
    line.clear();

    if reader.read_line(&mut line).ok()? == 0 {
        return None;
    }

    // ---------------------------------------------------------
    // Lecture des lignes de l'Anfield
    // ---------------------------------------------------------
    for y in 0..height {
        line.clear();

        // Lecture d'une ligne de terrain.
        if reader.read_line(&mut line).ok()? == 0 {
            return None;
        }

        // Suppression du retour à la ligne.
        let chars: Vec<char> = line.trim().chars().collect();

        // Une ligne doit contenir :
        //
        // 3 caractères de numéro
        // + 1 espace
        // + width caractères de terrain
        //
        // Donc au minimum width + 4 caractères.
        if chars.len() < width + 4 {
            return None;
        }

        // Copie des caractères de terrain dans le Board.
        //
        // chars[0..3] = numéro de ligne
        // chars[3]    = espace
        // chars[4..]  = terrain
        for x in 0..width {
            board.set(x, y, chars[x + 4]);
        }
    }

    // Anfield correctement rempli.
    Some(board)
}

/// Lit une Piece complète depuis le flux d'entrée.
/// Retour :
///
///     Some(Piece) -> pièce correctement lue
///     None        -> entrée invalide
pub fn read_piece<R: BufRead>(reader: &mut R) -> Option<Piece> {
    // Buffer utilisé pour lire les différentes lignes.
    let mut line = String::new();

    // ---------------------------------------------------------
    // Lecture du header
    // ---------------------------------------------------------
    //./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator
    // Exemple :
    //
    //     Piece 4 1:
    if reader.read_line(&mut line).ok()? == 0 {
        return None;
    }

    // Vérification du header.
    if !verify_piece_header(&line) {
        return None;
    }

    // ---------------------------------------------------------
    // Extraction des dimensions
    // ---------------------------------------------------------
    //
    // Exemple :
    //
    //     ["Piece", "4", "1:"]
    let parts: Vec<&str> = line.split_whitespace().collect();

    // Largeur de la pièce.
    let width = parts[1].parse::<usize>().ok()?;

    // Hauteur de la pièce.
    let height = parts[2].trim_end_matches(':').parse::<usize>().ok()?;

    // Vérification des dimensions.
    if !verify_dimensions(width, height) {
        return None;
    }

    // ---------------------------------------------------------
    // Création de la pièce
    // ---------------------------------------------------------
    let mut piece = Piece::new(width, height);

    // ---------------------------------------------------------
    // Lecture de chaque ligne de la pièce
    // ---------------------------------------------------------
    for y in 0..height {
        line.clear();

        // Lecture de la ligne.
        if reader.read_line(&mut line).ok()? == 0 {
            return None;
        }

        // Suppression du '\n' et des espaces autour.
        let chars: Vec<char> = line.trim().chars().collect();

        // La ligne doit contenir au minimum `width`
        // caractères.
        if chars.len() < width {
            return None;
        }

        // Copie des caractères dans la Piece.
        //
        // Les caractères intéressants sont généralement :
        //
        //     'O' -> cellule occupée par la pièce
        //     '.' -> cellule vide
        //
        // Le choix exact de la cellule active sera géré
        // par l'algorithme de placement.
        for x in 0..width {
            piece.set(x, y, chars[x]);
        }
    }

    // Piece correctement remplie.
    Some(piece)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
#[path = "test.rs"]
mod test;
