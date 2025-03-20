use crate::typedefs;

pub struct PlayerT {
    pub position: typedefs::Vec2T,
    pub z: f64,
    pub dir_angle: f64,
}

pub fn p_init(x:f64, y:f64, z:f64, angle:f64) -> PlayerT{
    let player: PlayerT = PlayerT {
        position: typedefs::Vec2T {x,y},
        z,
        dir_angle: angle,
    };
    player
}