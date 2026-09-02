/// Imprime la coordonnée du coin supérieur gauche de la pièce à placer,
/// au format attendu par le moteur de jeu : "X Y\n".
pub fn print_placement(x: usize, y: usize) {
    println!("{} {}", x, y);
}

#[cfg(test)]
#[path = "test.rs"]
mod test;
