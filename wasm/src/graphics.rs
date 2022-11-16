use js_sys::WebAssembly;
use std::rc::Rc;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::{console, WebGl2RenderingContext, WebGlProgram, WebGlTexture};

use crate::vao::new_with_sprite_shader;
use crate::webgl::SpriteGl;

use crate::matrix::Mat4;
use crate::vector::Vec3;

const GAME_WIDTH: i32 = 320;
const GAME_HEIGHT: i32 = 200;

pub struct Sprite {
    texture: Rc<WebGlTexture>,
    mask: Rc<WebGlTexture>,
    pub color: [f32; 4],
    pub local_position: Vec3,
    pub global_scale: Vec3,
    pub global_position: Vec3,
    pub global_rotation: f32,
    model: Mat4,
}

pub fn render(sprite_gl: &SpriteGl, sprite: &Sprite) -> () {
       sprite_gl.context.uniform_matrix4fv_with_f32_array(
         Some(&sprite_gl.model_matrix_uniform),
         false,
         &sprite.model.data,
       );

       let vao = new_with_sprite_shader(&sprite_gl.context, sprite_gl.vertex_position_attrib, sprite_gl.vertex_texture_attrib);

       sprite_gl.context.bind_vertex_array(Some(&vao.unwrap_throw().vao));

       sprite_gl.context.active_texture(WebGl2RenderingContext::TEXTURE0);
       sprite_gl.context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&sprite.texture));
       sprite_gl.context.uniform1i(Some(&sprite_gl.texture_sampler_uniform), 0);

       // sprite_gl.context.active_texture(WebGl2RenderingContext::TEXTURE1);
       // sprite_gl.context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&sprite.mask));
       // sprite_gl.context.uniform1i(Some(&sprite_gl.mask_sampler_uniform), 1);

       sprite_gl.context.uniform4fv_with_f32_array(Some(&sprite_gl.color_uniform), &sprite.color);

       {
           let offset = 0;
           let vertex_count = 6;
           sprite_gl.context.draw_arrays(WebGl2RenderingContext::TRIANGLES, offset, vertex_count);
           console::log_2(&"gl".into(), &format!("Hello, {:?}!", &sprite_gl.context).into());
       }
    
    //sprite_gl.context.bind_vertex_array(None);
}

pub fn new_sprite(
    gl: &WebGl2RenderingContext,
    texture: Rc<WebGlTexture>,
) -> Result<Sprite, JsValue> {
    let mask_array = [255, 255, 255, 255];
    let mask = crate::texture::create_rgba_texture_from_u8_array(&gl, 1, 1, &mask_array)?;
    Ok(Sprite {
        texture,
        mask,
        color: [1.0, 1.0, 1.0, 0.5],
        local_position: Vec3::new(10.0, 10.0, 10.0),
        global_scale: Vec3::new(1.0, 1.0, 1.0),
        global_position: Vec3::new(0.0, 0.0, 0.0),
        global_rotation: 0.0,
        model: Mat4::identity(),
    })
}

