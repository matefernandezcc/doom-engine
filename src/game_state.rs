use sdl2::TimerSubsystem;

///////////////////////////////// STRUCTS /////////////////////////////////
pub struct GameStateT {
    pub frame_start: u32, // Siempre es t: 0
    pub scrn_w: u32,
    pub scrn_h: u32,
    pub target_fps: f64,
    pub target_frame_time: f64,
    pub delta_time: f64,
    pub is_running: bool,
    pub is_paused: bool,
    pub is_fps_capped: bool,
    pub state_show_map: bool,
    pub is_debug_mode: bool,
}

impl GameStateT {
    // Constructor que inicializa un nuevo GameStateT
    pub fn new(scrnw: u32, scrnh: u32, target_fps: f64) -> Self {
        GameStateT {
            frame_start: 0,
            scrn_w: scrnw,
            scrn_h: scrnh,
            target_fps: target_fps as f64,
            target_frame_time: 1.0 / target_fps as f64,
            delta_time: 1.0 / target_fps as f64,
            is_running: true,
            is_paused: false,
            is_fps_capped: false,
            state_show_map: false,
            is_debug_mode: false,
        }
    }
}


///////////////////////////////// FUNCIONES ///////////////////////////////// 
pub fn frame_start(timer_subsystem: &TimerSubsystem, state: &mut GameStateT) {
    state.frame_start = timer_subsystem.ticks();
}

pub fn frame_end(timer_subsystem: &TimerSubsystem, state: &mut GameStateT) {
    let delta_time: f64 = (timer_subsystem.ticks() - state.frame_start) as f64 / 1000.0;
    state.delta_time = delta_time;

    if state.delta_time < state.target_frame_time {
        std::thread::sleep(std::time::Duration::from_secs_f64(state.target_frame_time - state.delta_time));
        state.delta_time = state.target_frame_time;
    }
}