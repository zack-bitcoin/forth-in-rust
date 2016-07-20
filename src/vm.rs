//use std::io;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 1 {
        println!("filename not given");
    }
    //println!("{}", args[1]);
    //let path = Path::new("compiled.bytes");
    let x = args[1].clone();
    let path = Path::new(&x);
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
fn forth(code: Vec<u8>) -> Vec<u32> {
    let c = flip(code);
    let stack: Vec<u32> = Vec::new();
    let alt_stack: Vec<u32> = Vec::new();
    let variables = Vec::new();
    let functions = Vec::new();
    return forth2(c, stack, alt_stack, variables, functions);
}
fn forth2(mut code: Vec<u8>, mut stack: Vec<u32>, mut a: Vec<u32>, mut v: Vec<(u32, u32)>, mut f: Vec<(u8, Vec<u8>)>) -> Vec<u32> {
    loop {
        if code.len() == 0 {
            return stack;
        }
        let d = code.pop().unwrap();
        if d == 32u8 {
            return stack;
        }
        let y = word(d, code, stack, a, v, f);
        let (c0, s0, a0, v0, f0) = y;
        code = c0;
        stack = s0;
        a = a0;
        v = v0;
        f = f0;
    }
}
fn word(x: u8, c: Vec<u8>, s: Vec<u32>, a: Vec<u32>, v: Vec<(u32, u32)>, f: Vec<(u8, Vec<u8>)>) -> (Vec<u8>, Vec<u32>, Vec<u32>, Vec<(u32, u32)>, Vec<(u8, Vec<u8>)>) {
    //println!("word {}", x);
    if (x%2) == 0u8 {
        if (x%4) == 0u8 {
            return even_even_word(x, c, s, a, v, f);
        }
        return even_odd_word(x, c, s, a, v, f);
    }
    if (x%4) == 1u8 {
        return odd_even_word(x, c, s, a, v, f);
    }
    return odd_odd_word(x, c, s, a, v, f);
}
fn even_even_word(x: u8, mut c: Vec<u8>, mut s: Vec<u32>, a: Vec<u32>, mut v: Vec<(u32, u32)>, mut f: Vec<(u8, Vec<u8>)>) -> (Vec<u8>, Vec<u32>, Vec<u32>, Vec<(u32, u32)>, Vec<(u8, Vec<u8>)>) {
    if x == 0u8 { //print
        if s.len() > 0 {
            println!("{}", s[s.len()-1]);
        }
        if s.len() > 1 {
            println!("{}", s[s.len()-2]);
        }
        if s.len() > 2 {
            println!("{}", s[s.len()-3]);
        }
        return (c, s, a, v, f);
    }
    if x == 4u8 { // / (division)
        let b = s.pop().unwrap();
        let d = s.pop().unwrap();
        s.push(b/d);
        return (c, s, a, v, f);
    }
    if x == 8u8 { // ! (store value in variable)
        let name = s.pop().unwrap();
        let value = s.pop().unwrap();
        v.push((name, value));
        return (c, s, a, v, f);
    }
    if x == 12u8 { //rot
        let x = s.pop().unwrap();
        let y = s.pop().unwrap();
        let z = s.pop().unwrap();
        s.push(y);
        s.push(z);
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
    if x == 32u8 { // finish
        return (c, s, a, v, f);
    }
    return (c, s, a, v, f);
}
fn even_odd_word(x: u8, mut c: Vec<u8>, mut s: Vec<u32>, mut a: Vec<u32>, v: Vec<(u32, u32)>, f: Vec<(u8, Vec<u8>)>) -> (Vec<u8>, Vec<u32>, Vec<u32>, Vec<(u32, u32)>, Vec<(u8, Vec<u8>)>) {
    if x == 2u8 { // *
        let b = s.pop().unwrap();
        let d = s.pop().unwrap();
        s.push(b*d);
        return (c, s, a, v, f);
    }
    if x == 6u8 { // >r
        let b = s.pop().unwrap();
        a.push(b);
        return (c, s, a, v, f);
    }
    if x == 10u8 { //dup
        let ab = s.pop().unwrap();
        s.push(ab);
        s.push(ab);
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
    if x == 22u8 { //push1
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
    if x == 34u8 { // or
        let z = s.pop().unwrap();
        let y = s.pop().unwrap();
        if (z != 0u32) || (y != 0u32) {
            s.push(1);
            return (c, s, a, v, f);
        }
        s.push( 0 );
        return (c, s, a, v, f);
    }        
    return (c, s, a, v, f);
}    
fn odd_even_word(x: u8, mut c: Vec<u8>, mut s: Vec<u32>, mut a: Vec<u32>, v: Vec<(u32, u32)>, f: Vec<(u8, Vec<u8>)>) -> (Vec<u8>, Vec<u32>, Vec<u32>, Vec<(u32, u32)>, Vec<(u8, Vec<u8>)>) {
    if x == 1u8 { // +
        let b = s.pop().unwrap();
        let d = s.pop().unwrap();
        s.push(b+d);
        return (c, s, a, v, f);
    }
    if x == 5u8 { // %
        let b = s.pop().unwrap();
        let d = s.pop().unwrap();
        s.push( d % b );
        return (c, s, a, v, f);
    }
    if x == 9u8 { // @
        let name = s.pop().unwrap();
        let (value, v) = get_value(v, name);
        s.push(value);
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
    if x == 21u8 { //pushn
        let y = c.pop().unwrap();
        s.push(y as u32);
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
    if x == 33u8 { // r@
        let z = a.pop().unwrap();
        s.push(z);
        a.push(z);
        return (c, s, a, v, f);
    }
    return (c, s, a, v, f);
}
fn odd_odd_word(x: u8, mut c: Vec<u8>, mut s: Vec<u32>, mut a: Vec<u32>, v: Vec<(u32, u32)>, f: Vec<(u8, Vec<u8>)>) -> (Vec<u8>, Vec<u32>, Vec<u32>, Vec<(u32, u32)>, Vec<(u8, Vec<u8>)>) {
    if x == 3u8 { // -
        let b = s.pop().unwrap();
        let d = s.pop().unwrap();
        s.push(d-b);
        return (c, s, a, v, f);
    }
    if x == 7u8 { // r>
        let b = a.pop().unwrap();
        s.push(b);
        return (c, s, a, v, f);
    }
    if x == 11u8 { // swap
        let y = s.pop().unwrap();
        let u = s.pop().unwrap();
        s.push(y);
        s.push(u);
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
        let (f, value) = get_function(f, name as u8);
        let c = vec_append(value, c);
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
    if x == 27u8 { //then
        return (c, s, a, v, f);
    }
    if x == 31u8 { // drop
        s.pop();
        return (c, s, a, v, f);
    }
    if x == 35u8 { // and
        let z = s.pop().unwrap();
        let y = s.pop().unwrap();
        if (z != 0u32) && (y != 0u32) {
            s.push(1);
            return (c, s, a, v, f);
        }
        s.push( 0 );
        return (c, s, a, v, f);
    }
    return (c, s, a, v, f);
}    
fn get_value(v: Vec<(u32, u32)>, n: u32) -> (u32, Vec<(u32, u32)>) {
    let mut c = 0;
    loop {
        if v.len() == c {
            return (0, v);
        }
        let (name, value) = v[c];
        if name == n {
            return (value, v);
        }
        c+=1;
    }
}
fn get_function(f: Vec<(u8, Vec<u8>)>, n:u8) -> (Vec<(u8, Vec<u8>)>, Vec<u8>) {
    let mut c = 0;
    loop {
        if f.len() == c {
            return (f, vec![]);
        }
        let (name, func) = f[c].clone();
        if name == n {
            return (f, func);
        }
        c+=1;
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
