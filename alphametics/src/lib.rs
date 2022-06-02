use std::collections::HashMap;
use std::collections::HashSet;

// LeetCode 1307: Verbal Arithmetic Puzzle.
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut equation = parse(input);
    // Leading letters cannot be zero
    let non_zero_letters: HashSet<char> = equation
        .iter()
        .filter(|w| w.len() > 1)
        .map(|w| w[w.len() - 1])
        .collect();

    let result = equation.pop().unwrap();
    // Validate equation, "ABC + DEF == GH" is invalid,
    // sum isn't wide enough
    if equation.iter().any(|w| w.len() > result.len()) {
        return None;
    }

    let solution = &mut HashMap::new();
    if can_solve(&equation, &result, &non_zero_letters, 0, 0, 0, solution) {
        Some(solution.clone())
    } else {
        None
    }
}

/* SEND + MORE = MONEY is parsed into:
 * [
 *   ['D', 'N', 'E', 'S'],
 *   ['E', 'R', 'O', 'M'],
 *   ['Y', 'E', 'N', 'O', 'M']
 * ]
 */
fn parse(s: &str) -> Vec<Vec<char>> {
    s.split_whitespace()
        .map(|x| x.trim().to_ascii_uppercase())
        .filter(|x| x.chars().all(|y| y.is_ascii_uppercase()))
        .map(|x| x.chars().rev().collect())
        .collect()
}

/*
 * Exit conditions:
 *   If we are beyond the leftmost digit of the sum:
 *     Return true if no carry, false otherwise.
 *     Also check that there is no leading zero in the sum.
 *   Else if addend and current column index is beyond the current row:
 *     Recur on row beneath this one.
 *
 * If we are currently trying to assign a char in one of the addends:
 *   If char already assigned, recur on row beneath this one.
 *   If not assigned, then:
 *     For every possible choice among the digits not in use:
 *       Make that choice and recur on row beneath this one.
 *         If successful, return true.
 *         Else, unmake assignment and try another digit.
 *     Return false if no assignment worked to trigger backtracking.
 *
 * Else if trying to assign a char in the sum:
 *   If char already assigned:
 *     If matches the sum digit, recur on next column to the left with carry.
 *     Else, return false to trigger backtracking.
 *   If char unassigned:
 *     If correct digit already used, return false.
 *     Else:
 *       Assign it and recur on next column to the left with carry:
 *         If successful return true.
 *         Else, unmake assignment, and return false to trigger backtracking.
 */
fn can_solve(
    equation: &[Vec<char>],
    result: &[char],
    non_zero_letters: &HashSet<char>,
    row: usize,
    col: usize,
    carry: u32,
    solution: &mut HashMap<char, u8>,
) -> bool {
    let addend = row < equation.len();
    let word = if addend { &equation[row] } else { result };
    let n = word.len();

    if col >= n && addend {
        return can_solve(
            equation,
            result,
            non_zero_letters,
            row + 1,
            col,
            carry,
            solution,
        );
    }
    if col == n && !addend {
        return carry == 0;
    }

    let letter = word[col];
    let assigned = solution.contains_key(&letter);

    if addend {
        if assigned {
            can_solve(
                equation,
                result,
                non_zero_letters,
                row + 1,
                col,
                carry + (solution[&letter] as u32),
                solution,
            )
        } else {
            let used: HashSet<&u8> = HashSet::from_iter(solution.values());
            let unused: Vec<u8> = (0..=9).filter(|x| !used.contains(x)).collect();
            for i in unused {
                if i == 0 && non_zero_letters.contains(&letter) {
                    continue;
                }
                solution.insert(letter, i);
                if can_solve(
                    equation,
                    result,
                    non_zero_letters,
                    row + 1,
                    col,
                    carry + (i as u32),
                    solution,
                ) {
                    return true;
                }
                solution.remove(&letter);
            }
            false
        }
    } else {
        let sum_digit = (carry % 10) as u8;
        if assigned {
            (solution[&letter] == sum_digit)
                && can_solve(
                    equation,
                    result,
                    non_zero_letters,
                    0,
                    col + 1,
                    carry / 10,
                    solution,
                )
        } else {
            let used = solution.values().any(|&x| x == sum_digit);
            if used {
                return false;
            }
            if sum_digit == 0 && non_zero_letters.contains(&letter) {
                return false;
            }
            solution.insert(letter, sum_digit);
            if can_solve(
                equation,
                result,
                non_zero_letters,
                0,
                col + 1,
                carry / 10,
                solution,
            ) {
                return true;
            }
            solution.remove(&letter);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_leading_zero_in_multicharacter_words() {
        assert_eq!(solve("CBA + CBA + CBA + CBA + CBA = EDD"), None);
        assert_eq!(solve("AA + BB = AA"), None);
    }

    #[test]
    fn test_with_leading_zero_in_single_character_word() {
        assert_eq!(
            solve("A + B = A"),
            Some(HashMap::from([('A', 1), ('B', 0)]))
        );
    }
}
