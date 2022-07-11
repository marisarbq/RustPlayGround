use core::num;
use std::borrow::{Borrow, BorrowMut};




fn main() {
    println!("Hello, world!");

    let mut number = 0;
    //可变变量
    number = -100;
    //shadowing 重新分配内存
    let number = "变成文字";
    let mut string = "文字";
    let mut string = &mut String::from("文字"); //&String = &str; * & 等符号是等后面表达式完全执行完了再生效的 &(epxression)
    // number = "不可变会报错"; 
    // {
    //     println!("内部:{}, {}",number, &string);
    // }

    // println!("外部:{}, {}",number, &string);

    // let clo_fn = || {
    //     println!("闭包内部:{}, {}",number, string);
    //     ()
    // };
    // clo_fn();
    println!("闭包外部:{}, {}",number, string);



    let bool = true;
}
