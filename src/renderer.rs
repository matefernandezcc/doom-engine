use sdl2::{video::{Window, WindowContext}, VideoSubsystem, render::{Canvas, Texture, TextureCreator}, pixels::PixelFormatEnum};
use crate::{game_state, player, typedefs};


///////////////////////////////// STRUCTS /////////////////////////////////
#[derive(Clone)]
pub struct RPlaneT {
    pub t: [i32; 1024],
    pub b: [i32; 1024],
}

#[derive(Clone)]
pub struct WallT {
    pub a: typedefs::Vec2T,
    pub b: typedefs::Vec2T, 
    pub portal_top_height: f64,
    pub portal_bot_height: f64,
    pub is_portal: bool,
}
    impl WallT {
        pub fn new(x1:f64, y1:f64, x2:f64, y2:f64, portal_top_height:f64, portal_bot_height:f64, is_portal:bool) -> Self {
            WallT { 
                a: typedefs::Vec2T::new(x1,y1), 
                b: typedefs::Vec2T::new(x2,y2),
                portal_top_height: portal_top_height, 
                portal_bot_height: portal_bot_height, 
                is_portal: is_portal,
            }
        }
    }

#[derive(Clone)]
pub struct SectorT {
    pub id: i32,
    pub walls: Vec<WallT>,
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
    impl SectorT {
        pub fn new(height: i32, elevation: i32, color: u32, ceil_clr: u32, floor_clr: u32) -> Self {
            SectorT {
                id: 0,
                walls: Vec::new(),  // Vec para manejar un número dinámico de paredes
                num_walls: 0,
                height,
                elevation,
                dist: 0.0,
                color,
                floor_clr,
                ceil_clr,
    
                portals_floorx_ylut: RPlaneT { t: [0; 1024], b: [0; 1024] },
                portals_ceilx_ylut: RPlaneT { t: [0; 1024], b: [0; 1024] },
                floorx_ylut: RPlaneT { t: [0; 1024], b: [0; 1024] },
                ceilx_ylut: RPlaneT { t: [0; 1024], b: [0; 1024] },
            }
        }

        pub fn add_wall(&mut self, wall: WallT) {
            if self.num_walls < 10 {
                self.walls[self.num_walls as usize] = wall;
                self.num_walls += 1;
            }
        }
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
pub fn init(video_subsystem: &VideoSubsystem, game_state: &game_state::GameStateT) {
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

pub fn update_screen(canvas: &mut Canvas<Window>, texture: &mut Texture, screen_buffer: &[u8], width: u32) {
    let pitch = (width * 4) as usize;  // 4 bytes por píxel (RGBA32)
    
    // Llamamos a `update` en la textura para actualizar con los nuevos datos
    if let Err(e) = texture.update(None, screen_buffer, pitch) {
        eprintln!("Error al actualizar la textura: {}", e);
    }

    // Copiamos la textura al canvas y la presentamos
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();
}

pub fn render(player: &player::PlayerT, game_state: &game_state::GameStateT){
    let is_debug_mode = game_state.is_debug_mode;
    //update_screen(canvas, texture, screen_buffer, width, height);
}

pub fn draw_walls(game_state: &game_state::GameStateT, canvas: &mut Canvas<Window>) {
    if !game_state.sectors.is_empty() {
        // Iterar sobre cada sector
        for sector in &game_state.sectors {
            // Iterar sobre las paredes de cada sector
            for wall in &sector.walls {
                // Calcular la proyección de las paredes en la pantalla
                // Para la simplicidad, consideramos que las paredes son líneas
                let x1 = wall.a.x as i32;
                let y1 = wall.a.y as i32;
                let x2 = wall.b.x as i32;
                let y2 = wall.b.y as i32;
                
                // Dibujar la pared
                canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255)); // Blanco para las paredes
                if let Err(e) = canvas.draw_line((x1, y1), (x2, y2)) {
                    eprintln!("Error al dibujar la línea: {}", e);
                }
            }
        }
    } else {
        eprintln!("No hay sectores en el game state");
    }
}

pub fn sector_add_wall(sector: &mut SectorT, wall: WallT) {
    // Añadir la pared a la lista de paredes del sector
    sector.add_wall(wall);
}

pub fn add_sector_to_queue(sector: &mut SectorT, sectors_queue: &mut SectorsQueueT) {
    if sectors_queue.num_sectors < 1024 {
        sectors_queue.sectors[sectors_queue.num_sectors as usize] = sector.clone();
        sectors_queue.num_sectors += 1;
    } else {
        eprintln!("La cola de sectores está llena");
    }
}

pub fn create_wall(ax: f64, ay: f64, bx: f64, by: f64, portal_top_height: f64, portal_bot_height: f64, is_portal: bool) -> WallT {
    WallT::new(ax, ay, bx, by, portal_top_height, portal_bot_height, is_portal)
}

pub fn create_portal(ax: i32, ay: i32, bx: i32, by: i32, top_height: i32, bot_height: i32) -> WallT {
    WallT::new(ax as f64, ay as f64, bx as f64, by as f64, top_height as f64, bot_height as f64, true)
}