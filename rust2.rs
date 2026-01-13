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
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
// // 2. 枚举 + Debug
// #[derive(Debug)]
// enum TrafficLight {
//     Red,
//     Yellow,
//     Green,
// }
// #[derive(Debug)]
// enum OperationResult {
//     Success(i32),
//     Error(&'static str),
// }
// // 3. Option枚举;除法函数
// fn divide(a: i32, b: i32) -> OperationResult {
//     if b == 0 {
//         OperationResult::Error("除数不能为0") 
//     } else {
//         OperationResult::Success(a / b)
//     }
// }
// fn main() {
//     // 结构体使用
//     let rect = Rectangle { width: 10, height: 20 };
//     println!("矩形面积：{}，详情：{:?}", rect.area(), rect);

//     // 红绿灯
//     let lights = [TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green];
//     for light in lights {
//         let action = match light {
//             TrafficLight::Red => "停止",
//             TrafficLight::Yellow => "减速",
//             TrafficLight::Green => "通行",
//         };
//         println!("交通灯({:?})：{}", light, action);
//     }

//     // 除法 + OperationResult
//     let division1 = divide(10, 2); 
//     let division2 = divide(10, 0); 
//     let results = [division1, division2];
    
//     for res in results {
//         match res {
//             OperationResult::Success(val) => println!("除法结果：{}", val),
//             OperationResult::Error(msg) => println!("除法错误：{}", msg),
//         }
//     }
// }
//2026-1-13日
//1. 结构体进阶：嵌套结构体 + 方法扩展
#[derive(Debug)]
// 坐标点（基础结构体）
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
// 矩形
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Point {
// 创建原点
    fn origin() -> Self {
        Point { x: 0, y: 0 }
    }

// 创建自定义点
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

// 移动坐标
    fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

// 实例方法：计算到另一个点的距离平方
    fn distance_sq(&self, other: &Point) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

impl Rectangle {
// 实例方法：计算矩形面积
    fn area(&self) -> u32 {
        let width = (self.bottom_right.x - self.top_left.x).abs() as u32;
        let height = (self.bottom_right.y - self.top_left.y).abs() as u32;
        width * height
    }

// 实例方法：判断点是否在矩形内
    fn contains(&self, p: &Point) -> bool {
        p.x >= self.top_left.x && p.x <= self.bottom_right.x
        && p.y >= self.top_left.y && p.y <= self.bottom_right.y
    }
}

//2. Result错误处理 + 枚举方法
#[derive(Debug)]
// 自定义错误类型
enum CalculationError {
    DivisionByZero,
    NegativeSquareRoot(i32),
    InvalidInput(&'static str),
}

#[derive(Debug)]
// 操作类型枚举
enum MathOperation {
    Add(i32, i32),
    Subtract(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
    SquareRoot(i32),
    Invalid(&'static str),
}

impl MathOperation {
// 枚举方法：执行并返回Result
    fn execute(&self) -> Result<i32, CalculationError> {
        match self {
            MathOperation::Add(a, b) => Ok(a + b),
            MathOperation::Subtract(a, b) => Ok(a - b), 
            MathOperation::Multiply(a, b) => Ok(a * b),
            MathOperation::Divide(a, b) => {
                if *b == 0 {
                    Err(CalculationError::DivisionByZero)
                } else {
                    Ok(a / b)
                }
            }
            MathOperation::SquareRoot(n) => {
                if *n < 0 {
                    Err(CalculationError::NegativeSquareRoot(*n))
                } else {
                    Ok((*n as f64).sqrt() as i32)
                }
            }
            MathOperation::Invalid(msg) => {
                Err(CalculationError::InvalidInput(msg))
            }
        }
    }

// 打印错误详情
    fn print_error_detail(err: &CalculationError) {
        match err {
            CalculationError::DivisionByZero => println!("错误：除数不能为0"),
            CalculationError::NegativeSquareRoot(n) => println!("错误：无法计算{}的平方根（负数）", n), // 读取字段
            CalculationError::InvalidInput(msg) => println!("错误：输入无效 - {}", msg),
        }
    }
}

//3. Trait基础：定义/实现特质
// 定义特质
trait Describable {
    fn describe(&self) -> String;

// 默认方法
    fn print_description(&self) {
        println!("描述：{}", self.describe());
    }
}
impl Describable for Point {
    fn describe(&self) -> String {
        format!("坐标点({},{})", self.x, self.y)
    }
}
impl Describable for MathOperation {
    fn describe(&self) -> String {
        match self {
            MathOperation::Add(a, b) => format!("{} + {}", a, b),
            MathOperation::Subtract(a, b) => format!("{} - {}", a, b),
            MathOperation::Multiply(a, b) => format!("{} × {}", a, b),
            MathOperation::Divide(a, b) => format!("{} ÷ {}", a, b),
            MathOperation::SquareRoot(n) => format!("√{}", n),
            MathOperation::Invalid(msg) => format!("无效操作：{}", msg),
        }
    }
}
// 4. 模式匹配进阶：if let/while let
// 简化Option匹配
fn print_option(opt: Option<i32>) {
// 替代match opt
    if let Some(value) = opt {
        println!("Option有值：{}", value);
    } else {
        println!("Option无值（None）");
    }
}

// 简化循环匹配
fn count_down() {
    let mut count = 5;
    while let Some(n) = if count >= 0 { Some(count) } else { None } {
        println!("倒计时：{}", n);
        count -= 1;
    }
    println!("倒计时结束！");
}


fn main() {
// 1. 嵌套结构体
    println!(" 嵌套结构体演示 ");
    let mut p1 = Point::new(1, 2);
    let p2 = Point::origin();
    let rect = Rectangle {
        top_left: Point::new(0, 5),
        bottom_right: Point::new(10, 0),
    };

    p1.move_by(3, 4); 
    println!("移动后的p1：{:?}", p1);
    println!("p1到p2的距离平方：{}", p1.distance_sq(&p2));
    println!("矩形面积：{}", rect.area());
    println!("p2是否在矩形内：{}", rect.contains(&p2));
    println!("p1是否在矩形内：{}", rect.contains(&p1));

//2. Trait特质
    println!("\n Trait特质演示 ");
    p1.print_description();
    let op_sub = MathOperation::Subtract(20, 8);
    op_sub.print_description();

//3. 枚举+Result错误处理
    println!("\n 枚举+错误处理演示 ");
    let operations = [
        MathOperation::Add(5, 3),
        MathOperation::Subtract(20, 8), 
        MathOperation::Divide(10, 0),   
        MathOperation::SquareRoot(-9), 
        MathOperation::Multiply(4, 5),
        MathOperation::SquareRoot(16),
        MathOperation::Invalid("非数字输入"), 
    ];
    for op in operations {
        op.print_description();
        match op.execute() {
            Ok(result) => println!("运算结果：{}", result),
            Err(e) => {
                MathOperation::print_error_detail(&e); 
            }
        }
    }

//4. 模式匹配进阶（if let/while let）
    println!("\n 模式匹配进阶演示 ");
    print_option(Some(99));
    print_option(None);
    count_down();
}