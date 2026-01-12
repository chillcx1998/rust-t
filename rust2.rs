//2026-1-8日

// fn main() {
//    println!("Hello, Rust! ");
// }


//2026-1-9日

// 生命周期
//fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
//    if a.len() > b.len() { a } else { b }
//}

// 传值+传引用
//fn add(x: i8, y: i8) -> i8 { x + y }
//fn modify(n: &mut i8) { *n *= 2; }

//fn main() {
// 所有权
//    let s = "rust demo";

// 基本/复合类型+常量
//   let num = 5;
//   let tup = (num, true);
//   let arr = [1, 2];
//   const MAX: u8 = 2;
//   let loop_times = MAX; 

// 控制流
//   let mut sum = 0;
//  for n in arr { sum = add(sum, n); }
//    let mut cnt = 0;
//   while cnt < loop_times { 
//     cnt += 1;
//    }

// 传引用
//    let mut val = 10;
//    modify(&mut val);
//
//    println!("{} {:?} {} {} {}", s, tup, sum, val, longer("a", "bb"));
//}

//2026-1-12日
// 1. 结构体 + Debug
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
// 2. 枚举 + Debug
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
#[derive(Debug)]
enum OperationResult {
    Success(i32),
    Error(&'static str),
}
// 3. Option枚举;除法函数
fn divide(a: i32, b: i32) -> OperationResult {
    if b == 0 {
        OperationResult::Error("除数不能为0") 
    } else {
        OperationResult::Success(a / b)
    }
}
fn main() {
    // 结构体使用
    let rect = Rectangle { width: 10, height: 20 };
    println!("矩形面积：{}，详情：{:?}", rect.area(), rect);

    // 红绿灯
    let lights = [TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green];
    for light in lights {
        let action = match light {
            TrafficLight::Red => "停止",
            TrafficLight::Yellow => "减速",
            TrafficLight::Green => "通行",
        };
        println!("交通灯({:?})：{}", light, action);
    }

    // 除法 + OperationResult
    let division1 = divide(10, 2); 
    let division2 = divide(10, 0); 
    let results = [division1, division2];
    
    for res in results {
        match res {
            OperationResult::Success(val) => println!("除法结果：{}", val),
            OperationResult::Error(msg) => println!("除法错误：{}", msg),
        }
    }
}