use std::io::{stdin, Read};

use color_eyre::Result;

fn cat(a: u128, b: u128) -> u128 {
    (a.to_string() + &b.to_string()).parse().unwrap()
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut ans = 0;

    for line in input.lines() {
        let (tv, ns) = line.split_once(": ").unwrap();
        let tv: u128 = tv.parse().unwrap();
        let ns: Vec<u128> = ns.split(' ').map(|n| n.parse().unwrap()).collect();

        fn recurse(tv: u128, ns: &[u128], acc: u128) -> bool {
            if ns.len() == 1 {
                ns[0] + acc == tv || ns[0] * acc == tv || cat(acc, ns[0]) == tv
            } else {
                recurse(tv, &ns[1..], ns[0] + acc)
                    || recurse(tv, &ns[1..], ns[0] * acc)
                    || recurse(tv, &ns[1..], cat(acc, ns[0]))
            }
        }

        if recurse(tv, &ns[1..], ns[0]) {
            ans += tv;
        }
    }

    println!("{}", ans);

    Ok(())
}
