//! Iterators over common [`Coord`](crate::coord::Coord) shapes and patterns.
//!
//! These patterns have no dependencies on actual cell data. Their intended use
//! is for ultimately passing into
//! [`Grid::selection_iter`](crate::grid::Grid::selection_iter) or
//! [`Grid::selection_iter_mut`](crate::grid::Grid::selection_iter_mut) to obtain
//! actual cell values.

use crate::coord::Coord;

/// Returns the orthogonal and diagonal (Moore) neighborhood of `coord`.
pub fn neighborhood<'a, C: Into<Coord>>(coord: C) -> impl Iterator<Item = Coord> + 'a {
    let coord = coord.into();
    [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ]
    .iter()
    .map(move |&offset| coord + offset.into())
}

/// Returns the orthogonal (Von Neumann) neighborhood of `coord`.
pub fn ortho_neighborhood<'a, C: Into<Coord>>(coord: C) -> impl Iterator<Item = Coord> + 'a {
    let coord = coord.into();
    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .iter()
        .map(move |&offset| coord + offset.into())
}

/// Returns the diagonal neighborhood of `coord` (for completeness).
pub fn diag_neighborhood<'a, C: Into<Coord>>(coord: C) -> impl Iterator<Item = Coord> + 'a {
    let coord = coord.into();
    [(1, 1), (1, -1), (-1, -1), (-1, 1)]
        .iter()
        .map(move |&offset| coord + offset.into())
}

/// Traces Bresenham's line algorithm between `from` and `to`.
pub fn line(from: Coord, to: Coord) -> impl Iterator<Item = Coord> {
    let delta = to - from;
    let x_step = Coord::new(delta.x.signum(), 0);
    let y_step = Coord::new(0, delta.y.signum());
    let x_is_major = delta.x.abs() > delta.y.abs();

    let (major_step, minor_step) = if x_is_major {
        (x_step, y_step)
    } else {
        (y_step, x_step)
    };

    let (major_fault, minor_fault) = if x_is_major {
        (delta.x.abs(), delta.y.abs())
    } else {
        (delta.y.abs(), delta.x.abs())
    };

    LineIter {
        end_coord: to,
        next_coord: from,
        major_step,
        minor_step,
        fault: major_fault as f32 / 2.0,
        major_fault,
        minor_fault,
        is_finished: false,
    }
}

struct LineIter {
    end_coord: Coord,
    next_coord: Coord,
    // Added to the coordinate every iteration.
    major_step: Coord,
    // Added to the coordinate when `fault` falls below zero.
    minor_step: Coord,
    fault: f32,
    // Amount to add to `fault` when it falls below zero.
    major_fault: i32,
    // Amount to subtract from `fault` every iteration.
    minor_fault: i32,
    is_finished: bool,
}

impl Iterator for LineIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Coord> {
        if self.is_finished {
            return None;
        }
        if self.next_coord == self.end_coord {
            self.is_finished = true;
            return Some(self.end_coord);
        }

        // We return the coordinate computed on the previous iteration
        let return_coord = self.next_coord;

        // TODO: AddAssign
        self.next_coord = self.next_coord + self.major_step;

        self.fault -= self.minor_fault as f32;
        // The choice of < over <= here seems arbitrary. The step patterns they
        // produce are mirror images of each other, for example:
        //  < 0.0 -- 3-4-4-5-4-3
        // <= 0.0 -- 3-4-5-4-4-3
        if self.fault < 0.0 {
            self.fault += self.major_fault as f32;
            // TODO: AddAssign
            self.next_coord = self.next_coord + self.minor_step;
        }

        Some(return_coord)
    }
}
