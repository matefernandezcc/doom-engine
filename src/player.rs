use crate::typedefs;

pub struct PlayerT {
    pub position: typedefs::Vec2T,
    pub z: f64,
    pub dir_angle: f64,
}

impl PlayerT {
    // Constructor que inicializa un nuevo PlayerT
    pub fn new(x: f64, y: f64, z: f64, angle: f64) -> Self {
        PlayerT {
            position: typedefs::Vec2T { x, y },
            z,
            dir_angle: angle,
        }
    }
}