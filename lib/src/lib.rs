pub mod crypto;
pub mod sha256;
pub mod types;
pub mod util;

use serde::{Deserialize, Serialize};
use uint::construct_uint;
//macro_rules! macros generates this at compile time, no need to write it
construct_uint! {
    #[derive(Serialize, Deserialize)]
    pub struct U256(4);
}
