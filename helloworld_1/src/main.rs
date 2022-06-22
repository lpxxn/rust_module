use crate::say::hi;

fn main() {
    // 相对路径
    say::hello();
    // 绝对路径调用
    crate::say::hello();

    // 不使用 use
    say::hi::hi_1();
    say::hi::hi_2();
    // 使用 use 后就可以这么调用
    hi::hi_1();

    // 重导出名称
    people::hi::hi_1();
    people::hello();
    // 但是不能 
    // people::say::hello();

    people_2::people::hello();
    people_2::info::name();
}

mod say {
    pub fn hello() {
        println!("Hello, world!");
    }
    fn hello_2() {
        println!("hello")
    }
    pub mod hi {
        pub fn hi_1() {
            super::hello_2();
        }
        pub fn hi_2() {
            println!("hi there");
        }
    }
}

pub mod people {
    // 重导出名称, main里的第16行用了say的hi方法
    pub use crate::say::hi;
    use crate::say;
    pub fn hello() {
        say::hello();
    }
    pub mod info {
        pub fn name() {
            println!("zhangsang");
        }
    }
}


mod people_2 {
    // 重导出名称
    // self main 21 行
    pub use crate::people::{self, info};
    pub fn hello() {
        info::name();
        // 如果去掉use里的info 可以直接这么用
        //people::info::name();
    }
}
/*
上面使用到了模块章节提到的 self 关键字，用来替代模块自身，结合上一节中的 self，可以得出它在模块中的两个用途：

use self::xxx，表示加载当前模块中的 xxx。此时 self 可省略
use xxx::{self, yyy}，表示，加载当前路径下模块 xxx 本身，以及模块 xxx 下的 yyy */
