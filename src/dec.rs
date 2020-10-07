pub type SodokuMatrix = [[u8; 9]; 9];
type Coord = [usize; 2];

pub struct Sodoku {
    matrix: SodokuMatrix,
    coords: Vec<Coord>,
}

impl Sodoku {
    // fn new(matrix: SodokuMatrix) -> Self {
    //     Self {
    //         matrix,
    //         coords: Self::find_empty_coords(&matrix),
    //     }
    // }

    

    fn find_empty_coords(sodoku: &SodokuMatrix) -> Vec<Coord> {
        let mut coords: Vec<Coord> = Vec::new();
        for i in 0..9 {
            for j in 0..9 {
                if sodoku[i][j] == 0 {
                    coords.push([i, j]);
                }
            }
        }
        coords
    }

    /// Convert from a `[[u8; 9]; 9]` matrix (array-of-array) to a `Sodoku`
    pub fn from_matrix(matrix: SodokuMatrix) -> Self {
        matrix.into()
    }

    pub fn with_difficulty(score: u8) -> Self {
        unimplemented!();
    }

    /// Solve the sodoku
    pub fn solve(&mut self) {
        let mut k = 0;
        self[k] = 1;
        while !(self.finished() && self.valid()) {
            if self.valid() {
                k += 1;
                self[k] = 1;
            } else {
                while !self.valid() {
                    self.backtrace(&mut k);
                }
            }
        }
    }

    fn backtrace(&mut self, k: &mut usize) {
        let coord = self.coords[*k];
        let mut v = &mut self[coord];
        while *v == 9 {
            *v = 0;
            *k -= 1;
            let coord = self.coords[*k];
            v = &mut self[coord];
        }
        *v += 1;
    }

    fn finished(&self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if self[[i, j]] == 0 {
                    return false;
                }
            }
        }
        true
    }

    /// Check whether the sodoku is still valid
    fn valid(&self) -> bool {
        self.valid_rows() && self.valid_cols() && self.valid_box()
    }

    fn valid_rows(&self) -> bool {
        for i in 0..9 {
            let mut used = [false; 10];
            for j in 0..9 {
                let n = self[[i, j]];
                if n != 0 {
                    if used[n as usize] {
                        return false;
                    } else {
                        used[n as usize] = true
                    }
                }
            }
        }
        true
    }
    fn valid_cols(&self) -> bool {
        for j in 0..9 {
            let mut used = [false; 10];
            for i in 0..9 {
                let n = self[[i, j]];
                if n != 0 {
                    if used[n as usize] {
                        return false;
                    } else {
                        used[n as usize] = true
                    }
                }
            }
        }
        true
    }
    fn valid_box(&self) -> bool {
        let mut used = [[[false; 16]; 3]; 3];
        for i_ in 0..3 {
            // "outer" i and j; indexes of 3*3 square blocks
            for j_ in 0..3 {
                for i in 3 * i_..3 * i_ + 3 {
                    // "inner" i and j; indexes of individual cells
                    for j in 3 * j_..3 * j_ + 3 {
                        let n = self[[i, j]];
                        if n != 0 {
                            if used[i_][j_][n as usize] {
                                return false;
                            } else {
                                used[i_][j_][n as usize] = true
                            }
                        }
                    }
                }
            }
        }
        true
    }
}


impl std::convert::From<SodokuMatrix> for Sodoku {
    fn from(matrix: SodokuMatrix) -> Self {
        Self {
            matrix,
            coords: Self::find_empty_coords(&matrix),
        }
    }
}

impl std::ops::Index<usize> for Sodoku {
    type Output = u8;

    fn index(&self, i: usize) -> &Self::Output {
        let coord = self.coords[i];
        &self[coord]
    }
}

impl std::ops::Index<Coord> for Sodoku {
    type Output = u8;

    fn index(&self, coords: Coord) -> &Self::Output {
        &self.matrix[coords[0]][coords[1]]
    }
}

impl std::ops::IndexMut<Coord> for Sodoku {
    fn index_mut(&mut self, coords: Coord) -> &mut u8 {
        &mut self.matrix[coords[0]][coords[1]]
    }
}

impl std::ops::IndexMut<usize> for Sodoku {
    fn index_mut(&mut self, i: usize) -> &mut u8 {
        let coord = self.coords[i];
        &mut self.matrix[coord[0]][coord[1]]
    }
}

use std::fmt;

impl fmt::Display for Sodoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(180);
        for i in 0..9 {
            for j in 0..9 {
                s.push_str(&format!("{} ", self[[i, j]]))
            }
            s.push('\n')
        }
        write!(f, "{}", s)
    }
}
