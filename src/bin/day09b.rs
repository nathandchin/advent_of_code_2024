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
    let mut idx = blocks.len() - 1;
    while idx > 0 {
        // Skip empty blocks
        if blocks[idx].filled == 0 {
            idx -= 1;
            continue;
        }

        let rblock = blocks[idx].clone();
        let mut lidx = 0;
        while lidx < idx {
            if blocks[lidx].empty >= rblock.filled {
                let new_block = {
                    // Move rblock into lblock
                    let lblock = blocks.get_mut(lidx).unwrap();
                    lblock.filled += rblock.filled;
                    lblock.empty -= rblock.filled;
                    lblock.id = rblock.id;

                    // Cleave in twain
                    if lblock.empty > 0 {
                        let l_empty = lblock.empty;
                        lblock.empty = 0;
                        lidx += 1;
                        idx += 1;
                        Some(Block {
                            id: None,
                            filled: 0,
                            empty: l_empty,
                        })
                    } else {
                        None
                    }
                };

                if let Some(new_block) = new_block {
                    blocks.insert(lidx, new_block);
                }

                // Empty the src
                blocks[idx].id = None;
                blocks[idx].filled = 0;
                blocks[idx].empty = rblock.filled;

                idx -= 1;
                break;
            }
            lidx += 1;
        }

        idx -= 1;
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

        // Don't skip empty blocks!
        pos += block.empty;
    }

    println!("{}", ans);

    Ok(())
}
