mod field;
mod point;
mod cell;

use std::fmt;

pub use crate::point::*;
pub use crate::field::*;
pub use crate::cell::*;


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    GameOn,
    GamePaused,
    GameOver,
}

pub struct Game {
    field: Field,
    tail: Point<isize>,
    head: Point<isize>,
    score: usize,
    snake_len: usize,
    state: GameState,
    no_turn_back: Direction,
}

impl Game {
    pub fn new() -> Self {
        let (width, height) = (20, 10);
        let mut f = Field::with_size(width, height);
        let snake_len = 45;
        //
        let (head, tail) = f.init_snake::<isize>(15, 7);
        let (head2, tail2) = f.init_snake::<isize>(15, 8);
        for x in tail2.x..=head2.x {
            let y = 8;
            let index = Point{ x, y };
            let d = f[index].snake_direction().unwrap().opposite();
            f[index] = Snake(d);
        }
        f[tail2] = Snake(Direction::Up);
        let (head3, tail3) = f.init_snake::<isize>(15, 9);
        f[head3] = Snake(Direction::Up);
        // f.drop_food(snake_len).unwrap();
        f[Point { x: 11, y: 3 }] = Cell::Food;
        Game {
            field: f,
            tail: tail3,
            head: head,
            score: 45,
            snake_len: snake_len,
            state: GameState::GameOn,
            no_turn_back: Direction::Left,
        }
    }

    pub fn step(&mut self) -> GameState {
        if !matches!(self.state, GameState::GameOn) { return self.state }

        let next_point = self.field[&self.head]
            .clone()
            .snake_direction()
            .unwrap()
            .advance(self.head.clone())
            .wrap(&self.field.size());
        let next_cell = self.field[&next_point].clone();

        match next_cell {
            Empty => {
                self.move_tail();
                self.move_head()
            },
            Snake(_) if next_point == self.tail => {
                self.move_tail();
                self.move_head()
            },
            Snake(_) => {
                self.state = GameState::GameOver;
            },
            Food => {
                self.move_head();
                self.snake_len += 1;
                self.score += 1;
                let _ = self.drop_food();
            },
        }
        self.state
    }

    fn move_head(&mut self) {
        let head: Cell = self.field[&self.head].clone();
        let direction = head.snake_direction().unwrap();
        let head_next = direction.advance(self.head.clone()).wrap(&self.field.size());
        self.field[&head_next] = Cell::Snake(direction);
        self.head = head_next;
        self.no_turn_back = direction.opposite();
    }

    fn move_tail(&mut self) {
        let tail = self.field[&self.tail].clone();
        self.field[&self.tail] = Cell::Empty;
        self.tail = tail.snake_direction().unwrap().advance(self.tail.clone()).wrap(&self.field.size());
    }

    fn drop_food(&mut self) -> Result<(), ()> {
        self.field.drop_food(self.snake_len)
    }

    pub fn turn(&mut self, dir: Direction) -> Result<(), ()> {
        if self.state == GameState::GameOver { return Err(()) }
        if self.no_turn_back == dir { return Err(()) }
        match self.field[&self.head] {
            Cell::Snake(ref mut d) => *d = dir,
            _ => unreachable!()
        }
        Ok(())
    }

    pub fn quit(&mut self) { self.state = GameState::GameOver; }

    pub fn state(&self) -> GameState { self.state }

    pub fn pause(&mut self) {
        if matches!(self.state, GameState::GameOn) {
            self.state = GameState::GamePaused;
        }
    }

    pub fn unpause(&mut self) {
        if matches!(self.state, GameState::GamePaused) {
            self.state = GameState::GameOn;
        }
    }

    pub fn is_paused(&self) -> bool {
        matches!(self.state, GameState::GamePaused)
    }

    pub fn field(&self) -> &Field { &self.field }

    pub fn score(&self) -> usize { self.score }

    pub fn head(&self) -> Point<isize> { self.head }

    pub fn tail(&self) -> Point<isize> { self.tail }

    pub fn snake_len(&self) -> usize { self.snake_len }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, " score: {}", self.score)?;
        writeln!(f, "{}", self.field)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let f = Field::with_size(3, 5);
        assert_eq!(f.size(), (3, 5));
    }
    #[test]
    fn print_game() {
        let game = Game::new();
        println!("{}", game);
    }

    #[test]
    fn change_direction() {
        let mut game = Game::new();
        game.turn(Direction::Down).unwrap();
        game.step();
        assert_eq!(game.field[&game.head].snake_direction().unwrap(), Direction::Down);
        assert_eq!(game.no_turn_back, Direction::Up);
        assert_eq!(game.turn(Direction::Up), Err(()));
    }

    #[test]
    fn drop_food_test() {
        let mut game = Game::new();
        game.field.drop_food(game.snake_len).unwrap();
    }
}
