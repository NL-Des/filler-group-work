use std::collections::{HashSet, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::data_read_and_simplify::Player;
use crate::our_robot_logic::rules;

const NEIGHBORS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

pub fn read_game(
    board_2d: Vec<Vec<char>>,
    piece_2d: Vec<Vec<char>>,
    Player { me, me_last, enemy, enemy_last }: Player,
) -> Option<(usize, usize)> {
    let candidates = rules::apply_rules_to_place_piece(&board_2d, &piece_2d, me, me_last, enemy, enemy_last);
    if candidates.is_empty() {
        return None;
    }

    let footprint: Vec<(usize, usize)> = piece_2d
        .iter()
        .enumerate()
        .flat_map(|(py, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &cell)| cell == 'O')
                .map(move |(px, _)| (px, py))
        })
        .collect();

    let dist_from_me = flood_fill_distances(&board_2d, |c| c == me || c == me_last);
    let dist_from_enemy = flood_fill_distances(&board_2d, |c| c == enemy || c == enemy_last);

    let (w_expansion, w_aggression, w_race, w_fragmentation) = phase_weights(&board_2d);

    let mut best_score = f64::MIN;
    let mut best_candidates: Vec<(usize, usize)> = Vec::new();

    for &(x, y) in &candidates {
        let score = score_placement(
            &board_2d,
            &footprint,
            (x, y),
            enemy,
            enemy_last,
            &dist_from_me,
            &dist_from_enemy,
            w_expansion,
            w_aggression,
            w_race,
            w_fragmentation,
        );

        if score > best_score {
            best_score = score;
            best_candidates.clear();
            best_candidates.push((x, y));
        } else if score == best_score {
            best_candidates.push((x, y));
        }
    }

    let pick = pseudo_random_index(best_candidates.len());
    best_candidates.get(pick).copied()
}

/// Détermine les poids des critères selon le taux de remplissage du plateau :
/// en début de partie on privilégie l'expansion, en fin de partie l'agression
/// et la course territoriale.
fn phase_weights(board_2d: &[Vec<char>]) -> (f64, f64, f64, f64) {
    let total_cells: usize = board_2d.iter().map(|row| row.len()).sum();
    let filled_cells = board_2d
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c != '.')
        .count();

    let filled_ratio = if total_cells == 0 {
        0.0
    } else {
        filled_cells as f64 / total_cells as f64
    };

    if filled_ratio < 0.4 {
        (3.0, 1.0, 1.0, 1.0) // early game : expansion
    } else {
        (1.0, 3.0, 2.0, 1.0) // late game : agression / course
    }
}

#[allow(clippy::too_many_arguments)]
fn score_placement(
    board_2d: &[Vec<char>],
    footprint: &[(usize, usize)],
    top_left: (usize, usize),
    enemy: char,
    enemy_last: char,
    dist_from_me: &[Vec<i32>],
    dist_from_enemy: &[Vec<i32>],
    w_expansion: f64,
    w_aggression: f64,
    w_race: f64,
    w_fragmentation: f64,
) -> f64 {
    let (top_left_x, top_left_y) = top_left;
    let board_cells: Vec<(usize, usize)> = footprint
        .iter()
        .map(|&(dx, dy)| (top_left_x + dx, top_left_y + dy))
        .collect();
    let occupied: HashSet<(usize, usize)> = board_cells.iter().copied().collect();

    let mut adjacent_empty: HashSet<(usize, usize)> = HashSet::new();
    let mut adjacent_enemy = 0;
    let mut race_wins = 0;

    for &(x, y) in &board_cells {
        for &(dx, dy) in &NEIGHBORS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if ny >= board_2d.len() || nx >= board_2d[0].len() {
                continue;
            }
            if occupied.contains(&(nx, ny)) {
                continue;
            }

            let cell = board_2d[ny][nx];
            if cell == '.' {
                adjacent_empty.insert((nx, ny));
            } else if cell == enemy || cell == enemy_last {
                adjacent_enemy += 1;
            }
        }

        if board_2d[y][x] == '.' && dist_from_me[y][x] <= dist_from_enemy[y][x] {
            race_wins += 1;
        }
    }

    let reachable_open_space = local_open_space(board_2d, &occupied, &adjacent_empty, footprint.len() * 4);
    let fragmentation_threshold = footprint.len() * 2;
    let fragmentation_penalty = if reachable_open_space < fragmentation_threshold {
        (fragmentation_threshold - reachable_open_space) as f64
    } else {
        0.0
    };

    w_expansion * adjacent_empty.len() as f64
        + w_aggression * adjacent_enemy as f64
        + w_race * race_wins as f64
        - w_fragmentation * fragmentation_penalty
}

/// BFS borné qui compte l'espace vide accessible autour d'un placement, pour
/// détecter les coups qui s'enfermeraient dans une poche fermée.
fn local_open_space(
    board_2d: &[Vec<char>],
    occupied: &HashSet<(usize, usize)>,
    starting_points: &HashSet<(usize, usize)>,
    cap: usize,
) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    for &point in starting_points {
        if visited.insert(point) {
            queue.push_back(point);
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        if visited.len() >= cap {
            break;
        }

        for &(dx, dy) in &NEIGHBORS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if ny >= board_2d.len() || nx >= board_2d[0].len() {
                continue;
            }
            if occupied.contains(&(nx, ny)) || board_2d[ny][nx] != '.' {
                continue;
            }
            if visited.insert((nx, ny)) {
                queue.push_back((nx, ny));
            }
        }
    }

    visited.len()
}

/// Calcule, pour chaque case du plateau, la distance (en nombre de sauts)
/// jusqu'à la case source multi-origine la plus proche satisfaisant `is_source`.
fn flood_fill_distances(board_2d: &[Vec<char>], is_source: impl Fn(char) -> bool) -> Vec<Vec<i32>> {
    let height = board_2d.len();
    let width = if height == 0 { 0 } else { board_2d[0].len() };

    let mut distances = vec![vec![i32::MAX; width]; height];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    for (y, row) in board_2d.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if is_source(cell) {
                distances[y][x] = 0;
                queue.push_back((x, y));
            }
        }
    }

    while let Some((x, y)) = queue.pop_front() {
        let current_distance = distances[y][x];

        for &(dx, dy) in &NEIGHBORS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if ny >= height || nx >= width {
                continue;
            }
            if distances[ny][nx] > current_distance + 1 {
                distances[ny][nx] = current_distance + 1;
                queue.push_back((nx, ny));
            }
        }
    }

    distances
}

/// Petit générateur pseudo-aléatoire (xorshift) pour départager les coups
/// à égalité de score, sans dépendance externe.
fn pseudo_random_index(len: usize) -> usize {
    if len <= 1 {
        return 0;
    }

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
        | 1;

    let mut state = seed;
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;

    (state % len as u64) as usize
}
