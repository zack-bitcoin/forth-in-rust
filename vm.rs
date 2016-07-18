fn main() {
    //let c = vec![3u32, 0u32, 5u32, 5u32, 1u32, 1u32, 2u32];
    // load 5, 5, 1 into memory.
    // add 5 and 1 to get 6
    // multiply 5 and 6 to get 30.

    // push 2 4 5 ! 5 @ print
    // 0 2 4 5 8 5 9 20
    //let c = vec![20u32, 2u32, 4u32, 5u32, 8u32, 20u32, 1u32, 5u32, 9u32, 20u32];
    
    // : function 5 + ; 3 function call print
    // 16 300 21 5 1 17 22 3 300 19 20
    //let c = vec![16u32, 300u32, 21u32, 5u32, 1u32, 17u32, 22u32, 3u32, 300u32, 19u32, 0u32];

    // : function dup push1 0 == if drop else >@ push1 2 * @> push1 1 - push1 function call then ; push3 2 10 function call print
    // 16 300 10 21 0 28 25 31 26 6 21 2 2 7 21 1 3 21 300 19 27 17 23 2 10 300 19 0
    let c = vec![16, 300, 10, 21, 0, 28, 25, 31, 26, 6, 21, 2, 2, 7, 21, 1, 3, 21, 300, 19, 27, 17, 23, 2, 10, 300, 19, 0];
    forth(c);
}

fn flip(x: Vec<u32>) -> Vec<u32> {
    return flip2(x, vec![]);
}
fn flip2(mut i: Vec<u32>, mut o: Vec<u32>) -> Vec<u32> {
    loop {
        if i.len() == 0 {
            break;
        }
        o.push(i.pop().unwrap());
    }
    return o;
}
    

fn forth(c: Vec<u32>) -> Vec<u32> {
    let c = flip(c);
    let s: Vec<u32> = Vec::new();
    let a: Vec<u32> = Vec::new();
    let v = Vec::new();
    let f = Vec::new();
    return forth2(c, s, a, v, f);
}

fn forth2(mut c: Vec<u32>, s: Vec<u32>, a: Vec<u32>, v: Vec<(u32, u32)>, f: Vec<(u32, Vec<u32>)>) -> Vec<u32> {
    if c.len() == 0 {
        return s;
    }
    let d = c.pop().unwrap();
    let (c, s, a, v, f) = word(d.clone(), c.clone(), s.clone(), a.clone(), v.clone(), f.clone());
    return forth2(c, s, a, v, f);
}

