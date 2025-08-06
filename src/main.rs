mod grid;

use std::{collections::{HashSet, VecDeque}, io};
use strum::{EnumIter, IntoEnumIterator};

use scanner_rust::ScannerAscii;

use crate::grid::Grid;

pub const SIZE: i32 = 5;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Maze {
    grid: Grid<bool>,
    start: Pos,
    end: Pos,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn move_(&self, m: Move) -> Self {
        let (dx, dy) = m.to_dir();
        Pos {
            x: (self.x + dx).clamp(0, SIZE - 1),
            y: (self.y + dy).clamp(0, SIZE - 1),
        }
    }

    fn to_tuple(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
enum Move {
    Up, Down, Left, Right
}

impl Move {
    fn to_char(self) -> char {
        match self {
            Move::Up => 'U',
            Move::Down => 'D',
            Move::Left => 'L',
            Move::Right => 'R',
        }
    }

    fn to_dir(self) -> (i32, i32) {
        match self {
            Move::Up => (0, -1),
            Move::Down => (0, 1),
            Move::Left => (-1, 0),
            Move::Right => (1, 0),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    positions: Vec<Pos>,
    moves: Vec<Move>,
}

fn solve(mazes: &[Maze]) -> Vec<Move>{
    let initial_state = State {
        positions: mazes.iter().map(|m| m.start).collect(),
        moves: Vec::new()
    };
    let mut states = VecDeque::new();
    states.push_back(initial_state.clone());

    let mut visited = HashSet::new();
    visited.insert(initial_state.positions);

    while let Some(state) = states.pop_front() {
        eprintln!("Looking at {state:?}");
        for m in Move::iter() {
            // Update State
            let mut new_state = state.clone();
            for (maze, pos) in mazes.iter().zip(new_state.positions.iter_mut()) {
                let new_pos = pos.move_(m);
                if !maze.grid[new_pos.to_tuple()] {
                    *pos = new_pos;
                }

            }
            new_state.moves.push(m);

            // Check Win condition
            if mazes.iter().zip(&new_state.positions).all(|(maze, &pos)| maze.end == pos) {
                return new_state.moves
            }

            if visited.insert(new_state.positions.clone()) {
                eprintln!("Pushing {new_state:?}");
                states.push_back(new_state);
            }
        }
    }

    unreachable!("No solution found");
}

fn main() {
    let mut sc = ScannerAscii::new(io::stdin());

    let num_boards = sc.next_u32().unwrap().unwrap();
    let mut boards = Vec::new();

    for _ in 0..num_boards {
        let start_y = sc.next_i32().unwrap().unwrap();
        let start_x = sc.next_i32().unwrap().unwrap();
        let end_y = sc.next_i32().unwrap().unwrap();
        let end_x = sc.next_i32().unwrap().unwrap();

        let mut grid = Grid::new(SIZE as _, SIZE as _, false);
        let num_walls = sc.next_usize().unwrap().unwrap();

        for _ in 0..num_walls {
            let y = sc.next_i32().unwrap().unwrap();
            let x = sc.next_i32().unwrap().unwrap();

            grid[(x, y)] = true;
        }

        boards.push(dbg!(Maze{
            grid,
            start: Pos { x: start_x, y: start_y},
            end: Pos{x: end_x, y: end_y},
        }));
    }

    let result = solve(&boards);
    let result: String = result.into_iter().map(Move::to_char).collect();
    println!("{result}")
}