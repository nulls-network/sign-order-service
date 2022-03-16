use libsecp256k1::{sign, Message, SecretKey, recover, RecoveryId};

use web3::signing::keccak256;
use bytes::buf::BufMut;

#[derive(Debug)]
struct TryFromSliceError(());

/// A signature (a 512-bit value, plus 8 bits for recovery ID).
#[derive(Clone, Copy, Debug)]
pub struct Signature(pub [u8; 65]);

const DEFAULT_V: u8 = 27u8;

fn slice_to_array_32<T>(slice: &[T]) -> Result<&[T; 32], TryFromSliceError> {
    if slice.len() == 32 {
        let ptr = slice.as_ptr() as *const [T; 32];
        unsafe {Ok(&*ptr)}
    } else {
        Err(TryFromSliceError(()))
    }
}

fn slice_to_array_64<T>(slice: &[T]) -> Result<&[T; 64], TryFromSliceError> {
    if slice.len() == 64 {
        let ptr = slice.as_ptr() as *const [T; 64];
        unsafe {Ok(&*ptr)}
    } else {
        Err(TryFromSliceError(()))
    }
}

fn to_bytes(data: Vec<String>) -> [u8;32] {
    let mut bytes: Vec<u8> = Vec::new();

    for s in data  {
        bytes.put_slice(s.as_bytes());
    }

    return keccak256(bytes.as_slice());
}

impl Default for Signature {
    fn default() -> Self {
        Signature([0u8; 65])
    }
}

impl From<(libsecp256k1::Signature, libsecp256k1::RecoveryId)> for Signature {
    fn from(x: (libsecp256k1::Signature, libsecp256k1::RecoveryId)) -> Signature {
        println!("Signature is {:?}", x);
        let mut r = Self::default();
        r.0[0..64].copy_from_slice(&x.0.serialize()[..]);
        r.0[64] = DEFAULT_V + x.1.serialize();
        r
    }
}

pub fn sign_data(data: Vec<String>, private_key: String) -> Signature {
    let bytes = to_bytes(data);
    println!("Hash is: {}", hex::encode(bytes));
    let message: Message = Message::parse(&bytes);
    let pk = hex::decode(private_key).unwrap();
    let secret_key = SecretKey::parse(slice_to_array_32(pk.as_slice()).unwrap()).unwrap();
    let signature = sign(&message, &secret_key);
    signature.into()
}

pub fn recover_data(data: Vec<String>, sign: String) -> Option<String> {
    let bytes = to_bytes(data);
    println!("Hash is: {}", hex::encode(bytes));
    let message: Message = Message::parse(&bytes);

    let sign_hex = hex::decode(sign).unwrap();
    let signature: libsecp256k1::Signature = libsecp256k1::Signature::parse_overflowing(slice_to_array_64(&sign_hex[0..64]).unwrap());
    let result = recover(&message, &signature, &RecoveryId::parse(sign_hex[64] - DEFAULT_V).unwrap());

    match result {
        Ok(pub_key) => {
            let pub_key_hash: [u8; 32] = keccak256(&pub_key.serialize()[1..65]);
            let eth_pub_key = &pub_key_hash.as_slice()[12..32];
            Option::Some(hex::encode(eth_pub_key))
        }
        Err(e) => {
            println!("{}", e);
            Option::None
        }
    }
}

#[test]
fn test() {
    let private_key = String::from("6704f9a70210bdaedd08fc89b7711c2b05fe68de91117886fd4931882232ac7f");
    let mut vec: Vec<String> = Vec::new();
    vec.push("202203160233113243".to_string());
    vec.push("100000001".to_string());
    vec.push("TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t".to_string());
    vec.push("1000000".to_string());
    vec.push("Y".to_string());
    let signature = sign_data(vec.clone(), private_key.clone());
    println!("Sign is {:?}", hex::encode(signature.0));

    let result = recover_data(vec, hex::encode(signature.0));

    println!("PubKey is {:?}", result);
}
