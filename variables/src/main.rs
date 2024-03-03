fn main() {
    // 数据类型
    let gusess: u32 = "42".parse().expect("Not a number!");
    println!("gusess value is {}", gusess);
    // 标量类型
    let x = 2.0; // f64
    let y: f32 = 3.8; // f32
    println!("x value is {}", x);
    println!("y value is {}", y);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("sum value is {}", sum);
    println!("difference value is {}", difference);
    println!("product value is {}", product);
    println!("quotient value is {}", quotient);
    println!("remainder value is {}", remainder);

    let t = true;
    let f: bool = false; // 附带了显式标注类型
    println!("t value is {}", t);
    println!("f value is {}", f);

    let c = 'z';
    let z = 'w';
    let heart_eyed_cat = '☯';
    println!("c value is {}", c);
    println!("z value is {}", z);
    println!("heart_eyed_cat value is {}", heart_eyed_cat);

    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // 解包？
    println!("The value of x is :{}", x);
    println!("The value of y is :{}", y);
    println!("The value of z is :{}", z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The value of five_hundred is :{}", five_hundred);
    println!("The value of six_point_four is :{}", six_point_four);
    println!("The value of one is :{}", one);

    // 数组
    let a = [1, 2, 3, 4, 5];
    println!("The value of a index 1 is :{}", a[1]);
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("The value of January is :{}", months[0]);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a.0 is :{}", a[0]);
    // 初始化 size 为 5 的数组，其中每个值都是 3
    let a = [3; 5];
    println!("The value of a.0 is :{}", a[0]);
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    let third_index = 2;
    println!("The value of first is :{}", first);
    println!("The value of second is :{}", second);
    println!("The value of third is :{}", a[third_index]);
}
