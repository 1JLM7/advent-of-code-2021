use std::str::FromStr;

use platform::*;

#[derive(Debug)]
struct Day01;

impl Challenge for Day01 {
    fn stage1(self, data: String) -> anyhow::Result<()> {
        let num_increasing = self
            .numbers::<u16>(data)
            .windows(2)
            .filter(|arr| {
                let last = arr[0];
                let current = arr[1];
                current > last
            })
            .count();
        println!("Num increasing: {}", num_increasing);
        Ok(())
    }

    fn stage2(self, data: String) -> anyhow::Result<()> {
        let windows_sum =
            Vec::from_iter(self.numbers(data).windows(3).map(|l| l.iter().sum::<u16>()));
        let num_increasing = windows_sum
            .windows(2)
            .filter(|arr| {
                let last = arr[0];
                let current = arr[1];
                current > last
            })
            .count();
        println!("Num increasing: {}", num_increasing);
        Ok(())
    }
}

impl Day01 {
    fn numbers<T: FromStr>(&self, data: String) -> Vec<T> {
        log::warn!("Loading numbers from data");
        data.split_whitespace()
            .filter_map(|s| s.parse::<T>().ok())
            .collect()
    }
}

platform::challenge!(Day01);
