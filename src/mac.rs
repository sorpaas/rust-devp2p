use aes::*;
use block_modes::{block_padding::NoPadding, BlockMode, Ecb};
use ethereum_types::{H128, H256};
use sha3::{Digest, Keccak256};

#[derive(Debug)]
pub struct MAC {
    secret: H256,
    hasher: Keccak256,
}

impl MAC {
    pub fn new(secret: H256) -> Self {
        Self {
            secret,
            hasher: Keccak256::new(),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.hasher.update(data)
    }

    pub fn update_header(&mut self, data: &[u8]) {
        let aes = Ecb::<_, NoPadding>::new(
            Aes256::new_varkey(self.secret.as_ref()).unwrap(),
            &Default::default(),
        );
        let mut encrypted = aes.encrypt_vec(self.digest().as_bytes());
        for i in 0..data.len() {
            encrypted[i] ^= data[i];
        }
        self.hasher.update(encrypted);
    }

    pub fn update_body(&mut self, data: &[u8]) {
        self.hasher.update(data);
        let prev = self.digest();
        let aes = Ecb::<_, NoPadding>::new(
            Aes256::new_varkey(self.secret.as_ref()).unwrap(),
            &Default::default(),
        );
        let mut encrypted = aes.encrypt_vec(self.digest().as_bytes());
        for i in 0..16 {
            encrypted[i] ^= prev[i];
        }
        self.hasher.update(encrypted);
    }

    pub fn digest(&self) -> H128 {
        H128::from_slice(&self.hasher.clone().finalize()[0..16])
    }
}
