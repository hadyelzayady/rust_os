//disable automatically linked standard library to run the code on bare hardware(freestanding code)
#![no_std]
//usual program before executing main, some other stuff starts before that like setting stack overflow guard
//this happens in crt0 (C runtime zero) then this crt0 invokes the entry point of rust runtime(main fn) which is marked by the 'start'
//language item. but our freestanding executable does have access to crt0 so we have to define our entry point
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;
// \! is the never return type to mark diverging function
//panic info contains the file and line where the panic happened
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

//*Language items are special functions and types that are required internally by the compiler.
//For example, the Copy trait is a language item that tells the compiler which types have copy semantics.
//When we look at the implementation of copy trait, we see it has the special #[lang = "copy"] attribute that defines it as a language item.

//*The eh_personality language item marks a function that is used for implementing stack unwinding.
//stack unwinding is complicated so we won't implement it, instead we disable it

//to not change the name of exported function _start
//extern "C" means use the C calling convention (calling a function in C pushes the address of current instruction and in called fn the ebp is pushed etc)

//the start never returns because this is our os which is called by bootloader and the only way to exit is to shutdown the machine

#[no_mangle]
pub extern "C" fn _start() -> ! {
    //after making static writer we can use writer directly from here(instead of carrying the instance around or call function print_something)
    use core::fmt::Write;

    vga_buffer::WRITER
        .lock()
        .write_str("hello again\n")
        .unwrap();

    write!(
        vga_buffer::WRITER.lock(),
        "some numbers {} and {}\n",
        42,
        10 / 3
    )
    .unwrap();
    //using the write_str direcly from fmt:write trait
    //this confirms write! macro uses write_str
    vga_buffer::WRITER
        .lock()
        .write_str("this is write_str\n")
        .unwrap();

    //use our custom println!
    //as this is our crate we don't have to write use crate::println!
    println!("Hello println {}", "!");
    //test panic handler
    panic!("Error hapenned");

    loop {}
}
