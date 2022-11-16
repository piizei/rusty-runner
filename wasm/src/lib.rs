// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod graphics;
mod matrix;
mod texture;
mod utils;
mod vao;
mod vector;
mod webgl;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("hello 9");
    let image_url = String::from("/imgs/small_blue_0.png");
    let sprite_gl = webgl::initialize_web_gl2().unwrap_throw();
    //let texture = texture::load_texture(&sprite_gl.context, &image_url).unwrap_throw();
    //let sprite = graphics::new_sprite(&sprite_gl.context, texture).unwrap_throw();
    // graphics::draw_triangle(&sprite_gl.context, &sprite_gl.program);
}
