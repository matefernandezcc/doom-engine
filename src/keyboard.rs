use core::f64;
use sdl2::{event::Event, keyboard::{Scancode, Keycode}, EventPump};
use crate::{game_state, player::{self, PlayerT}};

// Velocidades predeterminadas
pub struct SpeedT {
    pub mov_speed: f64,
    pub elevation_speed: f64,
    pub rot_speed: f64,
}
impl SpeedT {
    pub fn new() -> Self {
        SpeedT { 
            mov_speed: 150.0,
            elevation_speed: (500*100) as f64,
            rot_speed: 4.0,
        }
    }
}
///////////////////////////////// STRUCTS /////////////////////////////////
pub struct KeymapT {
    pub forward: Scancode,
    pub backward: Scancode,
    pub left: Scancode,
    pub right: Scancode,
    pub quit: Scancode,
    pub strafe_left: Scancode,
    pub strafe_right: Scancode,
    pub up: Scancode,
    pub down: Scancode,
    pub toggle_map: Scancode,
    pub debug_mode: Scancode,
}
impl KeymapT {
    pub fn new() -> Self {
        KeymapT {
            forward: Scancode::W,
            backward: Scancode::S,
            left: Scancode::A,
            right: Scancode::D,
            quit: Scancode::Escape,
            strafe_left: Scancode::Q,
            strafe_right: Scancode::E,
            up: Scancode::Space,
            down: Scancode::LCtrl,
            toggle_map: Scancode::M,
            debug_mode: Scancode::O,
        }
    }
}

pub struct KeystatesT {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub strafe_left: bool,
    pub strafe_right: bool,
    pub up: bool,
    pub down: bool,
    pub map_state: bool,
    pub is_debug: bool,
}
impl KeystatesT {
    pub fn new() -> Self {
        KeystatesT {
            forward: false,
            backward: false,
            left: false,
            right: false,
            strafe_left: false,
            strafe_right: false,
            up: false,
            down: false,
            map_state: false,
            is_debug: false,
        }
    }
}

#[derive(PartialEq)] // Esto es para poder usar el operador binario '==' con el enum
pub enum KbdKeyState {
    Up,
    Down,
}


///////////////////////////////// FUNCIONES /////////////////////////////////

// Procesar el estado de las teclas (actualiza el estado de 'KeystatesT')
pub fn k_process_keystates( 
    keystates: &mut KeystatesT,
    player: &mut player::PlayerT,
    delta_time: f64
) {
    let speed_config: SpeedT = SpeedT::new();

    // Movimientos hacia adelante (+) y atrás (-)
    if keystates.forward {
        player.position.x += speed_config.mov_speed * f64::cos(player.dir_angle) * delta_time;
        player.position.y += speed_config.mov_speed * f64::sin(player.dir_angle) * delta_time;
    }
    else if keystates.backward {
        player.position.x -= speed_config.mov_speed * f64::cos(player.dir_angle) * delta_time;
        player.position.y -= speed_config.mov_speed * f64::sin(player.dir_angle) * delta_time;
    }

    // Movimientos hacia izquierda (giro +) y derecha (giro -)
    if keystates.left {
        player.dir_angle += speed_config.rot_speed * delta_time;
    }
    else if keystates.right {
        player.dir_angle -= speed_config.rot_speed * delta_time;
    }

    // Movimientos strafes hacia izquierda y derecha
    if keystates.strafe_left {
        player.position.x += speed_config.mov_speed * f64::cos(player.dir_angle + f64::consts::PI/2.0) * delta_time;
        player.position.y += speed_config.mov_speed * f64::sin(player.dir_angle + f64::consts::PI/2.0) * delta_time;
    }
    else if keystates.strafe_right {
        player.position.x -= speed_config.mov_speed * f64::cos(player.dir_angle + f64::consts::PI/2.0) * delta_time;
        player.position.y -= speed_config.mov_speed * f64::sin(player.dir_angle + f64::consts::PI/2.0) * delta_time;
    }

    // Movimientos hacia arriba (+) y abajo (-)
    if keystates.up {
        player.z += speed_config.elevation_speed * delta_time;
    }
    else if keystates.down {
        player.z -= speed_config.elevation_speed * delta_time;
    }
}

// Manejar las teclas en tiempo real (presionar o soltar)
pub fn k_handle_realtimekeys(
    keymap: &mut KeymapT,
    keystates: &mut KeystatesT,
    key_scancode: &Scancode,
    state: KbdKeyState
){
    if *key_scancode == keymap.forward {
        keystates.forward = state == KbdKeyState::Down;
    } else if *key_scancode == keymap.backward {
        keystates.backward = state == KbdKeyState::Down;
    }

    if *key_scancode == keymap.left {
        keystates.left = state == KbdKeyState::Down;
    } else if *key_scancode == keymap.right {
        keystates.right = state == KbdKeyState::Down;
    }

    if *key_scancode == keymap.strafe_left {
        keystates.strafe_left = state == KbdKeyState::Down;
    } else if *key_scancode == keymap.strafe_right {
        keystates.strafe_right = state == KbdKeyState::Down;
    }

    if *key_scancode == keymap.up {
        keystates.up = state == KbdKeyState::Down;
    } else if *key_scancode == keymap.down {
        keystates.down = state == KbdKeyState::Down;
    }

    if *key_scancode == keymap.debug_mode && state == KbdKeyState::Down {
        keystates.is_debug = !keystates.is_debug;
    }
}

// Manejar Eventos del teclado
pub fn handle_events(
    event_pump: &mut EventPump,
    keymap: &mut KeymapT, 
    keystates: &mut KeystatesT, 
    game_state: &mut game_state::GameStateT, 
    player: &mut player::PlayerT
) {
    for event in event_pump.poll_iter() {
        match event {
            // Evento KeyDown
            Event::KeyDown { scancode: Some(scancode), .. } => {
                k_handle_realtimekeys(keymap, keystates,&scancode, KbdKeyState::Down);
                game_state.state_show_map = keystates.map_state;
                
                if scancode == keymap.quit {
                    game_state.is_running = false;
                }

                if scancode == keymap.debug_mode {
                    game_state.is_debug_mode = !game_state.is_debug_mode;
                }
                break;
            }

            // Evento KeyUp
            Event::KeyUp { scancode: Some(scancode), .. } => {
                k_handle_realtimekeys(keymap, keystates,&scancode, KbdKeyState::Up);
                break;
            }

            Event::Quit { .. } => {
                game_state.is_running = false;
                break;
            }
            _ => {break;}
        }
    }
    k_process_keystates(keystates, player, game_state.delta_time);
}