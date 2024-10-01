
use std::sync::Mutex;
use rsa::{RsaPublicKey, RsaPrivateKey, Pkcs1v15Encrypt};
use pkcs8::EncodePublicKey;
use rand::thread_rng;
use std::sync::Arc;
use aes::Aes128;
use cfb8::{
    cipher::{AsyncStreamCipher, NewCipher},
    Cfb8
};
type Aes128Cfb8 = Cfb8<Aes128>;
type CKey = [u8; 16];

fn encode_public_key(public_key: RsaPublicKey) -> Vec<u8>{
    public_key
        .to_public_key_der()
        .as_ref()
        .expect("Failed to encode public key")
        .to_vec()
}

pub struct AesKeeper{
    buisness_end: Option<Mutex<Aes128Cfb8>>,
    iv: Option<CKey>,
}

impl AesKeeper{
    pub fn new() -> Self{
        AesKeeper{
            buisness_end: None,
            iv: None,
        }
    }
    pub fn enable_aes(&mut self, key: Vec<u8>){
        let cipher = Aes128Cfb8::new_from_slices(&key, &key).expect("keysize invalid");
        self.buisness_end = Some(Mutex::new(cipher));
        let key = key.as_slice();
        let mut iv: CKey = [0; 16];

        if key.len() != 16 || iv.len() != 16{
            panic!("invalid key length\nexpected 16 bytes\ngot {}", key.len())
        }
        
        for i in 0..16{
            iv[i] = key[i];
        }
        self.iv = Some(iv);
    }
    pub fn encrypt(&mut self, data: &mut Vec<u8>) -> Vec<u8>{
        let mut data = data.to_vec();

        if let Some(cipher_mutex) = &self.buisness_end {
            let mut cipher = cipher_mutex.lock().expect("Failed to lock mutex");
            cipher.encrypt(&mut data);
        }

        data
    }
    pub fn decrypt(&mut self, data: &mut Vec<u8>) -> Vec<u8>{
        let mut data = data.to_vec();

        if let Some(cipher_mutex) = &self.buisness_end {
            let mut cipher = cipher_mutex.lock().expect("Failed to lock mutex");
            cipher.decrypt(&mut data);
        }       
        data
    }
}
    

#[derive(Clone)]
pub struct KeyController{
    pub public_key: RsaPublicKey,
    pub ready_pkey: Vec<u8>,
    private_key: Arc<RsaPrivateKey>,
    pub use_aes: bool,
}

impl KeyController{
    pub fn new() -> Self{
        println!("Generating Keys");
        
        let mut rng = thread_rng(); 
        let bits = 1024;
        let private_key = RsaPrivateKey::new(&mut rng, bits)
            .expect("   Failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
        println!("Keys Generated");

        KeyController{
            public_key: public_key.clone(),
            ready_pkey: encode_public_key(public_key),
            private_key: Arc::new(private_key),
            use_aes: false,
        }
    }

    pub fn get_public_key(&self) -> RsaPublicKey{ self.public_key.clone() }

    pub fn get_der_key(&self) -> Vec<u8>{ self.ready_pkey.clone() }

    pub fn decrypt(&mut self, data: &mut Vec<u8>) -> Vec<u8>{

        if !self.use_aes{
            return self.decrypt_pgp(data.to_vec());
        } 
        return self.decrypt_aes(data);
    }

    pub fn decrypt_pgp(&self, data: Vec<u8>) -> Vec<u8>{

        let private_key = self.private_key.clone();
        let decrypted = match private_key.decrypt(Pkcs1v15Encrypt,&data) {
            Ok(data) => data,
            Err(e) => {
                println!("Failed to decrypt data: {:?}", e);
                vec![]
            }
        };
        decrypted
    }

    pub fn encrypt_aes(&mut self, data: &mut Vec<u8>) -> Vec<u8> {
       data.to_vec()
    }

    fn decrypt_aes(&mut self, data: &mut Vec<u8>) -> Vec<u8>{
        data.to_vec()
    }
}
