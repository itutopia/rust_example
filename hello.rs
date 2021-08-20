fn main(){
    // 1. println!() 打印;
    println!("hello rust!");
        
    
    // 2.不可变变量关键字: let;  声明变量; rust强类型语言,具有自动判断变量的能力.
    // Rust 语言为了高并发安全而做的设计：在语言层面尽量少的让变量的值可以改变。所以 a 的值不可变。但这不意味着 a 不是"变量"（英文中的 variable），官方文档称 a 这种变量为"不可变变量"。
    let a = 12;
    //    a="abc"  a 声明后就被确定为整型数字，不能把字符串类型的值赋给它。  error: aborting due to previous error
    println!("a is {}", a);


    // 3.可变变量关键字: mut;   "可变"（mutable）可变变量
    let mut _b = 123;
     _b = 456;
    println!("b is {}",_b);

    // 4. 循环
    let mut number = 1;
    while number != 4 {
        println!("{}",number);
        number += 1;
    }
    println!("while loop demo");


    // 5. 循环2
    let mut l = 0;
    while l < 10 {
        l += 1;
        println!("l is {}" ,l);
    }
    
   
    // 6. for 循环:迭代遍历
    let a = [10,20,30,40,50];
    for i in a.iter(){
        println!("value is {} ", i);
    }

    // 7. for循环,访问数组指定下表
    for z in 0..5{
        println!("z[{}]={}",z,a[z]);
    }

    // 8. loop循环
    let s = ['I','T','U','T','O','P','I','A'];
    let mut m = 0;
    loop {
        let ch = s[m];
        if ch == '0'{
            break;
        }
        println!("\'{}\'",ch);
        m += 1;
    }

    // 9. loop循环 返回值.
    let x = ['T','E','A'];
    let mut n = 0;
    let location = loop {
        let ch= x[n];
        if ch == 'E'{
            break;
        }
        n += 1;
    };
    println!("\'0\'的索引为{:?}",location);

    println!("hello world");
}
