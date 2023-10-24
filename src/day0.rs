use crate::error::AoCError;
use alloc::borrow::ToOwned;
use alloc::string::String;

pub(crate) fn solve(input: String) -> Result<String, AoCError> {
    Ok("Hello, ".to_owned() + &input + "!")
}
