#[cfg(target_os = "emscripten")]
extern crate meowscript;

#[cfg(not(target_os = "emscripten"))]
use std::env;

#[cfg(not(target_os = "emscripten"))]
use std::fs;


#[cfg(target_os = "emscripten")]
use std::{ffi::CString, mem, os::raw::c_char};

#[cfg(not(target_os = "emscripten"))]
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 1 && args[1].as_str() == "run" {
        let filename = &args[2].split('.').collect::<Vec<_>>();
        if filename[filename.len() - 1] != "meow" {
            println!("File must have the extention .meow");
            return;
        }
        let content = fs::read_to_string(&args[2]).expect("Could not read file.");

        meowscript::interpret(content.as_str());
    } else {
    
        println!(
            "Welcome to the MeowScript REPL. Type in commands to get started.",
        );
        meowscript::repl::start();
    }

}

#[cfg(target_os = "emscripten")]
fn main() {}

#[cfg(target_os = "emscripten")]
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn interpret(src: *mut c_char) -> usize {
    let code = CString::from_raw(src).to_str().unwrap().to_string();
    let _ = meowscript::interpret(&code);
    0
}


#[cfg(target_os = "emscripten")]
#[no_mangle]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    let mut buf: Vec<u8> = Vec::with_capacity(len);

    let ptr = buf.as_mut_ptr();

    mem::forget(buf);

    ptr
}

#[cfg(target_os = "emscripten")]
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, len: usize) {
    let data = Vec::from_raw_parts(ptr, len, len);

    mem::drop(data)
}