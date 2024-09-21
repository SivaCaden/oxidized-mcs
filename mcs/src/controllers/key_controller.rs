
use rsa::{RsaPublicKey, RsaPrivateKey};
use pkcs8::EncodePublicKey;
use rand::thread_rng;
use std::sync::Arc;


fn encode_public_key(public_key: RsaPublicKey) -> Vec<u8>{
    public_key.to_public_key_der().as_ref().expect("Failed to encode public key").to_vec()
}

#[derive(Clone)]
pub struct KeyController{
    pub public_key: RsaPublicKey,
    pub ready_pkey: Vec<u8>,
    _private_key: Arc<RsaPrivateKey>,
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

}
