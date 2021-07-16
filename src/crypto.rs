use magic_crypt::MagicCrypt256;
use magic_crypt::MagicCryptError;
use magic_crypt::MagicCryptTrait;

/// encrypt data
pub struct Crypto {
  cipher: MagicCrypt256,
}

impl Crypto {
  pub fn new(key: &str) -> Self {
    Crypto {
      cipher: new_magic_crypt!(key, 256),
    }
  }

  pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
    self.cipher.encrypt_bytes_to_bytes(data)
  }

  pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, MagicCryptError> {
    self.cipher.decrypt_bytes_to_bytes(data)
  }
}
