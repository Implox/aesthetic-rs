extern crate unicode_hfwidth;
extern crate libaesthetic;

use std::io;
use std::io::Read;
use std::env;

use libaesthetic::make_aesthetic;

fn main() {
    if let Some(input_str) = env::args().nth(1).or_else(|| {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).map(|_| input).ok()
    }) {
        let aesthetic = make_aesthetic(input_str);
        println!("{}", aesthetic);
    }
}
