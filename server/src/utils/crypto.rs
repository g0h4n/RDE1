use log::error;
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, SaltString},
    Params, Pbkdf2,
};
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

/// Function to provide simple mechanism to decrypt some bytes with a key using AES-256-CBC
/// Thanks to: <https://github.com/jj-style/stegosaurust/blob/master/src/crypto.rs>
pub fn aes_decrypt(
    ciphertext: &[u8],
    key: &[u8]
) -> Vec<u8> {
    if !ciphertext.starts_with(b"Salted__") {
        error!("Message was not encrypted when encoded");
    }
    if ciphertext.len() < 16 {
        error!("Ciphertext is too short");
    }
    let (_, rest) = ciphertext.split_at(8); //ignore prefix 'Salted__'
    let (s, rest) = rest.split_at(8);
    let s = String::from_utf8(s.to_vec()).unwrap();
    let salt = SaltString::new(&s).unwrap();
    let password_hash = hash_password(key, &salt).unwrap();
    let password_hash = password_hash.hash.unwrap();
    let (key, iv) = password_hash.as_bytes().split_at(32);
    let cipher = Aes256CbcDec::new_from_slices(key, iv).unwrap();
    let r = cipher.decrypt_padded_vec_mut::<Pkcs7>(rest);
    match r {
        Ok(plaintext) => { return plaintext }
        Err(err) => { 
            error!("Inccorect key, can't decode AES! Reason: {err}");
            return Vec::new()
        }
    }
}

/// Function to hash password and salt,
/// to generate key for use with AES-256 encryption.
///
/// Uses PBKDF2 with 10,000 rounds of SHA256 hashing to generate a 48-byte response.
/// 48-byte response contains the 16-byte IV and 32-byte key.
/// Thanks to: <https://github.com/jj-style/stegosaurust/blob/master/src/crypto.rs>
pub fn hash_password<'a>(
    key: &'a [u8],
    salt: &'a SaltString,
) -> Result<PasswordHash<'a>, pbkdf2::password_hash::Error> {
    Pbkdf2.hash_password_customized(
        key,
        None,
        None,
        Params {
            rounds: 10_000,
            output_length: 48,
        },
        salt,
    )
}

/// Function to XOR (not used)
/// XOR u8^u8 
pub fn xor(
    datas: Vec<u8>,
    key: Vec<u8>
) -> Vec<u8> {
    let key_repeated = repeat_key(&key, datas.len());
    let ebytes : Vec<u8> = datas.iter()
        .zip(key_repeated)
        .map(|(d, k)| d ^ k)
        .collect();
    return ebytes
}

/// Function to repeate KEY to the same FILE buffer len() (not used)
/// From: <https://github.com/gavynriebau/xor/blob/master/src/main.rs#L309>
fn repeat_key(
    key : &Vec<u8>,
    required_len : usize
) -> Vec<u8> {
    let mut key_repeated = Vec::with_capacity(required_len);
    while key_repeated.len() < required_len {
        for &b in key {
            key_repeated.push(b);
            if key_repeated.len() == required_len {
                break;
            }
        }
    }
    key_repeated
}