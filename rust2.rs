//2026-1-8日

// fn main() {
//    println!("Hello, Rust! ");
// }


//2026-1-9日

// 生命周期
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

// 传值+传引用
fn add(x: i8, y: i8) -> i8 { x + y }
fn modify(n: &mut i8) { *n *= 2; }

fn main() {
// 所有权
    let s = "rust demo";

// 基本/复合类型+常量
    let num = 5;
    let tup = (num, true);
    let arr = [1, 2];
    const MAX: u8 = 2;
    let loop_times = MAX; 

// 控制流
    let mut sum = 0;
    for n in arr { sum = add(sum, n); }
    let mut cnt = 0;
    while cnt < loop_times { 
        cnt += 1;
    }

// 传引用
    let mut val = 10;
    modify(&mut val);

    println!("{} {:?} {} {} {}", s, tup, sum, val, longer("a", "bb"));
}