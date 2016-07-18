//use std::io;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
    // load 5, 5, 1 into memory.
    // add 5 and 1 to get 6
    // multiply 5 and 6 to get 30.
    // 21 5 21 5 21 1 1 2
    //let c = vec![21, 5, 21, 5, 21, 1, 1, 2, 0];

    // stores a variable and calls it from memory
    // pushn 4 pushn 5 ! pushn 5 @ print
    //let c = vec![21, 4, 21, 5, 8, 21, 5, 9, 0];
    
    // : function pushn 5 + ; pushn 3 pushn function call print
    //let c = vec![16, 200, 21, 5, 1, 17, 21, 3, 21, 200, 19, 0];

    // 2 to the power of 10
    // : power dup pushn 0 == if drop else >@ pushn 2 * @> pushn 1 - pushn function call then ; pushn 2 pushn 10 pushn power call print
    //let c = vec![16, 200, 10, 21, 0, 28, 25, 31, 26, 6, 21, 2, 2, 7, 21, 1, 3, 21, 18, 19, 27, 17, 21, 2, 21, 10, 21, 200, 19, 0];
    let path = Path::new("compiled.bytes");
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("couldn't open {}: {}", path.display(), err.description()),
    };
    let mut buffer = [32u8; 50_000];
    match file.read(&mut buffer) {
        Ok(_) => 0,
        Err(err) => panic!("couldn't read {}: {}", path.display(), err.description()),
    };
        
    forth(buffer.to_vec());
}
fn flip(x: Vec<u8>) -> Vec<u8> {
    return flip2(x, vec![]);
}
fn flip2(mut i: Vec<u8>, mut o: Vec<u8>) -> Vec<u8> {
    loop {
        if i.len() == 0 {
            break;
        }
        o.push(i.pop().unwrap());
    }
    return o;
}
fn forth(c: Vec<u8>) -> Vec<u32> {
    let c = flip(c);
    let s: Vec<u32> = Vec::new();
    let a: Vec<u32> = Vec::new();
    let v = Vec::new();
    let f = Vec::new();
    return forth2(c, s, a, v, f);
}
fn forth2(mut c: Vec<u8>, s: Vec<u32>, a: Vec<u32>, v: Vec<(u32, u32)>, f: Vec<(u8, Vec<u8>)>) -> Vec<u32> {
    if c.len() == 0 {
        return s;
    }
    let d = c.pop().unwrap();
    if d == 32u8 {
        return s;
    }
    let (c, s, a, v, f) = word(d.clone(), c.clone(), s.clone(), a.clone(), v.clone(), f.clone());
    return forth2(c, s, a, v, f);
}
fn word(x: u8, c: Vec<u8>, s: Vec<u32>, a: Vec<u32>, v: Vec<(u32, u32)>, f: Vec<(u8, Vec<u8>)>) -> (Vec<u8>, Vec<u32>, Vec<u32>, Vec<(u32, u32)>, Vec<(u8, Vec<u8>)>) {
    //println!("word {}", x);
    if (x%2) == 0u8 {
        return even_word(x, c, s, a, v, f);
    }
    return odd_word(x, c, s, a, v, f);
}
fn even_word(x: u8, mut c: Vec<u8>, mut s: Vec<u32>, mut a: Vec<u32>, mut v: Vec<(u32, u32)>, mut f: Vec<(u8, Vec<u8>)>) -> (Vec<u8>, Vec<u32>, Vec<u32>, Vec<(u32, u32)>, Vec<(u8, Vec<u8>)>) {
    if x == 0u8 { //print
        let mut sc = s.clone();
        if sc.len() > 0 {
            let z = sc.pop().unwrap();
            println!("{}", z);
        }
        if sc.len() > 1 {
            let z = sc.pop().unwrap();
            println!("{}", z);
        }
        if sc.len() > 2 {
            let z = sc.pop().unwrap();
            println!("{}", z);
        }
        return (c, s, a, v, f);
    }
    if x == 2u8 { // *
        let b = s.pop().unwrap();
        let d = s.pop().unwrap();
        s.push(b*d);
        return (c, s, a, v, f);
    }
    if x == 4u8 { // /
        let b = s.pop().unwrap();
        let d = s.pop().unwrap();
        s.push(b/d);
        return (c, s, a, v, f);
    }
    if x == 6u8 { // >@
        let b = s.pop().unwrap();
        a.push(b);
        return (c, s, a, v, f);
    }
    if x == 8u8 { // !
        let name = s.pop().unwrap();
        let value = s.pop().unwrap();
        v.push((name, value));
        return (c, s, a, v, f);
    }
    if x == 10u8 { //dup
        let ab = s.pop().unwrap();
        s.push(ab);
        s.push(ab);
        return (c, s, a, v, f);
    }
    if x == 12u8 { //rot
        let xa = s.pop().unwrap();
        let y = s.pop().unwrap();
        let z = s.pop().unwrap();
        s.push(y);
        s.push(z);
        s.push(xa);
        return (c, s, a, v, f);
    }
    if x == 14u8 { // 2dup
        let x = s.pop().unwrap();
        let y = s.pop().unwrap();
        s.push(y);
        s.push(x);
        s.push(y);
        s.push(x);
        return (c, s, a, v, f);
    }
    if x == 16u8 { // :
        let name = c.pop().unwrap();
        let (new_func, c) = split(c, 17u8);
        let new_code = replace(18u8, name, new_func);
        f.push((name, new_code));
        return (c, s, a, v, f);
    }
    if x == 20u8 { // push
        let mut b = s.pop().unwrap();
        loop {
            if b < 1u32 {
                break;
            }
            let d = c.pop().unwrap();
            s.push(d as u32);
            b -=1;
        }
        return (c, s, a, v, f);
    }
    if x == 22u8 { //push1
        let z = c.pop().unwrap() as u32;
        let b = c.pop().unwrap() as u32;
        let d = c.pop().unwrap() as u32;
        let e = c.pop().unwrap() as u32;
        let y = (((((e * 256) + d) * 256) + b) * 256) + z;
        s.push(y);
        return (c, s, a, v, f);
    }
    if x == 24u8 { //push3
        let z = c.pop().unwrap() as u32;
        let b = c.pop().unwrap() as u32;
        let e = c.pop().unwrap() as u32;
        let d = c.pop().unwrap() as u32;
        let y = (((((d * 256) + e) * 256) + b) * 256) + z;
        s.push(y);
        let z = c.pop().unwrap() as u32;
        let b = c.pop().unwrap() as u32;
        let e = c.pop().unwrap() as u32;
        let d = c.pop().unwrap() as u32;
        let y = (((((d * 256) + e) * 256) + b) * 256) + z;
        s.push(y);
        let z = c.pop().unwrap() as u32;
        let b = c.pop().unwrap() as u32;
        let e = c.pop().unwrap() as u32;
        let d = c.pop().unwrap() as u32;
        let y = (((((d * 256) + e) * 256) + b) * 256) + z;
        s.push(y);
        return (c, s, a, v, f);
    }
    if x == 26u8 { // else
        let (_, c) = split(c, 27);
        return (c, s, a, v, f);
    }
    if x == 28u8 { // ==
        let y = s.pop().unwrap();
        let z = s.pop().unwrap();
        if z == y {
            s.push(1u32);
            return (c, s, a, v, f);
        }
        s.push(0u32);
        return (c, s, a, v, f);
    }
    if x == 30u8 { // <
        let y = s.pop().unwrap();
        let z = s.pop().unwrap();
        if z < y {
            s.push(1u32);
            return (c, s, a, v, f);
        }
        s.push(0u32);
        return (c, s, a, v, f);
    }
    if x == 32u8 { // finish
        return (c, s, a, v, f);
    }
    return (c, s, a, v, f);
}    
fn odd_word(x: u8, mut c: Vec<u8>, mut s: Vec<u32>, mut a: Vec<u32>, v: Vec<(u32, u32)>, f: Vec<(u8, Vec<u8>)>) -> (Vec<u8>, Vec<u32>, Vec<u32>, Vec<(u32, u32)>, Vec<(u8, Vec<u8>)>) {
    if x == 1u8 { // +
        let b = s.pop().unwrap();
        let d = s.pop().unwrap();
        s.push(b+d);
        return (c, s, a, v, f);
    }
    if x == 3u8 { // -
        let b = s.pop().unwrap();
        let d = s.pop().unwrap();
        s.push(d-b);
        return (c, s, a, v, f);
    }
    if x == 5u8 { // %
        let b = s.pop().unwrap();
        let d = s.pop().unwrap();
        s.push( b % d );
        return (c, s, a, v, f);
    }
    if x == 7u8 { // @>
        let b = a.pop().unwrap();
        s.push(b);
        return (c, s, a, v, f);
    }
    if x == 9u8 { // @
        let name = s.pop().unwrap();
        let value = get_value(v.clone(), name);
        s.push(value);
        return (c, s, a, v, f);
    }
    if x == 11u8 { // swap
        let y = s.pop().unwrap();
        let u = s.pop().unwrap();
        s.push(y);
        s.push(u);
        return (c, s, a, v, f);
    }
    if x == 13u8 { // tuck
        let x = s.pop().unwrap();
        let y = s.pop().unwrap();
        let z = s.pop().unwrap();
        s.push(z);
        s.push(x);
        s.push(y);
        return (c, s, a, v, f);
    }
    if x == 15u8 { // 2swap
        let x = s.pop().unwrap();
        let y = s.pop().unwrap();
        let x2 = s.pop().unwrap();
        let y2 = s.pop().unwrap();
        s.push(y);
        s.push(x);
        s.push(y2);
        s.push(x2);
        return (c, s, a, v, f);
    }
    if x == 19u8 { // call
        let name = s.pop().unwrap();
        let value = get_function(f.clone(), name as u8);
        let c = vec_append(value, c);
        return (c, s, a, v, f);
    }
    if x == 21u8 { //push1
        let y = c.pop().unwrap();
        s.push(y as u32);
        return (c, s, a, v, f);
    }
    if x == 23u8 { //push2
        let z = c.pop().unwrap() as u32;
        let b = c.pop().unwrap() as u32;
        let e = c.pop().unwrap() as u32;
        let d = c.pop().unwrap() as u32;
        let y = (((((d * 256) + e) * 256) + b) * 256) + z;
        s.push(y);
        let z = c.pop().unwrap() as u32;
        let b = c.pop().unwrap() as u32;
        let e = c.pop().unwrap() as u32;
        let d = c.pop().unwrap() as u32;
        let y = (((((d * 256) + e) * 256) + b) * 256) + z;
        s.push(y);
        return (c, s, a, v, f);
    }
    if x == 25u8 { //if
        let y = s.pop().unwrap();
        if y == 0 {
            let (_, c) = split(c, 26);
            return (c, s, a, v, f);
        }
        return (c, s, a, v, f);
    }
    if x == 27u8 { //else
        return (c, s, a, v, f);
    }
    if x == 29u8 { // >
        let y = s.pop().unwrap();
        let z = s.pop().unwrap();
        if z > y {
            s.push(1u32);
            return (c, s, a, v, f);
        }
        s.push(0u32);
        return (c, s, a, v, f);
    }            
    if x == 31u8 { // drop
        s.pop();
        return (c, s, a, v, f);
    }
    return (c, s, a, v, f);
}    
fn get_value(mut v: Vec<(u32, u32)>, n: u32) -> u32 {
    loop {
        if v.len() == 0 {
            return 0;
        }
        let (name, value) = v.pop().unwrap();
        if name == n {
            return value;
        }
    }
}
fn get_function(mut f: Vec<(u8, Vec<u8>)>, n:u8) -> Vec<u8> {
    loop {
        if f.len() == 0 {
            return vec![];
        }
        let (name, func) = f.pop().unwrap();
        if name == n {
            return func;
        }

    }
}
fn split(c: Vec<u8>, s: u8) -> (Vec<u8>, Vec<u8>) {
    return split2(c, vec![], s);
}
fn split2(mut c: Vec<u8>, mut d: Vec<u8>, s: u8) -> (Vec<u8>, Vec<u8>) {
    loop {
    let a = c.pop().unwrap();
        if a == s {
            break;
        }
        d.push(a);
    }
    return (d, c);
}
fn replace(old: u8, new: u8, mut v: Vec<u8>) -> Vec<u8> {
    let mut u = vec![];
    loop {
        if v.len() == 0 {
            break;
        }
        let a = v.pop().unwrap();
        if a == old {
            u.push(new);
            continue;
        }
        u.push(a);
    }
    return flip(u);
}
fn vec_append(top: Vec<u8>, bottom: Vec<u8>) -> Vec<u8> {
    return vec_append2(top, bottom);
}
fn vec_append2(mut t: Vec<u8>, mut bottom: Vec<u8>) -> Vec<u8> {
    loop {
        if t.len() == 0 {
            break;
        }
        let a = t.pop().unwrap();
        bottom.push(a);
    }
    return bottom;
}
