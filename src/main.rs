use macroquad::{
    camera::{Camera3D, set_camera}, color::{BLACK, GREEN, PINK, WHITE}, input::{KeyCode, is_key_pressed}, math::{vec2, vec3}, models::{draw_cube, draw_grid}, prelude::{
        MaterialParams, 
        ShaderSource, 
        UniformDesc, 
        UniformType, 
        gl_use_default_material, 
        gl_use_material, 
        load_material, 
        render_target}, texture::{DrawTextureParams, FilterMode, draw_texture_ex}, window::{clear_background, next_frame, screen_height, screen_width}};

const FRAGMENT_SHADER: &str = include_str!("starfield_shader.glsl");

const VERTEX_SHADER: & str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying float iTime;

uniform mat4 Model;
uniform mat4 Projection;
uniform vec4 _Time;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    iTime = _Time.x;
}
";

#[macroquad::main("macro test")]
async fn main() {
    let direction_modifier: f32 = 0.0;
    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);
    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("iResolution", UniformType::Float2),
                UniformDesc::new("direction_modifier", UniformType::Float1),
            ],
            ..Default::default()
        },
    ).unwrap();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(BLACK);
        set_camera(&Camera3D{
            position: vec3(-20., 15., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        } );

        material.set_uniform("iResolution", (screen_width(), screen_height()));
        material.set_uniform("direction_modifier", direction_modifier);
        gl_use_material(&material);
        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();


        draw_grid(20, 1., WHITE, GREEN);

        draw_cube(vec3(2., 0., -3.), vec3(2., 2., 2.), None, PINK);
        next_frame().await;
    }
}
