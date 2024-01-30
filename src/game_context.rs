use std::ops::Add;

use rand::Rng;

use crate::{GRID_X_SIZE, GRID_Y_SIZE};

#[derive(Copy, Clone)]
pub enum GameState {
    Playing,
    Paused,
    OverYes,
    OverNo,
    Quit,
    Restart,
}

pub enum PlayerDirection {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Point(pub i32, pub i32);

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Point {
    pub fn new() -> Point {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..(GRID_X_SIZE));
        let y = rng.gen_range(0..(GRID_Y_SIZE));
        Point(x, y)
    }
}

pub struct GameContext {
    pub player_position: Vec<Point>,
    pub player_direction: PlayerDirection,
    pub food: Point,
    pub state: GameState,
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            player_position: vec![Point(3, 1), Point(2, 1), Point(1, 1)],
            player_direction: PlayerDirection::Right,
            state: GameState::Paused,
            food: Point::new(),
        }
    }

    pub fn next_tick(&mut self) {
        if let GameState::Paused | GameState::OverYes | GameState::OverNo = self.state {
            return;
        }

        if !self.in_bounds() {
            self.state = GameState::OverYes;
            return;
        }

        let head_position = self.player_position.first().unwrap();
        let next_head_position = match self.player_direction {
            PlayerDirection::Up => *head_position + Point(0, -1),
            PlayerDirection::Down => *head_position + Point(0, 1),
            PlayerDirection::Right => *head_position + Point(1, 0),
            PlayerDirection::Left => *head_position + Point(-1, 0),
        };

        if self.food != next_head_position {
            self.player_position.pop();
        } else {
            self.new_food();
        }

        if self.player_position.contains(&next_head_position) {
            self.state = GameState::OverYes;
            return;
        }

        self.player_position.reverse();
        self.player_position.push(next_head_position);
        self.player_position.reverse();
    }

    fn new_food(&mut self) {
        self.food = Point::new();
    }

    fn in_bounds(&self) -> bool {
        let head_position = self.player_position.first().unwrap();
        head_position.0 > 0
            && head_position.0 < GRID_X_SIZE
            && head_position.1 > 0
            && head_position.1 < GRID_Y_SIZE
    }

    pub fn move_up(&mut self) {
        self.player_direction = PlayerDirection::Up;
    }

    pub fn move_down(&mut self) {
        self.player_direction = PlayerDirection::Down;
    }

    pub fn move_right(&mut self) {
        if let GameState::OverNo | GameState::OverYes = self.state {
            self.state = GameState::OverNo;
        } else {
            self.player_direction = PlayerDirection::Right;
        }
    }

    pub fn move_left(&mut self) {
        if let GameState::OverNo | GameState::OverYes = self.state {
            self.state = GameState::OverYes;
        } else {
            self.player_direction = PlayerDirection::Left;
        }
    }

    pub fn select(&mut self) {
        if let GameState::OverYes = self.state {
            self.state = GameState::Restart;
        } else if let GameState::OverNo = self.state {
            self.state = GameState::Quit;
        }
    }

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
            _ => self.state,
        }
    }
}
