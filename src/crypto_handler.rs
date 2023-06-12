use aes::cipher::generic_array::{
  GenericArray,
  typenum::{
    UInt,
    UTerm,
    U12,
    bit::{B1, B0}
  }
};
use rand::Rng;
use std::{str, io::Error};
use pbkdf2::pbkdf2_hmac_array;
use sha2::Sha256;
use aes_gcm::{
  aead::{Aead, KeyInit, self},
  Aes256Gcm, Key,
};

pub fn generate_salt(length: usize) -> Vec<u8> {
  let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
  let salt: Vec<u8> = (0..length).map(|_| rng.gen()).collect();
  return salt;
}

// Derive a key from a user password using Argon2
pub fn derive_key(user_password: &str, salt: &[u8]) -> Result<[u8;32], Error> {
  let password: &[u8] = user_password.trim().as_bytes();
  // number of iterations
  let n: u32 = 4096;
  
  let key2: [u8; 32] = pbkdf2_hmac_array::<Sha256, 32>(password, salt, n);
  return Ok(key2);
}

pub fn encrypted_data(data: &[u8], user_key: &[u8; 32], user_nonce: &GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>) -> Result<Vec<u8>, aes_gcm::Error> {
  let key: &[u8; 32] = user_key;
  let key: &Key<Aes256Gcm> = key.into();
  let cipher: aes_gcm::AesGcm<aes::Aes256, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>> = Aes256Gcm::new(&key);
  let ciphertext: Vec<u8> = match cipher.encrypt(&user_nonce, data.as_ref()) {
    Ok(e) => e,
    Err(why) => panic!("Failed to encrypt ({})", why)
  };
  return Ok(ciphertext);
}


pub fn decrypted_data(data: Vec<u8>, user_key: &[u8; 32], user_nonce: &[u8; 12]) -> Result<String, Error> {
  let key: &[u8; 32] = user_key;
  let key: &Key<Aes256Gcm> = key.into();
  let cipher = Aes256Gcm::new(&key);

  let nonce: &GenericArray<u8, U12> = GenericArray::from_slice(user_nonce);

  let plaintext = match cipher.decrypt(nonce, &*data) {
      Ok(e) => e,
      Err(aead::Error) => {
        panic!("Decryption failed.");
      },
  };
  let result = match String::from_utf8(plaintext.to_vec()) {
      Ok(e) => e,
      Err(why) => panic!("Failed to convert decrypted data to readable string ({})", why),
  };
  return Ok(result);
}
