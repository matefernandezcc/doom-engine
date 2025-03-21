use sdl2::VideoSubsystem;

///////////////////////////////// FUNCIONES  /////////////////////////////////
pub fn init(video_subsystem: &VideoSubsystem, winw: u32, winh: u32) {
    // Creación de la ventana
    video_subsystem.window("Doom Engine in Rust", winw, winh)
        .position_centered()
        .build()
        .unwrap();
}
 