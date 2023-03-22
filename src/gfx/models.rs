use super::glutils::{self, *};
use super::shaders::Shaders;
use gl33::*;
use russimp;
use russimp::material::TextureType;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use ultraviolet::*;

fn offset_of<B, T>(base: &B, val: &T) -> usize {
    let base = base as *const _ as usize;
    val as *const _ as usize - base
}

#[derive(Default)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec2,
}

#[derive(Default)]
pub enum TexType {
    #[default]
    Diffuse,
    Specular,
}

#[derive(Default)]
pub struct Texture {
    pub id: u32,
    pub tex_type: TexType,
}

#[derive(Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    pub gl_vao: u32,
    pub gl_vbo: u32,
    pub gl_ebo: u32,
}

impl Mesh {
    pub fn prepare_tex(&self, gl: &GlFns, shader: &Shaders) {
        let mut diffuse_nr = 1;
        let mut specular_nr = 1;

        for (i, t) in self.textures.iter().enumerate() {
            unsafe {
                let tex_id: gl33::GLenum = GLenum(gl33::GL_TEXTURE0.0 + i as u32);
                gl.ActiveTexture(tex_id);
            }

            let uniform_name;
            match t.tex_type {
                TexType::Diffuse => {
                    uniform_name = format!("texture_diffuse{}", diffuse_nr);
                    diffuse_nr += 1;
                }
                TexType::Specular => {
                    uniform_name = format!("texture_specular{}", specular_nr);
                    specular_nr += 1;
                }
            }

            shader.try_set_i32(gl, &uniform_name.to_string(), i as i32);
            unsafe {
                gl.BindTexture(gl33::GL_TEXTURE_2D, t.id);
            }
        }
        unsafe {
            gl.ActiveTexture(GL_TEXTURE0);
        }
    }

