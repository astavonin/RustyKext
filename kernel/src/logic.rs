use externals;

pub fn do_work() {
    print("hello from rust\n");
}

fn print(s: &str) {
    let msg = s.as_ptr() as *const u8;
    unsafe {
        externals::printf(msg);
    }
}


