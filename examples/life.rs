use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use std::{thread, time::Duration};

use tapestry::{
    patterns::{Neighborhood, Rect},
    Coord, VecGrid,
};

const FPS: f32 = 30.0;
const FRAME_MILLIS: f32 = 1000.0 / FPS;

fn main() {
    let mut life_board = LifeBoard::random((16, 16));

    loop {
        // Clear terminal
        print!("\x1B[2J\x1B[1;1H");
        // Map grid of characters into a grid of strings, addings spaces between the
        // characters.
        let display_grid = life_board.grid.map(|cell| {
            format!("{} ", {
                match cell {
                    LifeState::Alive => '#',
                    LifeState::Dead => '∙',
                }
            })
        });
        println!("{}", display_grid);
        life_board.step();
        thread::sleep(Duration::from_millis(FRAME_MILLIS as u64));
    }
}

struct LifeBoard {
    grid: VecGrid<LifeState>,
}

impl LifeBoard {
    fn random<C: Into<Coord>>(dimensions: C) -> Self {
        Self {
            grid: VecGrid::with_generator(Rect::new(dimensions), |(_x, _y)| {
                rand::random::<LifeState>()
            }),
        }
    }

    fn step(&mut self) {
        let neighbor_counts = self
            .grid
            .iter()
            .map(|cell| self.live_neighbor_count(cell.0))
            // `collect` to release the borrow on `self`.
            .collect::<Vec<_>>();

        for ((_coord, cell), neighbor_count) in self.grid.iter_mut().zip(neighbor_counts) {
            *cell = LifeBoard::compute_state(*cell, neighbor_count)
        }
    }

    fn live_neighbor_count(&self, coord: Coord) -> usize {
        self.grid
            .selection_iter(Neighborhood::new(coord).iter())
            .filter(|r_cell| {
                if let Ok(cell) = r_cell {
                    *cell.1 == LifeState::Alive
                } else {
                    false
                }
            })
            .count()
    }

    fn compute_state(state: LifeState, neighbor_count: usize) -> LifeState {
        match state {
            LifeState::Alive => {
                if neighbor_count == 2 || neighbor_count == 3 {
                    LifeState::Alive
                } else {
                    LifeState::Dead
                }
            }
            LifeState::Dead => {
                if neighbor_count == 3 {
                    LifeState::Alive
                } else {
                    LifeState::Dead
                }
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum LifeState {
    Dead,
    Alive,
}

impl Default for LifeState {
    fn default() -> Self {
        LifeState::Dead
    }
}

// Allows us to randomly generate LifeState values.
impl Distribution<LifeState> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LifeState {
        match rng.gen_bool(0.3) {
            true => LifeState::Alive,
            false => LifeState::Dead,
        }
    }
}
