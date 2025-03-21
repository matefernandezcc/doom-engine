#[derive(Clone)]
pub struct Vec2T {
    pub x: f64,
    pub y: f64,
}
    impl Vec2T {
        pub fn new(x: f64, y: f64) -> Self {
            Vec2T { x, y }
        }
    }