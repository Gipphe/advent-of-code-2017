const UP: u32 = 0;
const RIGHT: u32 = 1;
const DOWN: u32 = 2;
const LEFT: u32 = 3;

fn find_manhattan(number: u32) -> u32 {
    let mut curr = 1;
    let mut dir = RIGHT;
    let mut horizontal_length = 1;
    let mut vertical_length = 1;
    let mut dist_covered = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    while curr != number {
        match dir {
            UP => {
                if dist_covered >= vertical_length {
                    dist_covered = 0;
                    vertical_length += 1;
                    dir = LEFT;
                    continue;
                }
                y -= 1;
            },
            RIGHT => {
                if dist_covered >= horizontal_length {
                    dist_covered = 0;
                    horizontal_length += 1;
                    dir = UP;
                    continue;
                }
                x += 1;
            },
            DOWN => {
                if dist_covered >= vertical_length {
                    dist_covered = 0;
                    vertical_length += 1;
                    dir = RIGHT;
                    continue;
                }
                y += 1;
            },
            LEFT => {
                if dist_covered >= horizontal_length {
                    dist_covered = 0;
                    horizontal_length += 1;
                    dir = DOWN;
                    continue;
                }
                x -= 1;
            },
            _ => panic!("Unknown direction"),
        };
        dist_covered += 1;
        curr += 1;
    }
    (x.abs() + y.abs()) as u32
}

fn sum_neighbors(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let col_length = grid[0].len();
    let row_length = grid.len();
    let mut sum = 0;
    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }
            let real_y = y as i32 + i;
            let real_x = x as i32 + j;
            if
                real_y < 0 || real_y >= row_length as i32 ||
                real_x < 0 || real_x >= col_length as i32
            {
                continue;
            }
            let val = grid[real_y as usize][real_x as usize];
            if val == 0 {
                continue;
            }
            sum += val;
        }
    }
    sum
}

fn grid_sums(input: u32) -> u32 {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for i in 0..200 {
        let mut row: Vec<u32> = Vec::new();
        for j in 0..200 {
            row.push(0);
        }
        grid.push(row);
    }
    let middle_y = grid.len() / 2;
    let middle_x = grid[0].len() / 2;
    grid[middle_y][middle_x] = 1;

    let mut x: usize = middle_x;
    let mut y: usize = middle_y;
    let mut dir = RIGHT;
    let mut horizontal_length = 1;
    let mut vertical_length = 1;
    let mut dist_covered = 0;
    let mut curr = 0;
    while curr <= input {
        match dir {
            UP => {
                if dist_covered >= vertical_length {
                    dist_covered = 0;
                    vertical_length += 1;
                    dir = LEFT;
                    continue;
                }
                y -= 1;
            },
            RIGHT => {
                if dist_covered >= horizontal_length {
                    dist_covered = 0;
                    horizontal_length += 1;
                    dir = UP;
                    continue;
                }
                x += 1;
            },
            DOWN => {
                if dist_covered >= vertical_length {
                    dist_covered = 0;
                    vertical_length += 1;
                    dir = RIGHT;
                    continue;
                }
                y += 1;
            },
            LEFT => {
                if dist_covered >= horizontal_length {
                    dist_covered = 0;
                    horizontal_length += 1;
                    dir = DOWN;
                    continue;
                }
                x -= 1;
            },
            _ => panic!("Unknown direction"),
        }
        dist_covered += 1;
        curr = sum_neighbors(&grid, x, y);
        if curr == 0 {
            curr = 1;
        }
        grid[y][x] = curr;
    }
    curr
}

// 37 36 35 34 33 32 31
// 38 17 16 15 14 13 30
// 39 18  5  4  3 12 29
// 40 19  6  1  2 11 28
// 41 20  7  8  9 10 27
// 42 21 22 23 24 25 26
// 43 44 45 46 47 48 49

fn main() {
    let input = 347991;
    let first = find_manhattan(input);
    let second = grid_sums(input);

    println!("First: {}", first);
    println!("Second: {}", second);
}
