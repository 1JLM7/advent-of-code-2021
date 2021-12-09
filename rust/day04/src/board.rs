use std::str::FromStr;

use ndarray::{Array1, Array2, ArrayView2, Axis, Zip};
use platform::anyhow;
#[derive(Debug, Clone)]
pub struct Board {
    data: Array2<u32>,
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = combine_rows(
            s.lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|l| {
                    l.split_whitespace()
                        .map(|s| s.parse())
                        .collect::<Result<Array1<_>, _>>()
                })
                .collect::<Result<Vec<_>, _>>()?,
        );
        Ok(Self { data })
    }
}

impl Board {
    pub fn shape(&self) -> (usize, usize) {
        self.data.dim()
    }
}

fn combine_rows<T: Clone>(arrays: impl IntoIterator<Item = Array1<T>>) -> Array2<T> {
    arrays
        .into_iter()
        .fold(None::<Array2<T>>, |arr, row| {
            let row = row.broadcast((1, row.dim())).unwrap();
            if let Some(mut arr) = arr {
                arr.append(Axis(0), row).unwrap();
                Some(arr)
            } else {
                Some(row.to_owned())
            }
        })
        .unwrap_or_else(|| Array2::from_shape_fn((0, 0), |_| unreachable!()))
}

#[derive(Debug, Clone)]
pub struct Player {
    bingo: Board,
    draws: Array2<bool>,
}

impl From<Board> for Player {
    fn from(bingo: Board) -> Self {
        let draws = Array2::from_elem(bingo.data.dim(), false);
        Self { bingo, draws }
    }
}

impl Player {
    pub fn board(&self) -> &Board {
        &self.bingo
    }

    pub fn winning(&self) -> bool {
        self.draws.rows().into_iter().any(|r| r.iter().all(|&b| b))
            || self
                .draws
                .columns()
                .into_iter()
                .any(|c| c.into_iter().all(|&b| b))
    }
    pub fn draw(&mut self, number: u32) {
        self.bingo
            .data
            .indexed_iter()
            .filter_map(|((i, j), &v)| if v == number { Some((i, j)) } else { None })
            .for_each(|(i, j)| self.draws[(i, j)] = true)
    }

    pub(crate) fn unmarked(&self) -> impl Iterator<Item = u32> {
        Zip::from(self.bingo.data.view())
            .and(self.draws.view())
            .map_collect(|&v, &b| if !b { Some(v) } else { None })
            .into_iter()
            .filter_map(|x| x)
    }
}
