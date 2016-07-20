//use std::io;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::env;
use std::process::exit;

macro_rules! die {
    ($($tok:tt)+) => {{
        let stderr = io::stderr();
        let mut stderr = stderr.lock();
        let _ = writeln!(stderr, $($tok)+);
        exit(1)
    }}
}

fn main() {
    let mut args = env::args_os().skip(1);
    let path = match args.next() {
        Some(p) => p,
        None => die!("no filename specified"),
    };
    let remaining_args = args.count();
    if remaining_args > 0 {
        die!("expected one argument, found {}", remaining_args);
    }
    let mut file = File::open(&path).unwrap_or_else(|err| die!("{}", err));
    let mut buffer = Vec::with_capacity(file.metadata().map(|m|m.len()).unwrap_or(0) as usize);
    file.read_to_end(&mut buffer).unwrap_or_else(|err| die!("{}", err));
    forth(buffer);
}

fn forth(mut code: Vec<u8>) -> Vec<u32> {
    // In-place reversal
    code.reverse();

    // Pre-allocate some space. Keeps short programs with small stacks from
    // spending time up front repeatedly re-allocating the stacks.
    let mut stack: Vec<u32> = Vec::with_capacity(32);
    let mut alt_stack: Vec<u32> = Vec::with_capacity(32);

    // Could also use a HashMap but BTreeMaps tend to be faster smaller tables.
    // If we had more guarantees about where variables could be written,
    // variable lookup could be significantly faster.
    let mut variables: BTreeMap<u32, u32> = BTreeMap::new();

    // To avoid allocating every time we define a function, store them all in
    // the same Vec, terminate them with a 17 byte (can't appear in function
    // definitions), and put a pointer to them in function_table.

    // We could put this on the stack but it's a bit large and a single large
    // allocation isn't too expensive.
    let mut function_table: Vec<usize> = vec![0; 256];
    let mut function_code: Vec<u8> = Vec::new();

    // NOTE: this could be significantly faster if we loosened the stack
    // abstraction a bit but forth really is all about stacks.

    while let Some(op) = code.pop() {
        match op {
            //print
            0u8 => {
                // Take the top three items from the stack.
                for &b in stack.iter().rev().take(3) {
                    println!("{}", b);
                }
            }
            // +
            1u8 => {
                let b = stack.pop().unwrap();
                let d = stack.pop().unwrap();
                stack.push(b+d);
            }
            // *
            2u8 => {
                let b = stack.pop().unwrap();
                let d = stack.pop().unwrap();
                stack.push(b*d);
            },
            // -
            3u8 => {
                let b = stack.pop().unwrap();
                let d = stack.pop().unwrap();
                stack.push(d-b);
            }
            4u8 => {
                let b = stack.pop().unwrap();
                let d = stack.pop().unwrap();
                stack.push(b/d);
            }
            // %
            5u8 => {
                let b = stack.pop().unwrap();
                let d = stack.pop().unwrap();
                stack.push( d % b );
            }
            // >r
            6u8 => {
                let b = stack.pop().unwrap();
                alt_stack.push(b);
            }
            // r>
            7u8 => {
                let b = alt_stack.pop().unwrap();
                stack.push(b);
            }
            // ! (store value in variable)
            8u8 => {
                let name = stack.pop().unwrap();
                let value = stack.pop().unwrap();
                variables.insert(name, value);
            }
            // @ (get)
            9u8 => {
                let name = stack.pop().unwrap();
                let value = *variables.get(&name).unwrap_or(&0);
                stack.push(value);
            }
            //dup
            10u8 => {
                let ab = stack.pop().unwrap();
                stack.push(ab);
                stack.push(ab);
            }
            // swap
            11u8 => {
                let y = stack.pop().unwrap();
                let u = stack.pop().unwrap();
                stack.push(y);
                stack.push(u);
            }
            //rot
            12u8 => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                let z = stack.pop().unwrap();
                stack.push(y);
                stack.push(z);
                stack.push(x);
            },
            // tuck
            13u8 => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                let z = stack.pop().unwrap();
                stack.push(z);
                stack.push(x);
                stack.push(y);
            },
            // 2dup
            14u8 => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(y);
                stack.push(x);
                stack.push(y);
                stack.push(x);
            }
            // 2swap
            15u8 => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                let x2 = stack.pop().unwrap();
                let y2 = stack.pop().unwrap();
                stack.push(y);
                stack.push(x);
                stack.push(y2);
                stack.push(x2);
            }
            // : (define function)
            16u8 => {
                let name = code.pop().unwrap();
                function_code.push(17u8);
                loop {
                    match code.pop().expect("unterminated function") {
                        17u8 => break,
                        18u8 => function_code.push(name),
                        op => function_code.push(op),
                    }
                }
                function_table[name as usize] = function_code.len();
            }
            17u8|18u8 => unreachable!(),
            // call
            19u8 => {
                let name = stack.pop().unwrap();
                let function_start = function_table[name as usize];
                assert!(function_start != 0, "attempted to call undefined function");
                for &byte in function_code[..function_start].iter().rev() {
                    match byte {
                        17u8 => break,
                        _ => code.push(byte),
                    }
                }
            }
            // push
            20u8 => {
                let b = stack.pop().unwrap();
                for _ in 0..b {
                    let d = code.pop().unwrap();
                    stack.push(d as u32);
                }
            }
            //pushn
            21u8 => {
                let y = code.pop().unwrap();
                stack.push(y as u32);
            }
            //push1..3
            22u8|23u8|24u8 => {
                let count = op - 21;
                for _ in 0..count {
                    let z = code.pop().unwrap() as u32;
                    let b = code.pop().unwrap() as u32;
                    let e = code.pop().unwrap() as u32;
                    let d = code.pop().unwrap() as u32;
                    let y = (d << 24) | (e << 16) | (b << 8) | z;
                    stack.push(y);
                }
            }
            // if
            25u8 => {
                let y = stack.pop().unwrap();
                if y == 0 {
                    // skip to else.
                    while code.pop().unwrap() != 26 { }
                }
            }
            // skip over else
            26u8 => while code.pop().unwrap() != 27 { },
            // endif
            27u8 => {},
            // ==
            28u8 => {
                let y = stack.pop().unwrap();
                let z = stack.pop().unwrap();
                stack.push((z == y) as u32);
            }
            // >
            29u8 => {
                let y = stack.pop().unwrap();
                let z = stack.pop().unwrap();
                stack.push((z > y) as u32);
            }
            // <
            30u8 => {
                let y = stack.pop().unwrap();
                let z = stack.pop().unwrap();
                stack.push((z < y) as u32);
            }
            // drop
            31u8 => {
                stack.pop().unwrap();
            }
            // Stop
            32u8 => break,
            // r@
            33u8 => {
                let z = alt_stack.pop().unwrap();
                stack.push(z);
                alt_stack.push(z);
            }
            // or
            34u8 => {
                let z = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push((z != 0 || y != 0) as u32)
            }
            // and
            35u8 => {
                let z = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push((z != 0u32 && y != 0u32) as u32);
            }
            _ => panic!("unknown op code {}", op),
        }
    }
    stack
}
