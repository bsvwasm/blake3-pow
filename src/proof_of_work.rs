use std::{borrow::Cow, panic::panic_any};

use blake3::Hasher;
use sha2::Digest;
use unroll::unroll_for_loops;
use wasm_bindgen::prelude::*;
use serde::*;
use primitive_types::U256;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfWork {
  hash: Vec<u8>,
  template: Vec<u8>,
  nonce: u64
}

#[wasm_bindgen]
impl ProofOfWork {
  pub fn get_hash(&self) -> Vec<u8> {
    self.hash.clone()
  }

  pub fn get_template(&self) -> Vec<u8> {
    self.template.clone()
  }

  pub fn get_nonce(&self) -> u64 {
    self.nonce
  }

  // pub fn to_json(&self) -> String {
  //   self.serialize(serializer)
  // }
}

#[wasm_bindgen]
#[derive(Debug)]
pub enum POWErrors {
  InvalidTarget,
  CantFindTarget
}

#[wasm_bindgen]
pub fn mine(template: &[u8], target_hex: &str) -> ProofOfWork {
  let target_u256 = U256::from_str_radix(target_hex, 16).unwrap();

  for nonce in 0..u64::MAX {
    let mut hasher = Hasher::new();
    hasher.update(template);
    hasher.update(&nonce.to_le_bytes());
    let hash = hasher.finalize();
    let hash_bytes = hash.as_bytes();
    let hash_value = U256::from_little_endian(hash_bytes);

    if hash_value < target_u256 {
      return ProofOfWork {
        hash: hash_bytes.to_vec(),
        template: template.to_vec(),
        nonce
      }
    }
  }


  panic!("{:#?}", POWErrors::CantFindTarget)
}

#[wasm_bindgen]
pub fn verify(pow: &ProofOfWork, target_hex: &str) -> bool {
  let target_u256 = U256::from_str_radix(target_hex, 16).unwrap();
  let mut hasher = Hasher::new();
  hasher.update(&pow.get_template());
  hasher.update(&pow.get_nonce().to_le_bytes());
  let hash = hasher.finalize();
  let hash_bytes = hash.as_bytes().to_vec();
  let hash_value = U256::from_little_endian(&hash_bytes);

  if hash_bytes != pow.get_hash() {
    return false;
  }

  if hash_value > target_u256 {
    return false;
  }

  true
}