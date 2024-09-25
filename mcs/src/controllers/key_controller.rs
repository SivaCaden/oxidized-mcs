
use rsa::{RsaPublicKey, RsaPrivateKey, Pkcs1v15Encrypt};
use pkcs8::EncodePublicKey;
use rand::thread_rng;
use std::sync::Arc;
use aes::Aes128;
use cipher::{AsyncStreamCipher, KeyIvInit,};
use cfb8;

type Aes128Cfb8Enc = cfb8::Encryptor<Aes128>;
type Aes128Cfb8Dec = cfb8::Decryptor<Aes128>;

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
    private_key: Arc<RsaPrivateKey>,
    pub use_aes: bool,
    aes_key: Option<Vec<u8>>,
    iv: Option<Vec<u8>>,
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
            private_key: Arc::new(private_key),
            use_aes: false,
            aes_key: None,
            iv: None,
        }
    }

    fn is_valid_aes(&self) -> bool{
        self.aes_key.is_some() || self.iv.is_some()
    }

    pub fn get_public_key(&self) -> RsaPublicKey{ self.public_key.clone() }

    pub fn get_der_key(&self) -> Vec<u8>{ self.ready_pkey.clone() }

    pub fn set_aes(&mut self, key: Vec<u8>){

        if key.len() != 16 {
            panic!("invalid key length\nexpected 16 bytes\ngot {}", key.len())
        }
        
        self.aes_key = Some(key.clone());
        self.iv = Some(key.clone());
        self.use_aes = true;


    }

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
        if !self.is_valid_aes() {
            panic!("AES Key or IV is not set");
        }

        let key = self.aes_key.as_ref().unwrap();
        let iv = self.iv.as_ref().unwrap();

        let encryptor = Aes128Cfb8Enc::new_from_slices(key, iv)
            .expect("Invalid Key or Iv Length");
        encryptor.encrypt(data);

        data.to_vec()
    }

    fn decrypt_aes(&mut self, data: &mut Vec<u8>) -> Vec<u8>{
        // decrypts data in place
        if !self.is_valid_aes() {
            panic!("AES key or IV is not set")
        }

        let key = self.aes_key.as_ref().unwrap();
        let iv = self.iv.as_ref().unwrap();

        let decryptor = Aes128Cfb8Dec::new_from_slices(key, iv)
            .expect("Invalid Key or Iv Length");


        decryptor.decrypt(data);

        data.to_vec()

    }
}
