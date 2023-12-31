//use log::error;
use rand::{distributions::Alphanumeric, Rng};
use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};
use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, SaltString},
    Params, Pbkdf2,
};
type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;

/// Function to provide simple mechanism to encrypt some bytes with a key using AES-256-CBC
/// Thanks to: <https://github.com/jj-style/stegosaurust/blob/master/src/crypto.rs>
pub fn aes_encrypt(
    plaintext: &[u8],
    key: &[u8]
) -> Vec<u8> {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    let salt = SaltString::new(&s).unwrap();
    let password_hash = hash_password(key, &salt).unwrap();
    let password_hash = password_hash.hash.unwrap();
    let (key, iv) = password_hash.as_bytes().split_at(32);
    let cipher = Aes256CbcEnc::new_from_slices(key, iv).unwrap();
    let ciphertext = cipher.encrypt_padded_vec_mut::<Pkcs7>(plaintext);
    let message = ["Salted__".as_bytes(), salt.as_bytes(), &ciphertext].concat();
    message
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
/// <https://github.com/gavynriebau/xor/blob/master/src/main.rs#L309>
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