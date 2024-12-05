advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    // we need to start a search in all directions starting from each cell in the grid
    // the grid is <vec<vec<char>>
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    // grid.iter().for_each(|r| {
    //    println!("{r:?}");
    // });

    let wc: Vec<char> =  "XMAS".chars().collect();
    let k = wc.len();

    let directions = [
        (-1, -1), // up-left
        (-1, 0),  // up
        (-1, 1),  // up-right
        (0, -1),  // left
        (0, 1),   // right
        (1, -1),  // down-left
        (1, 0),   // down
        (1, 1),   // down-right
    ];

    let mut sum = 0;
    for r in 0..rows {
        for c in 0..cols {
            for (dx, dy) in directions { // start search in each direction
                let (mut x, mut y) = (r as i32, c as i32);
                // println!("({x},{y}):{}, going ({dx},{dy})", grid[x as usize][y as usize]);
                let mut i = 0;
                while i < k {
                    if x == rows as i32 || x < 0 || y == cols as i32|| y < 0 {
                        break; // out of the boundaries
                    };
                    if grid[x as usize][y as usize] == wc[i] {
                        // println!("Match at ({x},{y}):{}, i:{i} going ({dx},{dy})", grid[x as usize][y as usize]);
                        if i == k - 1 {
                            sum += 1;
                            break;
                        } else {
                            x += dx;
                            y += dy;
                            i += 1;
                        }
                    } else {
                        break;
                    };
                }
            }
        }
    }
    Some(sum as u32)
}

fn check(grid: &Vec<Vec<char>>, x: i32, y: i32, letter: char) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();
    if x == rows as i32 || x < 0 || y == cols as i32|| y < 0 { return false };
    println!("check: ({x},{y}): {}={}: {}",  grid[x as usize][y as usize], letter, grid[x as usize][y as usize] == letter);
    grid[x as usize][y as usize] == letter
}
pub fn part_two(input: &str) -> Option<u32> {
    // walk evert grid cell and check for every A if the diaganal elements are M + S
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut sum = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c].eq(&'A') {
                println!("A: ({r},{c})");
                if (
                    (check(&grid, r as i32 - 1, c as i32 - 1, 'M') && check(&grid, r as i32 + 1, c as i32 + 1, 'S')) ||
                    (check(&grid, r as i32 - 1, c as i32 - 1, 'S') && check(&grid, r as i32 + 1, c as i32 + 1, 'M'))
                ) && (
                    (check(&grid, r as i32 - 1, c as i32 + 1, 'M') && check(&grid, r as i32 + 1, c as i32 - 1, 'S')) ||
                    (check(&grid, r as i32 - 1, c as i32 + 1, 'S') && check(&grid, r as i32 + 1, c as i32 - 1, 'M'))
                ){
                    println!("({r},{c}): {}", grid[r][c]);
                    sum += 1;
                }
            }
        }
    }
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two_example() {
        let s = r".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        let r = part_two(s);
        assert_eq!(r, Some(9_u32));
    }


    #[test]
    fn test_part_one_example() {
        let s = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let r = part_one(s);
        assert_eq!(r, Some(18_u32));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
