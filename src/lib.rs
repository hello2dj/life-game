mod utils;

use js_sys;
use web_sys;
use std::fmt;
use cfg_if::cfg_if;
use std::ops::Drop;
use web_sys::console;
use wasm_bindgen::prelude::*;


pub struct Timer<'a> {
  name: &'a str,
}

impl<'a> Timer<'a> {
  pub fn new(name: &'a str) -> Timer<'a> {
    console::time_with_label(name);
    Timer { name }
  }
}

impl<'a> Drop for Timer<'a> {
  fn drop(&mut self) {
    console::time_end_with_label(self.name)
  }
}

macro_rules! log {
  ( $( $t: tt )*) => {
    web_sys::console::log_1(&format!( $( $t) *).into());
  };
}

cfg_if::cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, dengjie!");
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
  Dead = 0,
  Alive = 1,
}

impl Cell {
  fn toggle(&mut self) {
    *self = match *self {
      Cell::Dead => Cell::Alive,
      Cell::Alive => Cell::Dead,
    };
  }
}

#[wasm_bindgen]
pub struct Universe {
  width: u32,
  height: u32,
  cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
  pub fn toggle_cell(&mut self, row: u32, column: u32) {
    let idx = self.get_index(row, column);
    self.cells[idx].toggle();
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }

  pub fn cells(&self) -> *const Cell {
    self.cells.as_ptr()
  }

  pub fn new() -> Universe {
    let width = 64;
    let height = 64;

    let cells = (0..width * height)
      .map(|i| {
        if js_sys::Math::random() < 0.5 {
          Cell::Alive
        } else {
          Cell::Dead
        }
      })
      .collect();

    Universe {
      width,
      height,
      cells,
    }
  }

  pub fn render(&self) -> String {
    self.to_string()
  }

  fn get_index(&self, row: u32, column: u32) -> usize {
    (row * self.width + column) as usize
  }

  fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    let mut count = 0;

    for delta_row in [self.height - 1, 0, 1].iter().cloned() {
         for delta_col in [self.width - 1, 0, 1].iter().cloned() {
           if delta_row == 0 && delta_col == 0 {
             continue;
           }

           let neighhor_row = (row + delta_row) % self.height;
           let neighhor_col = (column + delta_col) % self.width;

           let idx = self.get_index(neighhor_row, neighhor_col);
           count += self.cells[idx] as u8
      }
    }
    count
  }

  pub fn tick(&mut self) {
    let _timer = Timer::new("Universe::tick");

    let mut next = self.cells.clone();

    for row in 0..self.height {
      for col in 0..self.width {
        let idx = self.get_index(row, col);
        let cell = self.cells[idx];
        let live_nieghbors = self.live_neighbor_count(row, col);

        // log!(
        //             "cell[{}, {}] is initially {:?} and has {} live neighbors",
        //             row,
        //             col,
        //             cell,
        //             live_nieghbors
        //         );

        let next_cell = match (cell, live_nieghbors) {
          (Cell::Alive, x) if x < 2 => Cell::Dead,
          (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
          (Cell::Alive, x) if x > 3 => Cell::Dead,
          (Cell::Dead, 3) => Cell::Alive,
          (otherwise, _) => otherwise,
        };

        next[idx] = next_cell;
      }
    }

    self.cells = next;
  }
}

impl fmt::Display for Universe {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for line in self.cells.as_slice().chunks(self.width as usize) {
      for &cell in line {
        let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
        write!(f, "{}", symbol)?
      }
      write!(f, "\n")?;
    }
    Ok(())
  }
}