/*
 * WORLD/SOCKETS
 * =============
 */

use bevy_procedural_tilemaps::prelude::*;

pub struct TerrianSockets {
    pub dirt: DirtLayerSockets,
}

pub struct DirtLayerSockets {
    pub layer_up: Socket,
    pub layer_down: Socket,
    pub material: Socket,
}

pub fn create_sockets(socket_collection: &mut SocketCollection) -> TerrianSockets {
    let mut new_socket = || -> Socket { socket_collection.create() };

    let sockets = TerrianSockets {
        dirt: DirtLayerSockets {
            layer_up: new_socket(),
            material: new_socket(),
            layer_down: new_socket(),
        },
    };
    return sockets;
}
