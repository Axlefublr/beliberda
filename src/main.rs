use std::io;
use std::io::Read;
use std::io::Write;
use std::ops::RangeInclusive;
use rand::seq::IteratorRandom;

const CHARS: RangeInclusive<char> = ' '..='~';
const LENGTH: usize = 50;

fn main() {
    let chars = CHARS.choose_multiple(&mut rand::thread_rng(), LENGTH);
    println!("{}", chars.into_iter().collect::<String>());
    io::stdout().flush().unwrap();
    let input = &mut [0u8; LENGTH];
    io::stdin().read_exact(input).unwrap();
    println!("{}", input.map())
}
