use std::io::{stdin, Read};

use color_eyre::Result;

#[derive(Clone, Debug)]
struct Block {
    id: Option<u32>,
    filled: u32,
    empty: u32,
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    // Parse
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

    // Iterate over chunks, left to right
    let mut idx = 0;
    while idx < blocks.len() {
        if blocks[idx].empty == 0 {
            idx += 1;
            continue;
        }

        // Discard empty blocks at end
        while blocks.last().unwrap().filled == 0 {
            blocks.pop();
        }

        // Finished
        if idx == blocks.len() - 1 {
            break;
        }

        loop {
            // Filled up the current empty slot, move on
            if blocks[idx].empty == 0 {
                idx += 1;
                break;
            }

            // Rightmost file depleted, move inward
            if blocks.last().unwrap().filled == 0 {
                break;
            }

            // Update left block
            {
                // Get the data we need from the left block
                let id = blocks.last().unwrap().id;
                let (l_id, l_empty) = {
                    let l = &blocks[idx];
                    (l.id, l.empty)
                };

                // Cleave in twain - we're putting contents from two different
                // files in the place where there was one block, so it needs to
                // be split to hold both files' contents. Use vec::insert() to
                // do this. Expensive but *shrug*.
                if l_id.is_some() && l_id != id {
                    let new = Block {
                        id,
                        filled: 0,
                        empty: l_empty,
                    };
                    blocks[idx].empty = 0;
                    idx += 1;
                    blocks.insert(idx, new);
                }

                // Update left block. Whether it is unchanged or it is newly
                // created from a split makes no difference.
                let l = blocks.get_mut(idx).unwrap();
                l.id = id;
                l.empty -= 1;
                l.filled += 1;
            }

            // Update right block
            {
                let blocks_len = blocks.len();
                let r = blocks.get_mut(blocks_len - 1).unwrap();
                r.empty += 1;
                r.filled -= 1;
            }
        }
    }

    // Checksum
    let mut pos = 0;
    let mut ans: u128 = 0;
    for block in blocks {
        // Naively iterate. The real savings were in avoiding expanding all the
        // files above. This checksum calculation isn't fast but it doesn't take
        // too long for the puzzle inputs and it wastes no extra memory.
        for _ in 0..block.filled {
            ans += (pos * block.id.unwrap()) as u128;
            pos += 1;
        }
    }

    println!("{}", ans);

    Ok(())
}
