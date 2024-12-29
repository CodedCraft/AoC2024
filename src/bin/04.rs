advent_of_code::solution!(4);

// Converts the input string into a 2D matrix of characters
fn get_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

// General function to count occurrences of a word in a specific direction
fn count_word(
    matrix: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    dx: isize,
    dy: isize,
    word: &[char],
) -> u32 {
    let rows = matrix.len() as isize;
    let cols = matrix[0].len() as isize;

    for (k, &ch) in word.iter().enumerate() {
        let ni = i as isize + k as isize * dy;
        let nj = j as isize + k as isize * dx;

        if ni < 0 || nj < 0 || ni >= rows || nj >= cols || matrix[ni as usize][nj as usize] != ch {
            return 0;
        }
    }
    1
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut occurrences = 0;
    let matrix = get_matrix(input);
    let word = vec!['X', 'M', 'A', 'S'];

    // Define all 8 possible directions
    let directions = [
        (1, 0),   // Horizontal right
        (-1, 0),  // Horizontal left
        (0, 1),   // Vertical down
        (0, -1),  // Vertical up
        (1, 1),   // Diagonal down-right
        (1, -1),  // Diagonal down-left
        (-1, 1),  // Diagonal up-right
        (-1, -1), // Diagonal up-left
    ];

    // Iterate over every cell in the matrix
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            for &(dx, dy) in &directions {
                occurrences += count_word(&matrix, i, j, dx, dy, &word);
            }
        }
    }

    Some(occurrences)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = get_matrix(input);
    let mut occurrences = 0;

    for i in 0..(matrix.len() - 2) {
        for j in 0..(matrix[0].len() - 2) {
            // M.M
            // .A.
            // S.S
            if matrix[i][j] == 'M'
                && matrix[i][j + 2] == 'M'
                && matrix[i + 1][j + 1] == 'A'
                && matrix[i + 2][j] == 'S'
                && matrix[i + 2][j + 2] == 'S'
            {
                occurrences += 1;
                continue;
            }
            //S.M
            //.A.
            //S.M
            if matrix[i][j] == 'S'
                && matrix[i][j + 2] == 'M'
                && matrix[i + 1][j + 1] == 'A'
                && matrix[i + 2][j] == 'S'
                && matrix[i + 2][j + 2] == 'M'
            {
                occurrences += 1;
                continue;
            }
            //S.S
            //.A.
            //M.M
            if matrix[i][j] == 'S'
                && matrix[i][j + 2] == 'S'
                && matrix[i + 1][j + 1] == 'A'
                && matrix[i + 2][j] == 'M'
                && matrix[i + 2][j + 2] == 'M'
            {
                occurrences += 1;
                continue;
            }
            //M.S
            //.A.
            //M.S
            if matrix[i][j] == 'M'
                && matrix[i][j + 2] == 'S'
                && matrix[i + 1][j + 1] == 'A'
                && matrix[i + 2][j] == 'M'
                && matrix[i + 2][j + 2] == 'S'
            {
                occurrences += 1;
            }
        }
    }

    Some(occurrences)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
