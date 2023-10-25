#![no_std]

mod allocation;
mod day0;
mod day1;
mod error;

extern crate alloc;

use crate::error::AoCError;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;

static SOLUTIONS: [fn(String) -> Result<String, AoCError>; 2] = [day0::solve, day1::solve];

#[repr(C, packed)]
struct JSString {
    data: *mut u8,
    len: usize,
}

#[no_mangle]
extern "C" fn solve(day: usize, str: *mut JSString) -> bool {
    let input = unsafe { String::from_raw_parts((*str).data, (*str).len, (*str).len) };
    let (ok, output) = SOLUTIONS
        .get(day)
        .map_or(Err(format!("Invalid day {}", day).into()), |f| f(input))
        .map_or_else(|e| (false, String::from(e)), |r| (true, r));
    unsafe {
        (*str).len = output.len();
        (*str).data = Box::into_raw(output.into_boxed_str()) as *mut u8;
    }
    ok
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}
