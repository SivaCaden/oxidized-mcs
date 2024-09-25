/*
 * Minecraft player class for Minecraft server based on https://wiki.vg/Protocol
 * Represents one Minecraft Player as stored in Database
 *
 * Authored by: Three rats in a trench coat
 *
*/

#[derive(Debug, Clone)]
pub struct Player {
    pub uuid: String,
    pub name: String,
    
}

impl Player {
    pub fn new(uuid: String, name: String) -> Player {
        Player {
            uuid,
            name,
        }
    }

}
