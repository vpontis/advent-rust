type Grid = Vec<Vec<usize>>;

#[aoc(day15, part1, for_hash)]
pub fn solve_part1(input: &str) -> usize {
    let mut grid: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();

        for char in line.chars() {
            row.push(char.to_digit(10).unwrap() as usize);
        }

        let col_len = row.len();
        for i in 1..5 {
            for x in 0..col_len {
                row.push(
                    shift_val(
                        row[col_len * (i - 1) + x]
                    )
                );
            }
        }

        grid.push(row);
    }

    let row_len = grid.len();
    let col_len = grid[0].len();

    for i in 1..5 {
        for y in 0..row_len {
            let mut row = Vec::new();
            for x in 0..col_len {
                row.push(
                    shift_val(
                        grid[row_len * (i - 1) + y][x]
                    )
                );
            }
            grid.push(row);
        }
    }

    let mut grid_str = String::from("");
    for row in grid.iter() {
        for col in row {
            grid_str += &col.to_string();
        }
        grid_str += "\n";
    }
    println!("{}", &grid_str);

    let row_len = grid.len();

    let mut num_updated = 1;
    let mut risk_grid = vec![vec![100000 as usize; col_len]; row_len];
    risk_grid[0][0] = 0;

    while num_updated > 0 {
        num_updated = 0;

        for y in 0..row_len {
            for x in 0..col_len {
                if y == x && x == 0 {
                    continue;
                }

                // Check all neighbors to set the risk as equal to
                let cur_risk = risk_grid[y][x];
                let point_risk = grid[y][x];

                for nrisk in get_neighbor_values(&Point { x, y }, &risk_grid) {
                    let new_risk = nrisk + point_risk;
                    if new_risk < cur_risk {
                        num_updated += 1;
                        risk_grid[y][x] = new_risk;
                    }
                }
            }
        }
        println!("updated {}", num_updated);
    }

    risk_grid[row_len - 1][col_len - 1]
}

fn get_neighbor_values(point: &Point, grid: &Grid) -> Vec<usize> {
    let mut neighbors: Vec<usize> = Vec::new();

    for mut dx in 0..3 {
        for mut dy in 0..3 {
            if dy == dx && dy == 1 {
                continue;
            }

            // Don't do diagonals
            if dy != 1 && dx != 1 {
                continue;
            }

            // Can't do a negative index
            if point.y + dy == 0 || point.x + dx == 0 {
                continue;
            }

            let newy = point.y + dy - 1;
            let newx = point.x + dx - 1;

            if let Some(row) = grid.get(newy as usize) {
                if let Some(&v) = row.get(newx as usize) {
                    neighbors.push(v)
                }
            }
        }
    }

    neighbors
}

#[derive(Hash, Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}


fn shift_val(val: usize) -> usize {
    if val == 9 {
        return 1;
    }

    val + 1
}
