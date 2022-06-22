fn main() {
    println!("Hello, world!");
}


pub mod a {
    pub const I: i32 = 3;

    fn semisecret(x: i32) -> i32 {
        use self::b::c::J;
        x + J
    }

    pub fn bar(z: i32) -> i32 {
        semisecret(I) * z
    }
    pub fn foo(y: i32) -> i32 {
        semisecret(I) + y
    }

    mod b {
        pub(in crate::a) mod c {
            pub(in crate::a) const J: i32 = 4;
        }
    }
}

/*
pub(crate) 或 pub(in crate::a) 就是限制可见性语法，前者是限制在整个包内可见，后者是通过绝对路径，限制在包内的某个模块内可见，总结一下：

pub 意味着可见性无任何限制
pub(crate) 表示在当前包可见
pub(self) 在当前模块可见
pub(super) 在父模块可见
pub(in <path>) 表示在某个路径代表的模块中可见，其中 path 必须是父模块或者祖先模块

*/