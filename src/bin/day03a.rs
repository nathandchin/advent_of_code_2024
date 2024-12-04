use std::io::{stdin, Read};

use color_eyre::Result;
use regex::Regex;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mul_regex = Regex::new(r#"mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)"#)?;
    let mut ans = 0;

    for cap in mul_regex.captures_iter(&input) {
        ans += cap.name("x").unwrap().as_str().parse::<i32>()?
            * cap.name("y").unwrap().as_str().parse::<i32>()?;
    }

    println!("{}", ans);

    Ok(())
}
