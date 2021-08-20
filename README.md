Rust_example 
======
By ITUtopia

Rust
-
Rust是一门系统(System)编程语言,专注于安全,尤其是并发安全，支持函数式和命令式以及泛型等编程范式的多范式语言。

Rust在语法上和C++类似，但是设计者想要在保证性能的同时提供更好的内存安全。

Rust最初是由Mozilla研究院的Graydon Hoare设计创造，然后在Dave Herman, Brendan Eich以及很多其他人的贡献下逐步完善的。

Rust的设计者们通过在研发[Servo](https://github.com/servo/servo)网站浏览器布局引擎过程中积累的经验优化了Rust语言和Rust编译器。

Rust编译器是在MIT License 和 Apache License 2.0双重协议声明下的免费开源软件。

Rust已经连续四年（2016，2017，2018，2019）在Stack Overflow开发者调查的“最受喜爱编程语言”评选项目中折取桂冠。

安装和开发搭建
==

### Mac环境 
1.安装(使用brew各种出错，还慢)

curl https://sh.rustup.rs -sSf | sh

2编译

source $HOME/.cargo/env

3.版本查看

rustc --version

4.安装包版本查看

cargo --version

5.第一个例子

名为hello.rs
fn main() {
    println!("Hello World!");
}

6.编译代码

rustc hello.rs

7.运行可执行文件

./hello




