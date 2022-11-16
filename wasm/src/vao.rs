use std::{convert::TryInto};

use wasm_bindgen::JsValue;
use web_sys::{WebGlBuffer, WebGlVertexArrayObject, WebGl2RenderingContext};


/*
 * Simplified from https://github.com/jgcoded/webgl-rs-shooter/blob/main/src/vao.rs
 * 
 * Define a Vertex Array Object that
 * can be used with shaders that take
 * two attributes:
 *  * a vec2 of vertex position
 *  * a vec2 of vertex position
 *  */
 #[derive(Debug)]
pub struct VAO {
    square_buffer: WebGlBuffer,
    texture_buffer: WebGlBuffer,
    pub vao: WebGlVertexArrayObject
}


pub fn new_with_sprite_shader(
    gl: &WebGl2RenderingContext,
    vertex_position_attrib: i32,
    vertex_texture_attrib: i32
) -> Result<VAO, JsValue> {
    let square_buffer = create_square_buffer(&gl)?;
    let texture_buffer = create_texture_buffer(&gl)?;

    let vao = gl
        .create_vertex_array()
        .ok_or_else(|| String::from("Could not create VAO"))?;

    gl.bind_vertex_array(Some(&vao));
    let num_components = 2;
    let stride = 0;
    let offset = 0;
    let normalized = false;
    let buffer_type = WebGl2RenderingContext::FLOAT;
    {
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&square_buffer));
        gl.vertex_attrib_pointer_with_i32(
            vertex_position_attrib.try_into().unwrap(),
            num_components,
            buffer_type,
            normalized,
            stride,
            offset,
        );
        gl.enable_vertex_attrib_array(vertex_position_attrib.try_into().unwrap());
    }

    {
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&texture_buffer));
        gl.vertex_attrib_pointer_with_i32(
            vertex_texture_attrib.try_into().unwrap(),
            num_components,
            buffer_type,
            normalized,
            stride,
            offset,
        );
        gl.enable_vertex_attrib_array(vertex_texture_attrib.try_into().unwrap());

        Ok(VAO {
            square_buffer,
            texture_buffer,
            vao
        })
    }
}


pub fn delete(vao: &VAO, gl: &WebGl2RenderingContext) {
    gl.delete_buffer(Some(&vao.square_buffer));
    gl.delete_buffer(Some(&vao.texture_buffer));
    gl.delete_vertex_array(Some(&vao.vao));
}


pub fn create_buffer(
    gl: &WebGl2RenderingContext,
    data: js_sys::Float32Array,
) -> Result<WebGlBuffer, JsValue> {
    let buffer = gl.create_buffer().ok_or("Failed to create buffer")?;
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        WebGl2RenderingContext::ARRAY_BUFFER,
        &data,
        WebGl2RenderingContext::STATIC_DRAW,
    );

    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
    Ok(buffer)
}

pub fn create_square_buffer(gl: &WebGl2RenderingContext) -> Result<WebGlBuffer, JsValue> {
    let positions = [0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0];

    let data = js_sys::Float32Array::new_with_length(positions.len() as u32);
    data.copy_from(&positions);

    create_buffer(gl, data)
}

pub fn create_texture_buffer(gl: &WebGl2RenderingContext) -> Result<WebGlBuffer, JsValue> {
    let texture_coordinates = [0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0];

    let data = js_sys::Float32Array::new_with_length(texture_coordinates.len() as u32);
    data.copy_from(&texture_coordinates);

    create_buffer(gl, data)
}