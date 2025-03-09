use std::io::{stdin, Read};

use color_eyre::Result;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut stones: Vec<u128> = input.split(' ').map(|o| o.parse().unwrap()).collect();
    // eprintln!("{:?}", stones);

    const STOP: usize = 25;
    let mut i = 0;
    while i < STOP {
        i += 1;

        let mut new = Vec::with_capacity(stones.len());
        for stone in &stones {
            if *stone == 0 {
                new.push(1)
            } else if let Ok((l, r)) = try_split(*stone) {
                new.push(l);
                new.push(r);
            } else {
                new.push(stone * 2024);
            };
        }
        // eprintln!("{:?}", new);
        std::mem::swap(&mut stones, &mut new);
    }

    println!("{}", stones.len());

    Ok(())
}

fn try_split(n: u128) -> std::result::Result<(u128, u128), ()> {
    let s = n.to_string();

    if s.len() % 2 == 0 {
        let (l, r) = s.split_at(s.len() / 2);
        Ok((l.parse().unwrap(), r.parse().unwrap()))
    } else {
        Err(())
    }
}
