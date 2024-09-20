/*
 * Main file for a Rust-based Minecraft Server (WIP)
 * based entirely on https://wiki.vg/Protocol
 * Authored by: Three rats in a trench coat
 *              (Caden Siva, Foster Sullivan, and the one brain cell)
 *
 * File Structure is/will be as follows:
 * src
 * | main.rs - this file (spools server)
 * | server.rs - primary server router 
 * |
 * |--- util - directory for various utility libraries
 * |    | mod.rs - links submodules together
 * |    | datatypes.rs - Converts hex packets into Rust-friendly objects
 * |    | vector.rs - Basic 2D and 3D Vector library
 * |    | packet.rs - Class describing a packet, with methods to encode/decode packets.
 * |
 * |--- models - directory for database models
 * |    | mod.rs - links submodules together
 * |    | block.rs - Block class representing a minecraft block  
 * |    | player.rs - Player class representing a minecraft player
 * |    | entity.rs - Entity class representing a minecraft entity
 * |    | world.rs - World class representing the minecraft world
 * |    | config.rs - Config class representing the minecraft config
 * |
 * |--- controllers - directory for controllers to be routed to and to control
 * |    |             database models to store data
 * |    | mod.rs - links submodules together
 * |    | game_control.rs - "Game" state packets go here (may be split later)
 * |    | login_control.rs - "Login" state packets go here
 * |    | handshake_control.rs - "Handshake" state packets go here 
 * |    | config_control.rs - "Config" state packets go here 
 * |    | status_control.rs - "Status" state packets go here (may be merged with HS later)
 * |    | key_controller.rs - Controller for generating and handeling cryptographic keys 
*/

// Including all directories in crate hierarchy
pub mod util;
pub mod controllers{ pub mod handshake; pub mod status; pub mod key_controller; pub mod login;}
pub mod models;
pub mod server;
pub mod tests;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("running server");
    server::run().await;
    println!("bs after server");

    Ok(())
}
