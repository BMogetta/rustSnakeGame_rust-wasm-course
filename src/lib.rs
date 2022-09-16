
use wasm_bindgen::prelude::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/random.js")]
extern {
  fn random(max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
  Up,
  Right,
  Down,
  Left
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
  Won,
  Lost,
  Played,
}

#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(usize);

struct Snake {
  body: Vec<SnakeCell>,
  direction: Direction,
}

impl Snake {
  fn new(spawn_index: usize, size: usize) -> Snake {

    let mut body = vec!();

    for i in 0..size  {
      body.push(SnakeCell(spawn_index - i));
    }

    Snake { 
      body,
      direction: Direction::Left,
    }
  }
}

#[wasm_bindgen]
pub struct World {
  width: usize,
  size: usize,
  snake: Snake,
  next_cell: Option<SnakeCell>,
  reward_cell: Option<usize>,
  status: Option<GameStatus>,
  points: usize,
}

#[wasm_bindgen]
impl World {
  pub fn new(width: usize, snake_idx: usize) -> World {

    let snake = Snake::new(snake_idx, 3);
    let size = width * width;
    
      World { 
        width,
        size,
        reward_cell: World::gen_reward_cel(size,&snake.body),
        snake,
        next_cell: None,
        status: None,
        points: 0,
      }
  }

  fn gen_reward_cel(max: usize, snake_body: &Vec<SnakeCell>) -> Option<usize> {
    let mut reward_cell;

    loop {
      reward_cell = random(max);
      if !snake_body.contains(&SnakeCell(reward_cell)) { break;}
    }
    Some(reward_cell)
  }

  pub fn reward_cell(&self) -> Option<usize> {
    self.reward_cell
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn points(&self) -> usize {
    self.points
  }

  pub fn snake_head_idx(&self) -> usize {
    self.snake.body[0].0
  }

  pub fn start_game(&mut self) {
    self.status = Some(GameStatus::Played);
  }

  pub fn game_status(&self) -> Option<GameStatus> {
    self.status
  }

  pub fn game_status_text(&self) -> String {
    match self.status {
      Some(GameStatus::Won) => String::from("You have won!"),
      Some(GameStatus::Lost) => String::from("You have lost!"),
      Some(GameStatus::Played) => String::from("Playing"),
      None => String::from("No Status"),
    }
  }

  pub fn change_snake_direction(&mut self, direction: Direction) {

    let next_cell = self.gen_next_snake_cell(&direction);

    if self.snake.body[1].0 == next_cell.0 { return; }
    
    self.next_cell = Option::Some(next_cell);
    self.snake.direction = direction;
  }

  pub fn snake_length(&self) -> usize {
    self.snake.body.len()
  }

  // *const is a raw pointer
  // borrowing rules doesn't apply to it
  pub fn snake_cells(&self) -> *const SnakeCell {
    self.snake.body.as_ptr()
  }

  pub fn step(&mut self) {

    match self.status {
      Some(GameStatus::Played) => {
        let temp = self.snake.body.clone();

        match self.next_cell {
          Some(cell) => {
            self.snake.body[0] = cell;
            self.next_cell = Option::None;
          },
          None => {
            self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
          }
        }
      
        let length = self.snake.body.len();
      
        for i in 1..length {
          self.snake.body[i] = SnakeCell(temp[i - 1].0)
        }

        if self.snake.body[1..self.snake_length()].contains(&self.snake.body[0]) {
          self.status = Some(GameStatus::Lost)
        }
      
        if self.reward_cell == Some(self.snake_head_idx()) {
        
          if self.snake_length() < self.size {
            self.points +=1;
            self.reward_cell = World::gen_reward_cel(self.size, &self.snake.body);
          } else {
            self.reward_cell = None;
            self.status = Some(GameStatus::Won)
          }
        
          self.snake.body.push(SnakeCell(self.snake.body[1].0));
        }
      },
      _ => {}
    }
  }

  fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
    
    let snake_idx = self.snake_head_idx();
    let row = snake_idx / self.width;

    return match direction {
      Direction::Right => {
        SnakeCell((row * self.width) + (snake_idx + 1) % self.width)
      },
      Direction::Left => {
        SnakeCell((row * self.width) + (snake_idx - 1) % self.width)
      },
      Direction::Up => {
        SnakeCell((snake_idx - self.width) % self.size)
      },
      Direction::Down => {
        SnakeCell((snake_idx + self.width) % self.size)
      },
    };
  }
}