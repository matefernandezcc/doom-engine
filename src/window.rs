use sdl2::{video::Window, VideoSubsystem};

///////////////////////////////// FUNCIONES  /////////////////////////////////
pub fn w_init(video_subsystem: &VideoSubsystem, winw: u32, winh: u32) {
    // Creación de la ventana
    let window: Window = video_subsystem.window("Doom Engine in Rust", winw, winh)
        .position_centered()
        .build()
        .unwrap();
}