    pub fn draw(&self, gl: &GlFns, shader: &Shaders) {
        self.prepare_tex(gl, shader);
        // draw mesh
        gl.BindVertexArray(self.gl_vao);
        unsafe {
            gl.DrawElements(
                gl33::GL_TRIANGLES,
                self.indices.len() as i32,
                gl33::GL_UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
    pub fn setup_mesh(&mut self, gl: &GlFns) -> Result<(), String> {
        unsafe {
            gl.GenVertexArrays(1, &mut self.gl_vao);
            if self.gl_vao == 0 {
                return Err("failed setup_mesh: gl.GenVertexArrays(1, &mut gl_vao)".to_string());
            }

            gl.GenBuffers(1, &mut self.gl_vbo);
            if self.gl_vbo == 0 {
                return Err("failed setup_mesh: gl.GenBuffers(1, &mut gl_vbo)".to_string());
            }

            gl.GenBuffers(1, &mut self.gl_ebo);
            if self.gl_ebo == 0 {
                return Err("failed setup_mesh: gl.GenBuffers(1, &mut gl_ebo)".to_string());
            }

            gl.BindVertexArray(self.gl_vao);
            check_gl_err(gl);

            gl.BindBuffer(GL_ARRAY_BUFFER, self.gl_vbo);
            gl_buffer_data_arr_stat(gl, &self.vertices);
            check_gl_err(gl);

            gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.gl_ebo);
            gl_buffer_data_element_stat(gl, &self.indices);
            check_gl_err(gl);

            let vert_size = std::mem::size_of::<Vertex>() as i32;
            // position attribute
            gl.VertexAttribPointer(
                0,
                3,
                GL_FLOAT,
                0, //gl33::GL_FALSE,
                vert_size,
                (offset_of(&self.vertices[0], &self.vertices[0].position)) as *const _,
            );
            gl.EnableVertexAttribArray(0);
            check_gl_err(gl);

            // normals attribute
            gl.VertexAttribPointer(
                1,
                3,
                GL_FLOAT,
                0, //gl33::GL_FALSE,
                vert_size,
                (offset_of(&self.vertices[0], &self.vertices[0].normal)) as *const _,
            );
            gl.EnableVertexAttribArray(1);
            check_gl_err(gl);

            // tex coords attribute
            gl.VertexAttribPointer(
                2,
                2,
                GL_FLOAT,
                0, //gl33::GL_FALSE,
                vert_size,
                (offset_of(&self.vertices[0], &self.vertices[0].tex_coords)) as *const _,
            );
            gl.EnableVertexAttribArray(2);
            check_gl_err(gl);
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    texture_cache: HashMap<String, u32>,
    path: String,
}

impl Model {
    pub fn from(gl: &GlFns, filename: &str) -> Result<Self, String> {
        let scene = russimp::scene::Scene::from_file(
            filename,
            vec![
                // russimp::scene::PostProcess::CalculateTangentSpace,
                russimp::scene::PostProcess::Triangulate,
                russimp::scene::PostProcess::FlipUVs,
                russimp::scene::PostProcess::JoinIdenticalVertices,
                // russimp::scene::PostProcess::SortByPrimitiveType,
            ],
        )
        .unwrap();

        let mut model = Model {
            path: Path::new(filename)
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            ..Default::default()
        };

        model.process_node(gl, scene.root.as_ref().unwrap().clone(), &scene);

        Ok(model)
    }

    fn process_node(
        &mut self,
        gl: &GlFns,
        node: Rc<RefCell<russimp::node::Node>>,
        scene: &russimp::scene::Scene,
    ) {
        for mid in node.borrow().meshes.iter() {
            let mesh = &scene.meshes[*mid as usize];
            self.process_mesh(gl, mesh, scene);
        }

        for n in node.borrow().children.iter() {
            self.process_node(gl, n.clone(), scene);
        }
    }

    fn process_mesh(
        &mut self,
        gl: &GlFns,
        mesh: &russimp::mesh::Mesh,
        scene: &russimp::scene::Scene,
    ) {
        let mut m = Mesh::default();

        for i in 0..mesh.vertices.len() {
            let mut vert = Vertex::default();
            // position
            vert.position.x = mesh.vertices[i].x;
            vert.position.y = mesh.vertices[i].y;
            vert.position.z = mesh.vertices[i].z;
            // normals
            vert.normal.x = mesh.normals[i].x;
            vert.normal.y = mesh.normals[i].y;
            vert.normal.z = mesh.normals[i].z;
            // tex coords
            if !mesh.texture_coords.is_empty() {
                vert.tex_coords.x = mesh.texture_coords[0].as_ref().unwrap()[i].x;
                vert.tex_coords.y = mesh.texture_coords[0].as_ref().unwrap()[i].y;
            }

            m.vertices.push(vert);
        }

        // process indices
        for f in mesh.faces.iter() {
            m.indices.push(f.0[0]);
            m.indices.push(f.0[1]);
            m.indices.push(f.0[2]);
        }

        // process materials
        if mesh.material_index > 0 {
            let mat = &scene.materials[mesh.material_index as usize];
            // println!("Materials: {:?}", mat);
            for p in mat.properties.iter() {
                if p.key == "$tex.file" {
                    match p.semantic {
                        TextureType::Diffuse => {
                            let mut t = Texture {
                                id: 0,
                                tex_type: TexType::Diffuse,
                            };
                            if let russimp::material::PropertyTypeInfo::String(filename) = &p.data {
                                t.id = self
                                    .load_texture(gl, &format!("{}/{}", self.path, filename))
                                    .unwrap();
                                m.textures.push(t);
                            }
                        }
                        TextureType::Specular => {
                            let mut t = Texture {
                                id: 0,
                                tex_type: TexType::Specular,
                            };
                            if let russimp::material::PropertyTypeInfo::String(filename) = &p.data {
                                t.id = self
                                    .load_texture(gl, &format!("{}/{}", self.path, filename))
                                    .unwrap();
                                m.textures.push(t);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        self.meshes.push(m);
    }

    fn load_texture(&mut self, gl: &GlFns, filename: &str) -> Result<u32, String> {
        // lookup cache
        if self.texture_cache.contains_key(filename) {
            return Ok(*self.texture_cache.get(filename).unwrap());
        }
        // load texture
        let tex = glutils::load_texture(gl, filename)?;
        // update cache
        self.texture_cache.insert(filename.to_string(), tex);
        Ok(tex)
    }

    pub fn setup(&mut self, gl: &GlFns) -> Result<(), String> {
        for m in self.meshes.iter_mut() {
            m.setup_mesh(gl)?;
        }

        Ok(())
    }

    pub fn draw(&self, gl: &GlFns, shader: &Shaders) {
        for m in self.meshes.iter() {
            m.draw(gl, shader);
        }
    }
}
