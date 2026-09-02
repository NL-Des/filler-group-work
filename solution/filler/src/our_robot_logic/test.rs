use super::rules::{apply_rules_to_place_piece, if_placement_valid};

const ME: char = '@';
const ME_LAST: char = 'a';
const ENEMY: char = '$';
const ENEMY_LAST: char = 's';

fn small_board() -> Vec<Vec<char>> {
    vec![
        vec!['.', '.', '.', '.'],
        vec!['.', '@', '.', '.'],
        vec!['.', '.', '.', '$'],
        vec!['.', '.', '.', '.'],
    ]
}

fn dot_piece() -> Vec<Vec<char>> {
    vec![vec!['O']]
}

#[test]
fn valid_when_exactly_one_ally_overlap_and_no_enemy() {
    let board = small_board();
    let piece = dot_piece();

    // La pièce (1x1) posée exactement sur la case alliée (1,1).
    assert!(if_placement_valid(
        &board, &piece, (1, 1), ME, ME_LAST, ENEMY, ENEMY_LAST
    ));
}

#[test]
fn invalid_when_no_overlap() {
    let board = small_board();
    let piece = dot_piece();

    // La pièce posée sur une case vide.
    assert!(!if_placement_valid(
        &board, &piece, (0, 0), ME, ME_LAST, ENEMY, ENEMY_LAST
    ));
}

#[test]
fn invalid_when_overlap_on_enemy() {
    let board = small_board();
    let piece = dot_piece();

    // La pièce posée sur la case ennemie (3,2).
    assert!(!if_placement_valid(
        &board, &piece, (3, 2), ME, ME_LAST, ENEMY, ENEMY_LAST
    ));
}

#[test]
fn invalid_when_out_of_bounds() {
    let board = small_board();
    let piece = vec![vec!['O', 'O'], vec!['O', 'O']];

    // Un placement 2x2 dont le coin en bas à droite (4,4) dépasse le plateau 4x4 (indices 0..3).
    assert!(!if_placement_valid(
        &board, &piece, (3, 3), ME, ME_LAST, ENEMY, ENEMY_LAST
    ));
}

#[test]
fn invalid_when_two_ally_overlaps() {
    let board = vec![
        vec!['@', '@'],
        vec!['.', '.'],
    ];
    let piece = vec![vec!['O', 'O']];

    // La pièce recouvre les deux cases alliées à la fois.
    assert!(!if_placement_valid(
        &board, &piece, (0, 0), ME, ME_LAST, ENEMY, ENEMY_LAST
    ));
}

#[test]
fn apply_rules_finds_all_valid_placements_around_ally() {
    let board = small_board();
    let piece = dot_piece();

    let mut placements =
        apply_rules_to_place_piece(&board, &piece, ME, ME_LAST, ENEMY, ENEMY_LAST);
    placements.sort();

    // Avec une pièce 1x1, seule la case alliée (1,1) permet un recouvrement valide.
    assert_eq!(placements, vec![(1, 1)]);
}

#[test]
fn apply_rules_returns_empty_when_no_ally_cell_present() {
    let board = vec![
        vec!['.', '.'],
        vec!['.', '$'],
    ];
    let piece = dot_piece();

    let placements =
        apply_rules_to_place_piece(&board, &piece, ME, ME_LAST, ENEMY, ENEMY_LAST);

    assert!(placements.is_empty());
}
