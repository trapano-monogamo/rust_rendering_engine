#![allow(dead_code, unused_imports)]

use crate::*;
use crate::ecs::*;

#[derive(Default, Copy, Clone)]
struct Vertex {
    position: [f32;3],
    color: [f32;3],
    normal: [f32;3],
    tex_coord: [f32;2],
}
implement_vertex!(Vertex, position, color, normal, tex_coord);


/* ~ MESH ~
 * */
#[derive(Component)]
pub struct Mesh {
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
}

impl Mesh {
    pub fn triangle(display: &glium::Display) -> Self {
        let vertices: Vec<Vertex> = vec![
            Vertex { position: [-0.5, -0.5,  0.0], color: [1.0, 0.0, 0.0], normal: [ 0.0,  0.0,  0.0], tex_coord: [0.0, 0.0] },
            Vertex { position: [ 0.5, -0.5,  0.0], color: [0.0, 1.0, 0.0], normal: [ 0.0,  0.0,  0.0], tex_coord: [0.0, 0.0] },
            Vertex { position: [ 0.0,  0.5,  0.0], color: [0.0, 0.0, 1.0], normal: [ 0.0,  0.0,  0.0], tex_coord: [0.0, 0.0] },
        ];
        let indices: Vec<u16> = (0..3).collect();

        Self {
            vertex_buffer: glium::VertexBuffer::new(display, &vertices).unwrap(),
            index_buffer: glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap(),
        }
    }

    pub fn cube(display: &glium::Display) -> Self {
        //     4 --- 5
        //   / |   / |
        // 0 --- 1   |
        // |   6 |-- 7
        // | /   | /
        // 2 --- 3

        let vertices: Vec<Vertex> = vec![
            // 1
            Vertex { position: [-0.5,  0.5,  0.5], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, 1.0], tex_coord: [0.0, 0.0] }, // 0
            Vertex { position: [ 0.5,  0.5,  0.5], color: [0.0, 1.0, 0.0], normal: [0.0, 0.0, 1.0], tex_coord: [0.0, 0.0] }, // 1
            Vertex { position: [-0.5, -0.5,  0.5], color: [0.0, 0.0, 1.0], normal: [0.0, 0.0, 1.0], tex_coord: [0.0, 0.0] }, // 2
            Vertex { position: [-0.5, -0.5,  0.5], color: [1.0, 1.0, 0.0], normal: [0.0, 0.0, 1.0], tex_coord: [0.0, 0.0] }, // 2
            Vertex { position: [ 0.5,  0.5,  0.5], color: [1.0, 0.0, 1.0], normal: [0.0, 0.0, 1.0], tex_coord: [0.0, 0.0] }, // 1
            Vertex { position: [ 0.5, -0.5,  0.5], color: [0.0, 1.0, 1.0], normal: [0.0, 0.0, 1.0], tex_coord: [0.0, 0.0] }, // 3
            // 2
            Vertex { position: [ 0.5,  0.5,  0.5], color: [1.0, 0.0, 0.0], normal: [1.0, 0.0, 0.0], tex_coord: [0.0, 0.0] }, // 1
            Vertex { position: [ 0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0], normal: [1.0, 0.0, 0.0], tex_coord: [0.0, 0.0] }, // 5
            Vertex { position: [ 0.5, -0.5,  0.5], color: [0.0, 0.0, 1.0], normal: [1.0, 0.0, 0.0], tex_coord: [0.0, 0.0] }, // 3
            Vertex { position: [ 0.5, -0.5,  0.5], color: [1.0, 1.0, 0.0], normal: [1.0, 0.0, 0.0], tex_coord: [0.0, 0.0] }, // 3
            Vertex { position: [ 0.5,  0.5, -0.5], color: [1.0, 0.0, 1.0], normal: [1.0, 0.0, 0.0], tex_coord: [0.0, 0.0] }, // 5
            Vertex { position: [ 0.5, -0.5, -0.5], color: [0.0, 1.0, 1.0], normal: [1.0, 0.0, 0.0], tex_coord: [0.0, 0.0] }, // 7
            // 3
            Vertex { position: [ 0.5,  0.5, -0.5], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coord: [0.0, 0.0] }, // 5
            Vertex { position: [-0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coord: [0.0, 0.0] }, // 4
            Vertex { position: [ 0.5, -0.5, -0.5], color: [0.0, 0.0, 1.0], normal: [0.0, 0.0, -1.0], tex_coord: [0.0, 0.0] }, // 7
            Vertex { position: [ 0.5, -0.5, -0.5], color: [1.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coord: [0.0, 0.0] }, // 7
            Vertex { position: [-0.5,  0.5, -0.5], color: [1.0, 0.0, 1.0], normal: [0.0, 0.0, -1.0], tex_coord: [0.0, 0.0] }, // 4
            Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 1.0, 1.0], normal: [0.0, 0.0, -1.0], tex_coord: [0.0, 0.0] }, // 6
            // 4
            Vertex { position: [-0.5,  0.5, -0.5], color: [1.0, 0.0, 0.0], normal: [-1.0, 0.0, 0.0], tex_coord: [0.0, 0.0] }, // 4
            Vertex { position: [-0.5,  0.5,  0.5], color: [0.0, 1.0, 0.0], normal: [-1.0, 0.0, 0.0], tex_coord: [0.0, 0.0] }, // 0
            Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 0.0, 1.0], normal: [-1.0, 0.0, 0.0], tex_coord: [0.0, 0.0] }, // 6
            Vertex { position: [-0.5, -0.5, -0.5], color: [1.0, 1.0, 0.0], normal: [-1.0, 0.0, 0.0], tex_coord: [0.0, 0.0] }, // 6
            Vertex { position: [-0.5,  0.5,  0.5], color: [1.0, 0.0, 1.0], normal: [-1.0, 0.0, 0.0], tex_coord: [0.0, 0.0] }, // 0
            Vertex { position: [-0.5, -0.5,  0.5], color: [0.0, 1.0, 1.0], normal: [-1.0, 0.0, 0.0], tex_coord: [0.0, 0.0] }, // 2
            // 5
            Vertex { position: [-0.5,  0.5, -0.5], color: [1.0, 0.0, 0.0], normal: [0.0, 1.0, 0.0], tex_coord: [0.0, 0.0] }, // 4
            Vertex { position: [ 0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0], normal: [0.0, 1.0, 0.0], tex_coord: [0.0, 0.0] }, // 5
            Vertex { position: [-0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0], normal: [0.0, 1.0, 0.0], tex_coord: [0.0, 0.0] }, // 0
            Vertex { position: [-0.5,  0.5,  0.5], color: [1.0, 1.0, 0.0], normal: [0.0, 1.0, 0.0], tex_coord: [0.0, 0.0] }, // 0
            Vertex { position: [ 0.5,  0.5, -0.5], color: [1.0, 0.0, 1.0], normal: [0.0, 1.0, 0.0], tex_coord: [0.0, 0.0] }, // 5
            Vertex { position: [ 0.5,  0.5,  0.5], color: [0.0, 1.0, 1.0], normal: [0.0, 1.0, 0.0], tex_coord: [0.0, 0.0] }, // 1
            // 6
            Vertex { position: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0], normal: [0.0, -1.0, 0.0], tex_coord: [0.0, 0.0] }, // 2
            Vertex { position: [ 0.5, -0.5,  0.5], color: [0.0, 1.0, 0.0], normal: [0.0, -1.0, 0.0], tex_coord: [0.0, 0.0] }, // 3
            Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 0.0, 1.0], normal: [0.0, -1.0, 0.0], tex_coord: [0.0, 0.0] }, // 6
            Vertex { position: [-0.5, -0.5, -0.5], color: [1.0, 1.0, 0.0], normal: [0.0, -1.0, 0.0], tex_coord: [0.0, 0.0] }, // 6
            Vertex { position: [ 0.5, -0.5,  0.5], color: [1.0, 0.0, 1.0], normal: [0.0, -1.0, 0.0], tex_coord: [0.0, 0.0] }, // 3
            Vertex { position: [ 0.5, -0.5, -0.5], color: [0.0, 1.0, 1.0], normal: [0.0, -1.0, 0.0], tex_coord: [0.0, 0.0] }, // 7
        ];

        let indices: Vec<u16> = (0..36).collect();

        Self {
            vertex_buffer: glium::VertexBuffer::new(display, &vertices).unwrap(),
            index_buffer: glium::IndexBuffer::new( display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap(),
        }
    }
}


