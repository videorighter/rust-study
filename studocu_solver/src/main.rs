struct Sudoku {
    grid: [[u8; 9]; 9],
}

fn new_sudoku(numbers: &[u8; 81]) -> Sudoku {
    let mut grid = [[0; 9]; 9];
    for (index, &number) in numbers.iter().enumerate() {
        let row = index / 9;
        let col = index % 9;
        grid[row][col] = number;
    }
    Sudoku { grid }
}

fn print_sudoku(sudoku: &Sudoku) {
    for row in &sudoku.grid {
        for &num in row {
            print!("{} ", num);
        }
        println!();
    }
}

fn is_valid(sudoku: &Sudoku, row: usize, col: usize, num: u8) -> bool {
    for x in 0..9 {
        if sudoku.grid[row][x] == num || sudoku.grid[x][col] == num {
            return false;
        }
    }
    
    let start_row = (row / 3) * 3;
    let start_col = (col / 3) * 3;
    for i in 0..3 {
        for j in 0..3 {
            if sudoku.grid[start_row + i][start_col + j] == num {
                return false;
            }
        }
    }
    
    true
}

fn solve_sudoku(sudoku: &mut Sudoku) -> bool {
    for row in 0..9 {
        for col in 0..9 {
            if sudoku.grid[row][col] == 0 {
                for num in 1..=9 {
                    if is_valid(sudoku, row, col, num) {
                        sudoku.grid[row][col] = num;
                        if solve_sudoku(sudoku) {
                            return true;
                        }
                        sudoku.grid[row][col] = 0;
                    }
                }
                return false;
            }
        }
    }
    true
}

fn main() {
    let numbers: [u8; 81] = [
        // 81개 숫자 입력
    ];
    let mut sudoku = new_sudoku(&numbers);
    if solve_sudoku(&mut sudoku) {
        print_sudoku(&sudoku);
    } else {
        println!("No solution found");
    }
}
