
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=static=SDL2");
        println!("cargo:rustc-link-search=native=libs/win_sdl2/lib");
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=static=SDL2");
        println!("cargo:rustc-link-search=native=libs/macos_sdl2/lib");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=SDL2"); // link Dinámico
        // Si querés ruta específica: 
        // println!("cargo:rustc-link-search=native=/usr/local/lib");
    } else {
        panic!("Plataforma no soportada");
    }
}
