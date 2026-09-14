/// Imprime la coordonnée du coin supérieur gauche de la pièce à placer,
/// au format attendu par le moteur de jeu : "X Y\n".
pub fn print_placement(x: usize, y: usize) {
    print!("{}", format_placement(x, y));
}

fn format_placement(x: usize, y: usize) -> String {
    format!("{x} {y}\n")
}

#[cfg(test)]
#[path = "test.rs"]
mod test;
