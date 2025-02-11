use std::{
    fmt::Display,
    io::{stdin, Read},
};

use color_eyre::Result;

#[derive(Clone, Debug)]
struct Block {
    id: Option<u32>,
    filled: u32,
    empty: u32,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::with_capacity((self.filled + self.empty) as usize);
        if let Some(id) = self.id {
            let ch = char::from_digit(id, 10).unwrap();
            for _ in 0..self.filled {
                s.push(ch);
            }
        }

        for _ in 0..self.empty {
            s.push('.');
        }
        write!(f, "{}", s)
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut on_file = true;
    let mut id: u32 = 0;
    let mut blocks = vec![];
    for ch in input.chars() {
        if on_file {
            blocks.push(Block {
                id: Some(id),
                filled: ch.to_digit(10).unwrap(),
                empty: 0,
            });
            id += 1;
        } else {
            blocks.push(Block {
                id: None,
                filled: 0,
                empty: ch.to_digit(10).unwrap(),
            });
        }
        on_file = !on_file;
    }

    fn pprint(blocks: &Vec<Block>) {
        for block in blocks {
            eprint!("{}", block);
        }
        eprintln!();
    }

    // pprint(&blocks);

    let mut idx = 0;
    while idx < blocks.len() {
        if blocks[idx].empty == 0 {
            idx += 1;
            continue;
        }

        // eprintln!("here1");

        // Discard empty blocks at end
        while blocks.last().unwrap().filled == 0 {
            eprintln!("Popping at idx={}", blocks.len() - 1);
            // pprint(&blocks);
            blocks.pop();
        }

        // eprintln!("here2");

        if idx == blocks.len() - 1 {
            break;
        }

        loop {
            // dbg!(blocks[idx].id);
            // dbg!(idx);
            // dbg!(blocks.len());
            // eprintln!("here3");
            if blocks[idx].empty == 0 {
                idx += 1;
                break;
            }
            // eprintln!("here4");
            if blocks.last().unwrap().filled == 0 {
                break;
            }
            // eprintln!("here5");

            {
                let id = blocks.last().unwrap().id;
                let (l_id, l_empty) = {
                    let l = &blocks[idx];
                    (l.id, l.empty)
                };

                // Cleave in twain
                if l_id.is_some() && l_id != id {
                    eprintln!("Cleaving at idx={}, id={}", idx, l_id.unwrap());

                    let new = Block {
                        id,
                        filled: 0,
                        empty: l_empty,
                    };
                    blocks[idx].empty = 0;
                    idx += 1;
                    blocks.insert(idx, new);
                }
                let l = blocks.get_mut(idx).unwrap();

                l.id = id;
                l.empty -= 1;
                l.filled += 1;
            }
            // eprintln!("here6");
            {
                let blocks_len = blocks.len();
                let r = blocks.get_mut(blocks_len - 1).unwrap();
                r.empty += 1;
                r.filled -= 1;
            }
            // eprintln!("here7");

            // pprint(&blocks);
        }
    }

    // Checksum
    let mut pos = 0;
    let mut ans: u128 = 0;
    for block in blocks {
        for _ in 0..block.filled {
            ans += (pos * block.id.unwrap()) as u128;
            pos += 1;
        }
    }

    println!("{}", ans);

    Ok(())
}
