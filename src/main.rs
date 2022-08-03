use itertools::Itertools;
use rand::prelude::*;

struct Dodosuko(ThreadRng);

impl Iterator for Dodosuko {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        ["ドド", "スコ"].choose(&mut self.0).copied()
    }
}

fn main() {
    Dodosuko(rand::thread_rng())
        .into_iter()
        .inspect(|x| print!("{}", x))
        .tuple_windows()
        .take_while(|item| {
            !matches!(
                item,
                (
                    "ドド", "スコ", "スコ", "スコ", "ドド", "スコ", "スコ", "スコ", "ドド", "スコ", "スコ", "スコ",
                )
            )
        })
        .for_each(|_| {});
    println!("ラブ注入♡");
}
