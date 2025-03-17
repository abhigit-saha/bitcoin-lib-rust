use ecdsa::{Signature as ECDSASignature, SigningKey, VerifyingKey, signature::Signer};
use k256::Secp256k1;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature(pub ECDSASignature<Secp256k1>);

//note: publickey, merkleroot and hash also have partialeq and eq
//that implement equality and inequality.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PublicKey(pub VerifyingKey<Secp256k1>);

#[derive(Debug, Serialize, Deserialize, Clone)]

//kind of like operator overloading to inherit the deserialize and serialize functions
//from the module definition
pub struct PrivateKey(#[serde(with = "signkey_serde")] pub SigningKey<Secp256k1>);
impl PrivateKey {
    pub fn new_key() -> Self {
        PrivateKey(SigningKey::random(&mut rand::thread_rng()))
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.verifying_key().clone())
    }
}

mod signkey_serde {
    use serde::Deserialize;
    pub fn serialize<S>(
        key: &super::SigningKey<super::Secp256k1>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&key.to_bytes())
    }
    //the serde::Deserializer function is passed some data so the deserializer should
    //exist till the data exists, so we pass a lifecycle variable
    //that makes it last as long as the data itself
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<super::SigningKey<super::Secp256k1>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes: Vec<u8> = Vec::<u8>::deserialize(deserializer)?;
        Ok(super::SigningKey::from_slice(&bytes).unwrap())
    }
}
