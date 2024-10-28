use std::io;

const SIZE: usize = 19;
const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (1, 1), (1, -1)];

fn is_valid(x: isize, y: isize) -> bool {
    x >= 0 && x < SIZE as isize && y >= 0 && y < SIZE as isize
}

fn check_winner(board: &[[i32; SIZE]; SIZE], x: usize, y: usize, player: i32) -> Option<(usize, usize)> {
    for &(dx, dy) in &DIRECTIONS {
        let mut count = 1;
        let mut nx = x as isize;
        let mut ny = y as isize;

        // 왼쪽/위쪽 방향으로 5개의 돌이 연속인지 체크
        for _ in 0..4 {
            nx += dx;
            ny += dy;
            if is_valid(nx, ny) && board[nx as usize][ny as usize] == player {
                count += 1;
            } else {
                break;
            }
        }

        // 연속된 돌이 5개일 때, 반대 방향으로 더 있는지 체크 (6목 방지)
        if count == 5 {
            let prev_x = x as isize - dx;
            let prev_y = y as isize - dy;
            if is_valid(prev_x, prev_y) && board[prev_x as usize][prev_y as usize] == player {
                continue;
            }
            return Some((x + 1, y + 1)); // 1-based index로 반환
        }
    }
    None
}

fn main() {
    let mut board = [[0; SIZE]; SIZE];

    // 입력 받기
    for i in 0..SIZE {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for j in 0..SIZE {
            board[i][j] = row[j];
        }
    }
    for i in 0..SIZE {
        for j in 0..SIZE {
            if board[i][j] != 0 {
                if let Some((x, y)) = check_winner(&board, i, j, board[i][j]) {
                    println!("{}", board[i][j]);
                    println!("{} {}", x, y);
                    return;
                }
            }
        }
    }
    println!("0");
}
