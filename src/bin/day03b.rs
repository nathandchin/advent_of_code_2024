use std::io::{stdin, Read};

use color_eyre::Result;
use regex::Regex;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mul_regex =
        Regex::new(r#"(?<do>do\(\))|(?<dont>don't\(\))|mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)"#)?;
    let mut ans = 0;
    let mut muls_enabled = true;

    for cap in mul_regex.captures_iter(&input) {
        if cap.name("do").is_some() {
            muls_enabled = true;
        } else if cap.name("dont").is_some() {
            muls_enabled = false;
        } else {
            if muls_enabled {
                ans += cap.name("x").unwrap().as_str().parse::<i32>()?
                    * cap.name("y").unwrap().as_str().parse::<i32>()?;
            }
        }
    }

    println!("{}", ans);

    Ok(())
}
