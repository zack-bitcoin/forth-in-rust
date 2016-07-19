//use std::io;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new("code.fs");
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("couldn't open {}: {}", path.display(), err.description()),
    };
    let mut buffer = [0u8; 50_000];
    match file.read(&mut buffer) {
        Ok(_) => 0,
        Err(err) => panic!("couldn't read {}: {}", path.display(), err.description()),
    };
    println!("buffer 0 {}", buffer[0]);
    println!("buffer 1 {}", buffer[1]);
    println!("buffer 2 {}", buffer[2]);
    println!("buffer 3 {}", buffer[3]);
    println!("buffer 4 {}", buffer[4]);

    let a = vec![1u32,2u32,3u32,4u32,5u32];
    let b = to_vecu8(a.clone());
    let c = to_vecu32(b);
    println!("2 converts {} {}", c[0], a[0]);
    println!("2 converts {} {}", c[1], a[1]);
}

fn to_vecu32(mut x: Vec<u8>) -> Vec<u32> {
    let mut y = vec![];
    loop {
        if x.len() == 0 {
            return y;
        }
        let a = x.pop().unwrap() as u32;
        let b = x.pop().unwrap() as u32;
        let c = x.pop().unwrap() as u32;
        let d = x.pop().unwrap() as u32;
        let n = (((((d * 256) + c) * 256) + b) * 256) + a;
        y.push(n);
    }
}
fn to_vecu8(mut x: Vec<u32>) -> Vec<u8> {
    let mut y = vec![];
    loop {
        if x.len() == 0 {
            return y;
        }
        let n = x.pop().unwrap();
        let a = (n % 256) as u8;
        let b = (n % (256 * 256) / 256) as u8;
        let c = (n % (256 * 256 * 256) /256 / 256) as u8;
        let d = (n / 256 / 256 / 256) as u8;
        y.push(d);
        y.push(c);
        y.push(b);
        y.push(a);
    }
}
