pub fn panic_handler_generate_coredump()->(){
    let default_panic = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic_info|{
        default_panic(panic_info);

        let pid = std::process::id();

        use libc::kill;
        use libc::SIGQUIT;

        use std::convert::TryInto;
        unsafe { kill(pid.try_into().unwrap(), SIGQUIT)};
    }))
}

fn main() {
    panic_handler_generate_coredump();
    panic!("no panic");
}
