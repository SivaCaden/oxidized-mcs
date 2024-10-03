/*
 * Minecraft player class for Minecraft server based on https://wiki.vg/Protocol
 * Represents one Minecraft Player as stored in Database
 *
 * Authored by: Three rats in a trench coat
 *
*/

use crate::controllers::key_controller::AesKeeper;

pub struct Player {
    pub uuid: String,
    pub name: String,
    pub aes_keeper: Option<AesKeeper>,
    pub verify_token: Option<Vec<u8>>,
    
}

impl Player {
    pub fn new(uuid: String, name: String) -> Player {
        Player {
            uuid,
            name,
            aes_keeper: None,
            verify_token: None,
        }
    }
    pub fn set_verify_token(&mut self, token: Vec<u8>) {
        self.verify_token = Some(token);
    }
    pub fn get_verify_token(&self) -> Vec<u8> {
        match &self.verify_token {
            Some(token) => token.clone(),
            None => Vec::new(),
        }
    }

    pub fn set_aes(&mut self, key: Vec<u8>) {
        self.aes_keeper = Some(AesKeeper::new());
        if let Some(aes_keeper) = &mut self.aes_keeper {
            aes_keeper.enable_aes(key);
        }
    }
}
