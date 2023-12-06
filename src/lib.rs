#![no_std]

mod allocation;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod error;

extern crate alloc;

use crate::error::AoCError;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;

static SOLUTIONS: &[fn(String) -> Result<String, AoCError>] = &[
    day1::part1,
    day1::part2,
    day2::part1,
    day2::part2,
    day3::part1,
    day3::part2,
    day4::part1,
    day4::part2,
    day5::part1,
    day5::part2,
    day6::part1,
    day6::part2,
];

#[repr(C, packed)]
struct JSString {
    data: *mut u8,
    len: usize,
}

#[no_mangle]
extern "C" fn solve(index: usize, str: *mut JSString) -> bool {
    let input = unsafe { String::from_raw_parts((*str).data, (*str).len, (*str).len) };
    let (ok, output) = SOLUTIONS
        .get(index)
        .map_or(Err(format!("Invalid index {}", index).into()), |f| f(input))
        .map_or_else(|e| (false, String::from(e)), |r| (true, r));
    unsafe {
        (*str).len = output.len();
        (*str).data = Box::leak(output.into_boxed_str()).as_mut_ptr();
    }
    ok
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}
