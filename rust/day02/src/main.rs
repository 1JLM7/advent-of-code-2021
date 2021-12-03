use std::{fmt::Debug, str::FromStr};

use platform::{
    anyhow::{self, Context},
    challenge, Challenge,
};
use sub::Sub;

mod sub;

#[derive(Debug)]
struct Day02;

#[derive(Debug)]
enum Direction {
    FWD,
    UP,
    DOWN,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        let res = match s.trim() {
            "forward" => FWD,
            "up" => UP,
            "down" => DOWN,
            s => anyhow::bail!("Unknown command {:?}", s),
        };
        Ok(res)
    }
}

#[derive(Debug)]
struct Command(Direction, u16);

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let dir = words
            .next()
            .ok_or(anyhow::anyhow!("Missing command in {:?}", s))
            .and_then(|s| s.parse::<Direction>())?;
        let amount = words
            .next()
            .ok_or(anyhow::anyhow!("Missing amount in {:?}", s))
            .and_then(|s| s.parse::<u16>().context("Parsing command amount"))?;
        let cmd = Self(dir, amount);
        log::info!("Parsed command: {:?}", cmd);
        Ok(cmd)
    }
}

impl Challenge for Day02 {
    fn stage1(self, data: String) -> platform::anyhow::Result<()> {
        println!("End state: {}", process::<sub::Submarine>(data)?);
        Ok(())
    }
    fn stage2(self, data: String) -> platform::anyhow::Result<()> {
        println!("End state: {}", process::<sub::SubAim>(data)?);
        Ok(())
    }
}

fn process<T: Sub + Default + Debug>(data: String) -> anyhow::Result<u32> {
    let sub = data
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<Command>())
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .fold(T::default(), |sub, cmd| match cmd {
            Command(Direction::FWD, amt) => sub.forward(amt as _),
            Command(Direction::UP, amt) => sub.depth(-(amt as i16)),
            Command(Direction::DOWN, amt) => sub.depth(amt as _),
        });
    log::info!("Sub: {:?}", sub);
    Ok(sub.end_state())
}

#[cfg(test)]
mod tests {
    use super::process;
    use crate::sub::*;

    const DATA: &str = r"
forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn test_stage1() {
        let res = process::<Submarine>(String::from(DATA)).unwrap();
        assert_eq!(150, res);
    }

    #[test]
    fn test_stage2() {
        let res = process::<SubAim>(String::from(DATA)).unwrap();
        assert_eq!(900, res);
    }
}

challenge!(Day02);
