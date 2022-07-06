//https://www.bookstack.cn/read/DaseinPhaos-tlborm-chinese/mbe-macro-rules.md
//https://zhuanlan.zhihu.com/p/494952481


//公开一个宏
#[macro_export(local_inner_macros)]
macro_rules! my_test {
    ($($text:expr)*) => {
        $text
    }
}
