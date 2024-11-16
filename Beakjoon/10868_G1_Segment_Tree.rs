use core::num;
use std::{backtrace, cmp, fmt::Write, io};

fn main() {
    //input n, m
    let (n, m) = input_two_value();
    let loopSize = n + 1;

    let mut tree: Vec<i32> = Vec::new();
    tree.resize(1 << 18, 1000000001);
    
    //input value and update segment tree
    for i in (1..loopSize) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("FAIL");
        let trimmed = input.trim();

        match trimmed.parse::<i32>() {
            Ok(num) => update(&mut tree, 1, 1, n, i, num),
            Err(_) => println!("That's not a valid integer!"),
        }
    }
    
    //query segment tree [a,b]
    let mut output = String::new();
    for i in (0..m) {
        let (a, b) = input_two_value();
        let ret = query(&mut tree, 1, 1, n, a, b);

        writeln!(output, "{}", ret).unwrap();
    }
    print!("{}", output);
}

fn update(_tree: &mut Vec<i32>, node: usize, s: i32, e: i32, idx: i32, cnt: i32) {
    _tree[node] = cmp::min(_tree[node], cnt);
    if (s != e) {
        let mid = (s + e) / 2;
        if (idx <= mid) {
            update(_tree, node * 2, s, mid, idx, cnt);
        } else {
            update(_tree, node * 2 + 1, mid + 1, e, idx, cnt);
        }
    }
}

fn query(_tree: &mut Vec<i32>, node: usize, s: i32, e: i32, l: i32, r: i32) -> i32 {
    if (e < l || r < s) {
        return 1000000001;
    }
    if (l <= s && e <= r) {
        return _tree[node];
    }

    let mid = (s + e) / 2;
    let first = query(_tree, node * 2, s, mid, l, r);
    let second = query(_tree, node * 2 + 1, mid + 1, e, l, r);
    return cmp::min(first, second);
}

fn input_two_value() -> (i32, i32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("FAIL");
    let input = input.trim();
    let mut iter = input.split_whitespace();
    let n: i32 = iter
        .next()
        .expect("N input Fail")
        .parse()
        .expect("N parse Fail");
    let m: i32 = iter
        .next()
        .expect("M input Fail")
        .parse()
        .expect("M parse Fail");

    (n, m)
}
