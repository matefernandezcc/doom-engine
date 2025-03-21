use rand::Rng;
use sdl2::Sdl;


///////////////////////////////// Algunas funciones utiles /////////////////////////////////

// Genera un numero u32 aleatorio
pub fn u_rand_range_ui(min: u32, max: u32) -> u32 {
    let mut rng = rand::rng();
    rng.random_range(min..max)  // Num aleatorio en el rango [min, max)
} 


// Muestra por pantalla si los contextos de SDL fueron bien inicializados
fn print_sdl_info(sdl_context: &Sdl, _event_pump: &sdl2::EventPump) {
    // Subsistema de video
    match sdl_context.video() {
        Ok(video) => println!("Video Subsystem Initialized: {:?}", video),
        Err(e) => println!("Failed to initialize Video subsystem: {}", e),
    }

    // Subsistema timer
    match sdl_context.timer() {
        Ok(timer) => println!("Timer Subsystem Initialized: {:?}", timer),
        Err(e) => println!("Failed to initialize Timer subsystem: {}", e),
    }

    // Confirmar que el event_pump está disponible
    println!("Event pump is initialized and active.");
}