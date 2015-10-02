#![feature(lang_items, no_std, core_str_ext)]
#![no_std]

mod externals;
mod logic;

#[no_mangle]
pub unsafe fn rust_main() {
    logic::do_work();
}


#[lang = "panic_fmt"]
#[allow(unused_variables)]
#[cfg(not(test))]
extern fn panic_fmt(args: &core::fmt::Arguments,
                    file: &str,
                    line: u32) -> ! {
    loop {}
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern fn eh_personality() {}

#[test]
fn it_works() {
}
