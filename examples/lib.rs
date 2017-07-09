extern crate iso4217;
extern crate num;
use iso4217::ISO4217;
use num::FromPrimitive;

fn main() {
    let code = ISO4217::XAU;
    let num_code = code as i32;
    println!("{}", code);
    println!("{}", num_code);
    println!("{:?}", ISO4217::from_i32(num_code));
}
