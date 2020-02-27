
// 使用 no_std 属性 声明禁用链接标准库
#![no_std] 
// 禁用所有 Rust 层级入口点。
#![no_main]

// fn main() {
    // println!("Hello, world!");
// }

// 导入依赖
use core::panic::PanicInfo;


/// panic是调用这个函数
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle] // 不重整函数名
// 因为编译器会寻找一个名为 ”_start“ 的函数， 所以这个函数就是入口点
// 默认命名为‘_start’
pub extern "C" fn _start() -> ! {
    loop{}
}
