pub mod crypto;
pub mod sha256;
pub mod types;
pub mod util;

//not really required.
extern crate ciborium;
extern crate serde;
extern crate sha256;

use uint::construct_uint;
//macro_rules! macros generates this at compile time, no need to write it
construct_uint! {
    pub struct U256(4);
}
