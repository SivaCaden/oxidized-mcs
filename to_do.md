## Housekeeping / Refacctor

- [ ] Move handle_connection fn from main.rs to server.rs
- [ ] Remove Packet struct from main.rs to util/packet.rs
- [ ] Restructure the State enum, potentially move out to a util or in server.rs

## Implement TDD for different util modules

- [ ] Datatypes.rs
- [ ] Packet.rs (crafter/decoder)

## Implement TDD for DB Objects

- [ ] Config.rs
- [ ] Block.rs
- [ ] Entity.rs
- [ ] World.rs
- [ ] Player.rs

## Discuss which DB to use

- MongoDB (noSQL)
- PostgreSQL
- Potentially GraphQL

## Discuss how to structure packet obj

- Opt 1: Packet trait implemented over other specific types       (i.e. HandshakePacket implements Packet)
- Opt 2: General Packet type genericized with an enum type        (i.e. enum State{ Handshake, ...}; Packet<State::Handshake>)
- Opt 3: Store Packet State as a piece of data on the Packet type (i.e. enum State{ Handshake, ...}; struct Packet { state: State, ... }; packet1.get_state();}

Divide and conquer!
