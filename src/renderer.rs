use sdl2::{video::{Window, WindowContext}, VideoSubsystem, render::{Canvas, Texture, TextureCreator}, pixels::PixelFormatEnum, Sdl};
use crate::{game_state, player, typedefs, utils, window};


///////////////////////////////// STRUCTS /////////////////////////////////
pub struct RPlaneT {
    pub t: [i32; 1024],
    pub b: [i32; 1024],
}

pub struct WallT {
    pub a: typedefs::Vec2T,
    pub b: typedefs::Vec2T, 
    pub portal_top_height: f64,
    pub portal_bot_height: f64,
    pub is_portal: bool,
}

pub struct SectorT {
    pub id: i32,
    pub walls: [WallT; 10],
    pub num_walls: i32,
    pub height: i32,
    pub elevation: i32,
    pub dist: f64,
    pub color: u32,
    pub floor_clr: u32,
    pub ceil_clr: u32,

    pub portals_floorx_ylut: RPlaneT,
    pub portals_ceilx_ylut: RPlaneT,
    pub floorx_ylut: RPlaneT,
    pub ceilx_ylut: RPlaneT,
}

pub struct SectorsQueueT {
    pub sectors: [SectorT; 1024],
    pub num_sectors: i32,
}


///////////////////////////////// SCREEN  /////////////////////////////////
pub struct Screen { 
    pub screen_buffer: Vec<u32>, // pixeles
    pub screen_buffer_size: usize, // cant total de pixeles
}

impl Screen {
    // Crear un Screen vacio
    pub fn new() -> Self {
        Screen {
            screen_buffer: Vec::new(),
            screen_buffer_size: 0,
        }
    }

    // Iniciar el Screen y retornar la superficie de renderizado (Canvas)
    pub fn init_screen(&mut self, video_subsystem: &VideoSubsystem, w: u32, h: u32) -> Canvas<Window> {
        self.screen_buffer_size = (w * h) as usize; // Width*Height (Cantidad de pixeles de la ventana)
        self.screen_buffer = vec![0; self.screen_buffer_size]; // Inicia todo el vector en 0 con tamaño screen_buffer_size
        
        // Iniciar un WindowBuilder para crear una ventana
        let window: Window = video_subsystem.window("Engine", w, h)
            .position_centered()
            .build().unwrap();

        // Se crea el Canvas a partir de window
        let canvas: Canvas<Window> = window.into_canvas().accelerated().build().unwrap();
        
        canvas // Retorna el Canvas (lo hice asi para solucionar errores de lifetimes :D )
    }

    // Crear y cargar la textura con los datos de screen_buffer
    pub fn render(&mut self, canvas: &mut Canvas<Window>, w: u32, h: u32) {

        // Iniciar el texture_creator para poder usar el canvas
        let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

        // Crear textura RGBA32 (32 bits por pixel: 8 bits por canal rojo, verde, azul y alfa).
        match texture_creator.create_texture_streaming(PixelFormatEnum::RGBA32, w, h) {
            Ok(mut texture) => {
                // Actualizar la textura con el screen_buffer
                texture.update(None, unsafe { &self.screen_buffer.align_to::<u8>().1 }, (w * 4) as usize).unwrap();
                canvas.copy(&texture, None, None).unwrap();
                canvas.present();
            }
            Err(e) => {
                eprintln!("Error creating screen texture: {}", e);
                self.shutdown();
            }
        }
    }

    fn shutdown(&self) {
        eprintln!("Shutting down screen resources.");
    }
}

///////////////////////////////// FUNCIONES  /////////////////////////////////
pub fn r_init(video_subsystem: &VideoSubsystem, game_state: &game_state::GameStateT) {
    let scrnw: u32 = game_state.scrn_w / 2;
    let scrnh: u32 = game_state.scrn_h / 2;

    // Crear el Canvas y Renderer
    let mut screen: Screen = Screen::new(); // Se crea el Screen
    let mut canvas = screen.init_screen(video_subsystem, scrnw, scrnh);

    // Logical size
    if let Err(e) = canvas.set_logical_size(scrnw, scrnh) {
        eprintln!("Error al establecer el tamaño lógico: {}", e);
    }
}

pub fn r_update_screen(canvas: &mut Canvas<Window>, texture: &mut Texture, screen_buffer: &[u8], width: u32, height: u32) {
    let pitch = (width * 4) as usize;  // Asumimos que estamos usando 4 bytes por píxel (por ejemplo, RGBA32)
    
    // Llamamos a `update` en la textura para actualizar con los nuevos datos
    if let Err(e) = texture.update(None, screen_buffer, pitch) {
        eprintln!("Error al actualizar la textura: {}", e);
    }

    // Luego, copiamos la textura al canvas y la presentamos
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();
}

pub fn render(player: &player::PlayerT, game_state: &game_state::GameStateT){
    let is_debug_mode = game_state.is_debug_mode;
    r_update_screen(canvas, texture, screen_buffer, width, height);
}

//pub fn r_draw_walls(player: &player::PlayerT, game_state: &game_state::GameStateT){}

//pub fn r_create_sector(height: i32, elevation: i32, color: u32, ceil_clr: u32, floor_clr: u32) -> SectorT{}

//pub fn r_sector_add_wall(sector: &SectorT, vertices: WallT){}

//pub fn r_add_sector_to_queue(sector: &SectorT){}

//pub fn r_create_wall(ax: i32, ay: i32, bx: i32, by: i32) -> WallT{}

//pub fn r_create_portal(ax: i32, ay: i32, bx: i32, by: i32, th:i32, bh:i32) -> WallT{}