pub trait UniformsList: glium::uniforms::Uniforms { }


#[derive(Component)]
pub struct Shader {
    program: glium::program::Program,
    uniforms: Box<dyn glium::uniforms::Uniforms>,
}


// TODO: create default materials
#[derive(Component, Default)]
pub struct Material {
    pub shininess: f32,
    pub color: [f32;3],
    pub ambient: [f32;3],
    pub specular: [f32;3],
    pub diffuse: [f32;3],
}


#[derive(Component)]
pub struct Renderable {
    mesh: Mesh,
    material: Material,
    shader: glium::program::Program,
}

impl Renderable {
    pub fn triangle(
        vertex_shader_src: &str,
        fragment_shader_src: &str,
        display: &glium::Display
    ) -> Self {
        Self {
            mesh: Mesh::triangle(display),
            material: Material::default(),
            shader: glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap(),
        }
    }
    
    pub fn cube(
        vertex_shader_src: &str,
        fragment_shader_src: &str,
        display: &glium::Display
    ) -> Self {
        Self {
            mesh: Mesh::cube(display),
            material: Material::default(),
            shader: glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap(),
        }
    }
}


#[derive(Component)]
pub struct Light {
    pub position: [f32;3],
    pub direction: [f32;3],
    pub color: [f32;3],
}


#[derive(Component, Default, Debug)]
pub struct Transform {
    pub position: [f32;3],
    pub rotation: [f32;3],
    pub scale:    [f32;3],
}


/* Ideally, the mesh would be the base renderable component. Materials and shaders should be optional
 * and only serve the purpose of rendering more complex objects in different ways.
 * The shader struct should contain the program and it's uniforms, which should be built on demand.
 *
 *  pub fn render(mesh: &Mesh, material: Option<&Material>, shader: Option<&Shader>, target: &mut glium::Frame, _display: &glium::Display) {}
 * */

// TODO: uniforms
pub fn render(obj: &Renderable, target: &mut glium::Frame, _display: &glium::Display) {
    use glium::Surface;

    target.draw(
        &obj.mesh.vertex_buffer,
        &obj.mesh.index_buffer,
        &obj.shader,
        &glium::uniforms::EmptyUniforms,
        &Default::default()
    ).unwrap();
}
