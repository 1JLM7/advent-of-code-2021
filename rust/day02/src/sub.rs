pub trait Sub {
    fn forward(self, amount: u16) -> Self;
    fn depth(self, amount: i16) -> Self;
    fn end_state(&self) -> u32;
}

#[derive(Debug, Default)]
pub struct Submarine {
    depth: u16,
    pos: u16,
}

impl Sub for Submarine {
    fn forward(mut self, amount: u16) -> Self {
        self.pos += amount;
        self
    }

    fn depth(mut self, amount: i16) -> Self {
        if amount < 0 {
            self.depth -= (-amount) as u16;
        } else {
            self.depth += amount as u16;
        }
        self
    }

    fn end_state(&self) -> u32 {
        self.pos as u32 * self.depth as u32
    }
}

#[derive(Debug, Default)]
pub struct SubAim {
    depth: u32,
    pos: u32,
    aim: u32,
}

impl Sub for SubAim {
    fn forward(mut self, amount: u16) -> Self {
        self.pos += amount as u32;
        self.depth += self.aim * amount as u32;
        self
    }

    fn depth(mut self, amount: i16) -> Self {
        if amount < 0 {
            self.aim -= (-amount) as u32;
        } else {
            self.aim += amount as u32;
        }
        self
    }

    fn end_state(&self) -> u32 {
        self.pos as u32 * self.depth as u32
    }
}
