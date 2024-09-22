
use rsa::{RsaPublicKey, RsaPrivateKey, Pkcs1v15Encrypt};
use pkcs8::EncodePublicKey;
use rand::{rngs::StdRng, thread_rng, SeedableRng};
use std::sync::Arc;


fn encode_public_key(public_key: RsaPublicKey) -> Vec<u8>{
    public_key
        .to_public_key_der()
        .as_ref()
        .expect("Failed to encode public key")
        .to_vec()
}

#[derive(Clone)]
pub struct KeyController{
    pub public_key: RsaPublicKey,
    pub ready_pkey: Vec<u8>,
    _private_key: Arc<RsaPrivateKey>,
}

impl KeyController{
    pub fn new() -> Self{
        let seed: [u8; 32] = [0; 32];
        
        let mut rng = StdRng::from_seed(seed);
        let bits = 1024;
        let private_key = RsaPrivateKey::new(&mut rng, bits)
            .expect("   Failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
        println!("Keys Generated");

        KeyController{
            public_key: public_key.clone(),
            ready_pkey: encode_public_key(public_key),
            _private_key: Arc::new(private_key),
        }
    }
    pub fn get_public_key(&self) -> RsaPublicKey{
        self.public_key.clone()
    }
    pub fn get_der_key(&self) -> Vec<u8>{
        self.ready_pkey.clone()
    }
    pub fn decrypt(&self, data: Vec<u8>) -> Vec<u8>{
        let private_key = self._private_key.clone();
        let decrypted = private_key
            .decrypt(Pkcs1v15Encrypt,&data)
            .expect("Failed to decrypt data");
        decrypted
    }

}
