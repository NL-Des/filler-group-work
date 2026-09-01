use std::io::{self, BufRead};

/// Lit une seule ligne depuis l'entrée standard.
///
/// Cette fonction peut être utilisée pour récupérer une ligne
/// envoyée par le game engine.
///
/// La fonction retourne la ligne sans le '\n' final
/// et sans les espaces inutiles au début ou à la fin.
pub fn read_input() -> String {
    // Récupération de stdin.
    let stdin = io::stdin();

    // Création d'un lock sur stdin.
    //
    // Le lock permet de lire proprement depuis l'entrée standard.
    let mut reader = stdin.lock();

    // String qui recevra la ligne.
    let mut input = String::new();

    // Lecture d'une ligne.
    //
    // Le game engine doit normalement toujours fournir
    // une entrée valide pendant l'exécution du programme.
    reader.read_line(&mut input).unwrap();

    // Suppression du retour à la ligne et des espaces inutiles.
    input.trim().to_string()
}

/// Vérifie si une ligne contient l'identification d'un joueur.
/// Retourne :
///     true  -> si la ligne contient p1 ou p2
///     false -> sinon
pub fn verify_player_input(line: &str) -> bool {
    line.contains("p1") || line.contains("p2")
}

/// Vérifie le format du header de l'Anfield.
///
/// Après split_whitespace(), on obtient :
///     ["Anfield", "20", "15:"]
///
/// Cette fonction vérifie uniquement que le format est correct.
/// La vérification que les dimensions sont > 0 est faite
/// séparément avec verify_dimensions().
pub fn verify_board_header(line: &str) -> bool {
    // Séparation de la ligne selon les espaces.
    let parts: Vec<&str> = line.split_whitespace().collect();

    // Verification du Header qui doit contenir exactement 3 éléments, Anfiled, Hauteur et Largeur
    if parts.len() != 3 {
        return false;
    }

    // Le premier élément doit être exactement "Anfield".
    if parts[0] != "Anfield" {
        return false;
    }

    // Conversion de la largeur.
    let width = parts[1].parse::<usize>();

    // Conversion de la hauteur et suppression du ":" avant
    let height = parts[2].trim_end_matches(':').parse::<usize>();

    // Le header est valide uniquement si largeur et hauteur
    // sont bien des nombres.
    width.is_ok() && height.is_ok()
}

/// Vérifie le format du header d'une pièce.
///
/// Après split_whitespace() :
///     ["Piece", "4", "1:"]
pub fn verify_piece_header(line: &str) -> bool {
    // Découpage de la ligne.
    let parts: Vec<&str> = line.split_whitespace().collect();

    // Trois éléments sont obligatoires.
    if parts.len() != 3 {
        return false;
    }

    // Le premier élément doit être "Piece".
    if parts[0] != "Piece" {
        return false;
    }

    // Vérification de la largeur.
    let width = parts[1].parse::<usize>();

    // Vérification de la hauteur.
    let height = parts[2].trim_end_matches(':').parse::<usize>();

    // Les deux dimensions doivent être numériques.
    width.is_ok() && height.is_ok()
}

/// Vérifie les dimensions d'une structure.

/// Une largeur ou une hauteur égale à zéro est invalide.
pub fn verify_dimensions(width: usize, height: usize) -> bool {
    // Les deux dimensions doivent être strictement positives.
    width > 0 && height > 0
}

/// Tests du fichier.
///
/// Le fichier test.rs est uniquement compilé avec `cargo test`.
#[cfg(test)]
#[path = "test.rs"]
mod test;
