use crate::U256;
use serde::Deserialize;
use serde::Serialize;
use sha256::digest;
use std::fmt;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Hash(U256);
impl Hash {
    //hash anything that can be serde serialized
    //generic syntax
    pub fn hash<T: serde::Serialize>(data: &T) -> Self {
        let mut serialized: Vec<u8> = vec![];
        if let Err(e) = ciborium::into_writer(data, &mut serialized) {
            panic!("Failed to serialize data: {:?}. This should not happen", e)
        }

        //"digest" the data in serialized format into a hex.
        let hash = digest(&serialized);
        let hash_bytes = hex::decode(hash).unwrap();
        //since rust prefers explicit programming, we need to
        //explicitly convert the vector output to an array hash_array
        let hash_array: [u8; 32] = hash_bytes.as_slice().try_into().unwrap();
        //this array of 32 bytes is converted then to an integer of 256 bits
        //and then stored in a tuple as a single element.
        Hash(U256::from_big_endian(&hash_array)) //for some reason ::from is not working
    }
    //check if a hash matches a target.
    //note: in real bitcoin implementation, they don't specify the target
    //explicitly but rather give us tools for inferring it on the go if needed.
    pub fn matches_target(&self, target: U256) -> bool {
        self.0 <= target
    }
    //zero hash (used for coinbase tx etc.)
    pub fn zero() -> Self {
        Hash(U256::zero())
    }
}
//this works like operator overloading.

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //"{:x}" is a directive specifying that self.0 should not be
        //printed in an integer format but rather in the hexadecimal format
        //which is done by "f" (our formatter)
        write!(f, "{:x}", self.0)
    }
}
