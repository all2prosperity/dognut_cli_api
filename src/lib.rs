extern crate core;
extern crate libc;
extern crate bitflags;

#[cfg(rtc)]
pub mod decode;
pub mod network;
pub mod img_decode;
pub mod pb;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
