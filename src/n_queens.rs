use std::{arch::x86_64::_mm_broadcastd_epi32, vec};

use crate::solution::Solution;

impl Solution {
    pub fn solve_n_queues(n: i32) -> Vec<Vec<String>> {
        // . blank Q . queue

        let mut board = vec![vec!['.'; n as usize]; n as usize];
        let mut solution: Vec<Vec<String>> = vec![];
        backtrace(&mut board, &mut solution, n, 0);
        solution
    }
}

fn backtrace(board: &mut Vec<Vec<char>>, solution: &mut Vec<Vec<String>>, n: i32, row: i32) {
    for colum in 0..n {
        if !collision(&board, n, row, colum) {
            board[row as usize][colum as usize] = 'Q';

            if row == n - 1 {
                solution.push(board.iter().map(|v| v.iter().collect()).collect());
            } else {
                backtrace(board, solution, n, row + 1);
            }

            //对应模板:撤销选择， 将该选择重新加如列表
            board[row as usize][colum as usize] = '.';
        }
    }
}

fn collision(board: &Vec<Vec<char>>, n: i32, row: i32, column: i32) -> bool {
    let mut up_row = row - 1;
    let mut left_colum = column - 1;
    let mut right_colum = column + 1;
    while up_row >= 0 {
        if (board[up_row as usize][column as usize] == 'Q') {
            return true;
        }
        if left_colum >= 0 && board[up_row as usize][left_colum as usize] == 'Q' {
            return true;
        }
        if right_colum < n && board[up_row as usize][right_colum as usize] == 'Q' {
            return true;
        }

        up_row -= 1;
        left_colum -= 1;
        right_colum += 1;
    }
    false
}
