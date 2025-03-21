use sdl2::{Sdl, VideoSubsystem, TimerSubsystem, EventPump};
mod typedefs; mod player; mod game_state; mod keyboard; mod window; mod renderer; mod utils;

///////////////////////////////// SDL Contextos /////////////////////////////////
pub struct SdlContextWrapper {
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub timer_subsystem: TimerSubsystem,
    pub event_pump: EventPump,
}

impl SdlContextWrapper {
    pub fn init() -> Result<Self, String> {
        let sdl_context: Sdl = sdl2::init()?;
        let video_subsystem: VideoSubsystem = sdl_context.video()?;
        let timer_subsystem: TimerSubsystem = sdl_context.timer()?;
        let event_pump: EventPump  = sdl_context.event_pump()?;

        Ok(SdlContextWrapper {
            sdl_context,
            video_subsystem,
            timer_subsystem,
            event_pump,
        })
    }
}

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

///////////////////////////////// MAIN /////////////////////////////////
fn game_loop() {
    println!("SDL2");
}
fn main() {
    // Iniciar instancias de SDL (para usar la biblioteca) ✔
    let sdl_wrapper: SdlContextWrapper = SdlContextWrapper::init().unwrap();

    // Keyboard IO ✔


    // Events handler ✔ 

    // Renderer ✔
    let mut screen = renderer::Screen::new();

    let mut canvas = screen.init_screen(&sdl_wrapper.sdl_context, 800, 600);
    screen.render(&mut canvas);

    // player

    // rendering context (window)
    
    //print_sdl_info(&sdl_wrapper.sdl_context, &sdl_wrapper.event_pump);
    //game_loop();
}