use rand::Rng;

pub fn u_rand_range_ui(min: u32, max: u32) -> u32 {
    let mut rng = rand::rng();
    rng.random_range(min..max)  // Num aleatorio en el rango [min, max)
} 