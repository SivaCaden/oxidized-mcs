
use pkcs1::EncodeRsaPublicKey;
use rsa::{RsaPublicKey, RsaPrivateKey, Pkcs1v15Encrypt};
use rand::thread_rng;
use std::sync::Arc;

#[derive(Clone)]
pub struct KeyController{
    pub public_key: RsaPublicKey,
    private_key: Arc<RsaPrivateKey>,
}

impl KeyController{
    pub fn new() -> Self{
        let mut rng = thread_rng();
        let bits = 1024;
        let private_key = RsaPrivateKey::new(&mut rng, bits)
            .expect("   Failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
        println!("Keys Generated");

        KeyController{
            public_key,
            private_key: Arc::new(private_key),
        }
    }
    pub fn get_public_key(&self) -> RsaPublicKey{
        self.public_key.clone()
    }

    pub fn encode_public_key(&self) -> Vec<u8>{
        let out_bytes = match self.public_key.clone().to_pkcs1_der() {
            Ok(document) => {
                let bytes = document.as_bytes().to_vec();
                bytes
            },
            Err(e) => {
                println!("ough {}", e);
                Vec::new() 
            }
        };
        return out_bytes;

    }

    pub fn encode(&self, data: Vec<u8>) -> Vec<u8>{
        self.public_key
            .encrypt(&mut thread_rng(), Pkcs1v15Encrypt, &data)
            .expect("Failed to encrypt data")
    }
    pub fn decode(&self, data: Vec<u8>) -> Vec<u8>{
        self.private_key
            .decrypt(Pkcs1v15Encrypt, &data)
            .expect("Failed to decrypt data")
    }
}
