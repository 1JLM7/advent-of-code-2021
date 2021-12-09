use enum_map::{Enum, EnumMap};
use ndarray::{Array1, Axis};
use platform::{
    anyhow::{self, Context},
    challenge, Challenge,
};
use std::{fmt, str::FromStr};

use ndarray::Array2;

#[derive(Debug, Clone, Enum, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Bit {
    Z,
    I,
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Z => write!(f, "0"),
            Self::I => write!(f, "1"),
        }
    }
}

impl TryFrom<char> for Bit {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '0' => Self::Z,
            '1' => Self::I,
            _ => anyhow::bail!("Unknown char {:?}", value),
        })
    }
}

impl From<Bit> for usize {
    fn from(bit: Bit) -> usize {
        match bit {
            Bit::Z => 0,
            Bit::I => 1,
        }
    }
}

impl Bit {
    fn bin2dec(iter: impl Iterator<Item = Self>) -> usize {
        iter.fold(0, |num, bit| 2 * num + usize::from(bit))
    }
}

struct Diagnostic {
    data: Array2<Bit>,
}

impl FromStr for Diagnostic {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        if lines.is_empty() {
            anyhow::bail!("Data is empty");
        }

        let cols = lines[0].len();
        let rows = lines.len();

        let data = lines
            .into_iter()
            .flat_map(|s| s.chars().map(Bit::try_from))
            .collect::<Result<Array1<_>, _>>()?
            .into_shape((rows, cols))
            .context("Cannot reshape array")?;
        Ok(Self { data })
    }
}

impl Diagnostic {
    pub fn power_consumption(&self) -> usize {
        self.gamma_rate() * self.epsilon_rate()
    }

    pub fn life_support_rating(&self) -> usize {
        self.oxygen_rate() * self.co2_rate()
    }

    pub fn gamma_rate(&self) -> usize {
        self.stage1_rate(|map| {
            if map[Bit::I] > map[Bit::Z] {
                Bit::I
            } else {
                Bit::Z
            }
        })
    }

    pub fn epsilon_rate(&self) -> usize {
        self.stage1_rate(|map| {
            if map[Bit::I] > map[Bit::Z] {
                Bit::Z
            } else {
                Bit::I
            }
        })
    }

    pub fn oxygen_rate(&self) -> usize {
        self.stage2_rate(|map| {
            if map[Bit::I] >= map[Bit::Z] {
                Bit::I
            } else {
                Bit::Z
            }
        })
    }

    pub fn co2_rate(&self) -> usize {
        self.stage2_rate(|map| {
            if map[Bit::I] >= map[Bit::Z] {
                Bit::Z
            } else {
                Bit::I
            }
        })
    }

    fn stage1_rate(&self, choose: impl Fn(EnumMap<Bit, usize>) -> Bit) -> usize {
        Bit::bin2dec((0..self.data.shape()[1]).map(|i| choose(self.count_bit(i))))
    }

    fn stage2_rate(&self, choose: impl Fn(EnumMap<Bit, usize>) -> Bit) -> usize {
        let mut numbers = self.data.clone();

        for i in 0..(self.data.shape()[1]) {
            let counts = numbers.column(i).fold(EnumMap::default(), |mut map, bit| {
                map[*bit] += 1;
                map
            });
            numbers = numbers
                .rows()
                .into_iter()
                .filter(|r| r[i] == choose(counts))
                .fold(
                    Array2::from_shape_simple_fn((0, numbers.dim().1), || unreachable!()),
                    |mut arr, row| {
                        arr.append(Axis(0), row.broadcast((1, row.dim())).unwrap())
                            .unwrap();
                        arr
                    },
                );
            if numbers.dim().0 == 1 {
                return Bit::bin2dec(numbers.row(0).into_iter().copied());
            }
        }
        if numbers.dim().0 == 1 {
            Bit::bin2dec(numbers.row(0).into_iter().copied())
        } else {
            0
        }
    }

    fn count_bit(&self, bit: usize) -> EnumMap<Bit, usize> {
        self.data
            .column(bit)
            .fold(EnumMap::default(), |mut map, bit| {
                map[*bit] += 1;
                map
            })
    }
}

#[derive(Debug)]
struct Day03;

impl Challenge for Day03 {
    fn stage1(self, data: String) -> anyhow::Result<()> {
        let rate = data.parse::<Diagnostic>()?.power_consumption();
        println!("Power consumption: {}", rate);
        Ok(())
    }

    fn stage2(self, data: String) -> anyhow::Result<()> {
        let rate = data.parse::<Diagnostic>()?.life_support_rating();
        println!("Life support rating: {}", rate);
        Ok(())
    }
}

challenge!(Day03);

#[cfg(test)]
mod tests {
    use crate::Bit;

    use super::Diagnostic;

    const TEST_DATA: &str = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    #[test]
    fn test_data_parsing() {
        let diag: Diagnostic = TEST_DATA.parse().unwrap();
        assert_eq!([12, 5], diag.data.shape());
    }

    #[test]
    fn test_bit_counting() {
        let diag: Diagnostic = TEST_DATA.parse().unwrap();
        let counts = diag.count_bit(0);
        assert_eq!(counts[Bit::Z], 5);
        assert_eq!(counts[Bit::I], 7);
    }

    #[test]
    fn test_stage1() {
        let diag: Diagnostic = TEST_DATA.parse().unwrap();
        let gamma = diag.gamma_rate();
        let epsilon = diag.epsilon_rate();

        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
    }

    #[test]
    fn test_stage2() {
        let diag: Diagnostic = TEST_DATA.parse().unwrap();
        let oxygen = diag.oxygen_rate();
        let co2 = diag.co2_rate();

        assert_eq!(oxygen, 23);
        assert_eq!(co2, 10);
    }
}
