mod typedefs; mod player; mod game_state; mod keyboard; mod window; mod renderer; mod utils;
use core::f64;
use player::PlayerT;
use game_state::GameStateT;
use keyboard::{KeymapT, KeystatesT};
use renderer::{SectorT, WallT};
use sdl2::{EventPump, Sdl, TimerSubsystem, VideoSubsystem};

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

///////////////////////////////// MAIN /////////////////////////////////
fn game_loop(mut context: SdlContextWrapper, mut game_state: GameStateT, mut player: PlayerT, mut keymap: KeymapT, mut keystates: KeystatesT, w:u32, h:u32){
    let mut screen: renderer::Screen = renderer::Screen::new();
    let mut canvas = screen.init_screen(&context.video_subsystem, w, h);
    while game_state.is_running {
        game_state::frame_start(&context.timer_subsystem, &mut game_state);
        keyboard::handle_events(&mut context.event_pump, &mut keymap, &mut keystates, &mut game_state, &mut player);
        screen.render(&mut canvas, w, h);
        //renderer::render(&player, &game_state);
        game_state::frame_end(&context.timer_subsystem, &mut game_state);
    }
}
fn main() {
    let width: u32 = 1024;
    let height: u32 = 768;
    let target_fps: f64 = 120.0;

    // Iniciar instancias de SDL (para usar la biblioteca)
    let sdl_wrapper: SdlContextWrapper = SdlContextWrapper::init().unwrap();
    let game_state: game_state::GameStateT = game_state::GameStateT::new(width, height, target_fps);
    let player: PlayerT = PlayerT::new(40.0, 40.0, (height * 10) as f64, f64::consts::PI/2.0);

    // Keyboard IO
    let keymap: KeymapT = keyboard::KeymapT::new();
    let keystates: KeystatesT = keyboard::KeystatesT::new();

    // Window & Render init 
    window::init(&sdl_wrapper.video_subsystem, width, height);
    renderer::init(&sdl_wrapper.video_subsystem, &game_state);

    // Sectores 
    let mut s1: SectorT = SectorT::new(10, 5, 0xFF00FF, 0x0000FF, 0x00FF00);
    let wall1 = WallT::new(0.0, 0.0, 10.0, 0.0, 5.0, 10.0, false);
    let wall2 = WallT::new(10.0, 0.0, 10.0, 10.0, 5.0, 10.0, false);
    
    game_loop(sdl_wrapper, game_state, player, keymap, keystates, width, height);
}