fn word(x: u32, c: Vec<u32>, s: Vec<u32>, a: Vec<u32>, v: Vec<(u32, u32)>, f: Vec<(u32, Vec<u32>)>) -> (Vec<u32>, Vec<u32>, Vec<u32>, Vec<(u32, u32)>, Vec<(u32, Vec<u32>)>) {
    //println!("word {}", x);
    if (x%2) == 0u32 {
        return even_word(x, c, s, a, v, f);
    }
    return odd_word(x, c, s, a, v, f);
}
fn even_word(x: u32, mut c: Vec<u32>, mut s: Vec<u32>, mut a: Vec<u32>, mut v: Vec<(u32, u32)>, mut f: Vec<(u32, Vec<u32>)>) -> (Vec<u32>, Vec<u32>, Vec<u32>, Vec<(u32, u32)>, Vec<(u32, Vec<u32>)>) {
    if x == 0u32 { //print
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
    if x == 2u32 { // *
        let b = s.pop().unwrap();
        let d = s.pop().unwrap();
        s.push(b*d);
        return (c, s, a, v, f);
    }
    if x == 4u32 { // /
        let b = s.pop().unwrap();
        let d = s.pop().unwrap();
        s.push(b/d);
        return (c, s, a, v, f);
    }
    if x == 6u32 { // >@
        let b = s.pop().unwrap();
        a.push(b);
        return (c, s, a, v, f);
    }
    if x == 8u32 { // !
        let name = s.pop().unwrap();
        let value = s.pop().unwrap();
        v.push((name, value)); //new
        //v.push(vec![name, value]);
        return (c, s, a, v, f);
    }
    if x == 10u32 { //dup
        let ab = s.pop().unwrap();
        s.push(ab);
        s.push(ab);
        return (c, s, a, v, f);
    }
    if x == 12u32 { //rot
        let xa = s.pop().unwrap();
        let y = s.pop().unwrap();
        let z = s.pop().unwrap();
        s.push(y);
        s.push(z);
        s.push(xa);
        return (c, s, a, v, f);
    }
    if x == 14u32 { // 2dup
        let x = s.pop().unwrap();
        let y = s.pop().unwrap();
        s.push(y);
        s.push(x);
        s.push(y);
        s.push(x);
        return (c, s, a, v, f);
    }
    if x == 16u32 { // :
        let name = c.pop().unwrap();
        let (new_func, c) = split(c, 17u32);
        let new_code = replace(18u32, name, new_func);
        f.push((name, new_code));
        return (c, s, a, v, f);
    }
    if x == 20u32 { // push
        let mut b = s.pop().unwrap();
        loop {
            if b < 1u32 {
                break;
            }
            let d = c.pop().unwrap();
            s.push(d);
            b -=1;
        }
        return (c, s, a, v, f);
    }
    if x == 22u32 { //push2
        let y = c.pop().unwrap();
        s.push(y);
        let y = c.pop().unwrap();
        s.push(y);
        return (c, s, a, v, f);
    }
    if x == 24u32 { //push2
        let y = c.pop().unwrap();
        s.push(y);
        let y = c.pop().unwrap();
        s.push(y);
        let y = c.pop().unwrap();
        s.push(y);
        let y = c.pop().unwrap();
        s.push(y);
        return (c, s, a, v, f);
    }
    if x == 26u32 { // else
        let (_, c) = split(c, 27);
        return (c, s, a, v, f);
    }
    if x == 28u32 { // ==
        let y = s.pop().unwrap();
        let z = s.pop().unwrap();
        if z == y {
            s.push(1u32);
            return (c, s, a, v, f);
        }
        s.push(0u32);
        return (c, s, a, v, f);
    }
    if x == 30u32 { // <
        let y = s.pop().unwrap();
        let z = s.pop().unwrap();
        if z < y {
            s.push(1u32);
            return (c, s, a, v, f);
        }
        s.push(0u32);
        return (c, s, a, v, f);
    }
    return (c, s, a, v, f);
}    
fn odd_word(x: u32, mut c: Vec<u32>, mut s: Vec<u32>, mut a: Vec<u32>, v: Vec<(u32, u32)>, f: Vec<(u32, Vec<u32>)>) -> (Vec<u32>, Vec<u32>, Vec<u32>, Vec<(u32, u32)>, Vec<(u32, Vec<u32>)>) {
    if x == 1u32 { // +
        let b = s.pop().unwrap();
        let d = s.pop().unwrap();
        s.push(b+d);
        return (c, s, a, v, f);
    }
    if x == 3u32 { // -
        let b = s.pop().unwrap();
        let d = s.pop().unwrap();
        s.push(d-b);
        return (c, s, a, v, f);
    }
    if x == 5u32 { // %
        let b = s.pop().unwrap();
        let d = s.pop().unwrap();
        s.push( b % d );
        return (c, s, a, v, f);
    }
    if x == 7u32 { // @>
        let b = a.pop().unwrap();
        s.push(b);
        return (c, s, a, v, f);
    }
    if x == 9u32 { // @
        let name = s.pop().unwrap();
        let value = get_value(v.clone(), name);
        s.push(value);
        return (c, s, a, v, f);
    }
    if x == 11u32 { // swap
        let y = s.pop().unwrap();
        let u = s.pop().unwrap();
        s.push(y);
        s.push(u);
        return (c, s, a, v, f);
    }
    if x == 13u32 { // tuck
        let x = s.pop().unwrap();
        let y = s.pop().unwrap();
        let z = s.pop().unwrap();
        s.push(z);
        s.push(x);
        s.push(y);
        return (c, s, a, v, f);
    }
    if x == 15u32 { // 2swap
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
    if x == 19u32 { // call
        let name = s.pop().unwrap();
        let value = get_function(f.clone(), name);
        let c = vec_append(value, c);
        return (c, s, a, v, f);
    }
    if x == 21u32 { //push1
        let y = c.pop().unwrap();
        s.push(y);
        return (c, s, a, v, f);
    }
    if x == 23u32 { //push1
        let y = c.pop().unwrap();
        s.push(y);
        let y = c.pop().unwrap();
        s.push(y);
        let y = c.pop().unwrap();
        s.push(y);
        return (c, s, a, v, f);
    }
    if x == 25u32 { //if
        let y = s.pop().unwrap();
        if y == 0 {
            let (_, c) = split(c, 26);
            return (c, s, a, v, f);
        }
        return (c, s, a, v, f);
    }
    if x == 27u32 { //else
        return (c, s, a, v, f);
    }
    if x == 29u32 { // >
        let y = s.pop().unwrap();
        let z = s.pop().unwrap();
        if z > y {
            s.push(1u32);
            return (c, s, a, v, f);
        }
        s.push(0u32);
        return (c, s, a, v, f);
    }            
    if x == 31u32 { // drop
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

fn get_function(mut f: Vec<(u32, Vec<u32>)>, n:u32) -> Vec<u32> {
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
fn split(c: Vec<u32>, s: u32) -> (Vec<u32>, Vec<u32>) {
    return split2(c, vec![], s);
}
fn split2(mut c: Vec<u32>, mut d: Vec<u32>, s: u32) -> (Vec<u32>, Vec<u32>) {
    loop {
    let a = c.pop().unwrap();
        if a == s {
            break;
        }
        d.push(a);
    }
    return (d, c);
}
fn replace(old: u32, new: u32, mut v: Vec<u32>) -> Vec<u32> {
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
fn vec_append(top: Vec<u32>, bottom: Vec<u32>) -> Vec<u32> {
    return vec_append2(top, bottom);
}
fn vec_append2(mut t: Vec<u32>, mut bottom: Vec<u32>) -> Vec<u32> {
    loop {
        if t.len() == 0 {
            break;
        }
        let a = t.pop().unwrap();
        bottom.push(a);
    }
    return bottom;
}
