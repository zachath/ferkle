// Input: the face values (1-6) of 1 to 6 selected dice.
// Output: Farkle score corresponding to input.
// Every selected die must be part of a scoring combination, otherwise the selection is invalid.
pub fn selection(faces: &[u32]) -> Result<u32, String> {
    // Reject selections of 0 or 7+ dice.
    if faces.is_empty() || faces.len() >= 7 {
        return Err("You must select between 1 and 6 dice".to_string());
    }

    // Reject any face outside 1-6. `|&n|` destructures the `&u32` the iterator yields into a plain `u32`.
    if faces.iter().any(|&n| !(1..=6).contains(&n)) {
        return Err("Dice values must be between 1 and 6".to_string());
    }

    // counts[face] = number of dice showing that face (index 0 unused).
    let mut counts = [0u32; 7];
    for &n in faces {
        counts[n as usize] += 1; // Array indices must be `usize`, so cast the `u32` face.
    }

    // Convert `Option` to `Result`: `Some(score)` becomes `Ok(score)`, `None` becomes this error.
    best_score(counts).ok_or_else(|| "Not all selected dice score".to_string())
}

// Highest score using every die in `counts`, or None if some die can't score.
// Works recursively: remove one scoring combo, then score whatever dice remain.
fn best_score(counts: [u32; 7]) -> Option<u32> {
    // Base case: no dice left means every die was used, which is valid and adds 0 points.
    if counts.iter().all(|&c| c == 0) {
        return Some(0);
    }

    // Highest total found so far. Stays `None` if no combo uses up all the dice.
    let mut best: Option<u32> = None;

    // Closure that takes the dice left after removing a combo and the points that combo is worth.
    // If the leftover dice can all score, update `best` with the combo's points plus the leftover score.
    // It is `mut` because it modifies `best`, which it captures from the enclosing function.
    let mut try_combo = |remaining: [u32; 7], points: u32| {
        // `if let` only runs the block when the recursive call returns `Some`.
        if let Some(rest) = best_score(remaining) {
            // `Option::max` treats `None` as smaller than any `Some`, so the first valid total always wins over `None`.
            best = best.max(Some(points + rest));
        }
    };

    // Straights: (lowest face, highest face, points).
    for (low, high, points) in [(1, 6, 1500), (1, 5, 500), (2, 6, 750)] {
        if (low..=high).all(|f| counts[f] > 0) { // Is every face in the straight present at least once?
            let mut remaining = counts; // Copy counts (arrays of `Copy` types are copied, not moved).
            (low..=high).for_each(|f| remaining[f] -= 1); // Remove one die of each face in the straight.
            try_combo(remaining, points); // Score the straight plus whatever dice remain.
        }
    }

    // Three or more of a kind; each die beyond three doubles the value.
    for face in 1..=6 {
        let base = if face == 1 { 1000 } else { face as u32 * 100 }; // Value of three of this face: 1 = 1000, 2 = 200, 3 = 300, etc...
        // Try every group size from 3 up to all dice of this face. An empty range (fewer than 3) runs zero times.
        for n in 3..=counts[face] {
            let mut remaining = counts; // Copy counts.
            remaining[face] -= n; // Remove the n dice in this group.
            try_combo(remaining, base << (n - 3)); // `<<` shifts left, which doubles once per die beyond three.
        }
    }

    // Single 1s and 5s.
    for (face, points) in [(1, 100), (5, 50)] { // Iterate over an array of (face, points) tuples.
        if counts[face] > 0 { // Is face present in count?
            let mut remaining = counts; // Copy counts.
            remaining[face] -= 1; // Remove one die of this face.
            try_combo(remaining, points); // Score the single die plus whatever dice remain.
        }
    }

    best // Highest valid total, or `None` if no combo uses up all the dice.
}

// `#[cfg(test)]` compiles this module only when running `cargo test`.
#[cfg(test)]
mod tests {
    use super::selection; // Bring `selection` from the parent module into scope.

    #[test] // Marks the function as a test that `cargo test` runs.
    fn singles() {
        assert_eq!(selection(&[1]), Ok(100));
        assert_eq!(selection(&[5]), Ok(50));
        assert_eq!(selection(&[1, 5]), Ok(150));
    }

    #[test]
    fn straights() {
        assert_eq!(selection(&[1, 2, 3, 4, 5]), Ok(500)); // 1 -> 5
        assert_eq!(selection(&[2, 3, 4, 5, 6]), Ok(750)); // 2 -> 6
        assert_eq!(selection(&[6, 5, 4, 3, 2, 1]), Ok(1500)); // 1 -> 6
        assert_eq!(selection(&[1, 2, 3, 4, 5, 5]), Ok(550)); // 1 -> 5 + scoring 5
    }

    #[test]
    fn of_a_kind() {
        assert_eq!(selection(&[1, 1, 1]), Ok(1000));
        assert_eq!(selection(&[2, 2, 2]), Ok(200));
        assert_eq!(selection(&[6, 6, 6]), Ok(600));
        assert_eq!(selection(&[2, 2, 2, 2]), Ok(400)); // One extra die doubles 200.
        assert_eq!(selection(&[2, 2, 2, 2, 2]), Ok(800));
        assert_eq!(selection(&[2, 2, 2, 2, 2, 2]), Ok(1600));
        assert_eq!(selection(&[1, 1, 1, 1]), Ok(2000)); // Beats 1000 + 100 from three 1s plus a single 1.
        assert_eq!(selection(&[3, 3, 3, 1, 5]), Ok(450)); // 300 + 100 + 50
    }

    #[test]
    fn invalid() {
        assert!(selection(&[]).is_err()); // Empty input.
        assert!(selection(&[1, 1, 1, 1, 1, 1, 1]).is_err()); // Input too long.
        assert!(selection(&[0]).is_err()); // Invalid die.
        assert!(selection(&[7]).is_err()); // Invalid die.
        assert!(selection(&[2]).is_err()); // The 2 does not score.
        assert!(selection(&[1, 2]).is_err()); // The 2 does not score.
        assert!(selection(&[2, 2, 3, 3]).is_err()); // No scoring groups.
        assert!(selection(&[1, 2, 3, 4, 5, 2]).is_err()); // Straight with non scoring face.
    }
}
