use std::str::FromStr;

use line::Line;
use platform::{
    anyhow::{self, Context},
    challenge, Challenge,
};
use regex::Regex;
use vec::{boundary, Vec2};

mod line;
mod vec;

struct VentList {
    vents: Vec<Line<u32>>,
}

impl FromStr for VentList {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"(\d+),(\d+)\s+->\s+(\d+),(\d+)").unwrap();
        let vents = s
            .lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| {
                let m = regex.captures(s.trim()).context("Syntax error")?;
                let line = Line::new(
                    Vec2::new(
                        m.get(1).unwrap().as_str().parse()?,
                        m.get(2).unwrap().as_str().parse()?,
                    ),
                    Vec2::new(
                        m.get(3).unwrap().as_str().parse()?,
                        m.get(4).unwrap().as_str().parse()?,
                    ),
                );
                Ok::<_, Self::Err>(line)
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { vents })
    }
}

impl VentList {
    fn isect_stage1(&self) -> usize {
        let (min, max) = match boundary(self.vents.iter().flat_map(|line| line.points()).copied()) {
            Some(v) => v,
            None => return 0,
        };

        itertools::iproduct!(min.y..max.y, min.x..max.x)
            .map(|(y, x)| Vec2::new(x, y))
            .filter(|vec| {
                if self.vents.iter().any(|l| l.on(vec)) {
                    log::info!("{:?} intersects lines", vec);
                    true
                } else {
                    false
                }
            })
            .count()
    }
}

#[derive(Debug)]
struct Day05;

impl Challenge for Day05 {
    fn stage1(self, data: String) -> anyhow::Result<()> {
        let vents = VentList::from_str(&data)?;
        let num_isect = vents.isect_stage1();
        println!("# intersecting (vert. and hor. only): {}", num_isect);
        Ok(())
    }
    fn stage2(self, data: String) -> anyhow::Result<()> {
        todo!()
    }
}

challenge!(Day05);

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use test_log::test;

    use crate::{line::Line, vec::Vec2, VentList};

    const TEST_DATA: &str = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_parsing() {
        let ventlist = VentList::from_str(TEST_DATA).unwrap();
        let vents = &ventlist.vents[..2];
        let expected = &[
            Line::new(Vec2::new(0, 9), Vec2::new(5, 9)),
            Line::new(Vec2::new(8, 0), Vec2::new(0, 8)),
        ];

        assert_eq!(vents, expected);
    }

    #[test]
    fn test_stage1() {
        let ventlist = VentList::from_str(TEST_DATA).unwrap();

        assert_eq!(5, ventlist.isect_stage1());
    }
}
