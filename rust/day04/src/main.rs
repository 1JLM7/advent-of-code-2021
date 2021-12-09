use std::str::FromStr;

use board::{Board, Player};
use platform::{
    anyhow::{self, Context},
    challenge, Challenge,
};

mod board;

struct Game {
    draw_stack: Vec<u32>,
    players: Vec<Player>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("\n\n");
        let stack = parts
            .next()
            .context("Data is empty")?
            .rsplit(",") // Splitting in reverse to allow `.pop()` to return in normal order
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;
        let players = parts
            .map(|s| Board::from_str(s).map(Player::from))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            draw_stack: stack,
            players,
        })
    }
}

impl Game {
    fn play_all<'a>(&'a mut self) -> impl 'a + Iterator<Item = u32> {
        std::iter::from_fn(|| self.play())
    }

    fn play(&mut self) -> Option<u32> {
        while !self.draw_stack.is_empty() {
            if let Some((players, draw)) = self.step() {
                log::info!("{} player(s) wins drawing {}", players.len(), draw);
                let player = players.first().unwrap();
                let score = player.unmarked().sum::<u32>() * draw;
                log::info!("-> winning score: {}", score);
                return Some(score);
            }
        }

        None
    }

    fn step(&mut self) -> Option<(Vec<Player>, u32)> {
        let next = self.draw_stack.pop()?;
        for player in &mut self.players {
            player.draw(next);
        }

        let winners: Vec<_> = self
            .players
            .iter()
            .enumerate()
            .filter_map(|(i, p)| if p.winning() { Some(i) } else { None })
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .map(|i| self.players.remove(i))
            .collect();
        if !winners.is_empty() {
            Some((winners, next))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Day04;

impl Challenge for Day04 {
    fn stage1(self, data: String) -> anyhow::Result<()> {
        let mut game = Game::from_str(&data)?;
        if let Some(score) = game.play() {
            println!("Score: {}", score);
            Ok(())
        } else {
            anyhow::bail!("No players could win the game !")
        }
    }

    fn stage2(self, data: String) -> anyhow::Result<()> {
        let mut game = Game::from_str(&data)?;
        if let Some(score) = game.play_all().last() {
            println!("Last score: {}", score);
            Ok(())
        } else {
            anyhow::bail!("No players could win the game !")
        }
    }
}

challenge!(Day04);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::Game;

    const TEST_DATA: &str = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
    #[test]
    fn test_parsing() {
        let game = Game::from_str(TEST_DATA).unwrap();

        assert_eq!(&[1, 26, 3, 19], &game.draw_stack[..4]);

        assert!(game.players.iter().all(|p| p.board().shape() == (5, 5)))
    }

    #[test]
    fn test_stage1() {
        let mut game = Game::from_str(TEST_DATA).unwrap();

        assert_eq!(Some(4512), game.play())
    }

    #[test]
    fn test_stage2() {
        let mut game = Game::from_str(TEST_DATA).unwrap();
        let last_score = game.play_all().last();

        assert_eq!(Some(1924), last_score);
    }
}
