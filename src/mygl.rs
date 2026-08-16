use std::collections::HashMap;

use gl;
use gl::types::GLenum;

pub struct ArrayObject(u32);
impl ArrayObject {
    pub fn new() -> Self {
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao); }
        assert_ne!(vao, 0);

        Self(vao)
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.0);
        }
    }

    pub fn get_handle(&self) -> u32 {
        self.0
    }
}

pub struct BufferObject(u32);
impl BufferObject {
    pub fn new() -> Self {
        let mut vbo = 0;
        unsafe { gl::GenBuffers(1, &mut vbo); }
        assert_ne!(vbo, 0);

        Self(vbo)
    }

    pub fn bind(&self, target: GLenum) {
        unsafe {
            gl::BindBuffer(target, self.0);
        }
    }

    pub fn get_handle(&self) -> u32 {
        self.0
    }
}

pub struct Shader(u32, GLenum);
impl Shader {
    pub fn new(shader_type: GLenum) -> Self {
        unsafe {
            let vertex_shader = gl::CreateShader(shader_type);
            assert_ne!(vertex_shader, 0);
            Self(vertex_shader, shader_type)
        }
    }

    pub fn source(&self, code: &str) {
        unsafe {
            gl::ShaderSource(
                self.0,
                1,
                &(code.as_bytes().as_ptr().cast()),
                &(code.len().try_into().unwrap()),
            );
        }
    }

    pub fn compile(&self) {
        unsafe { gl::CompileShader(self.0) };
    }

    pub fn compile_success(&self) -> bool {
        let mut compiled = 0;
        unsafe { gl::GetShaderiv(self.0, gl::COMPILE_STATUS, &mut compiled) };
        compiled == i32::from(gl::TRUE)
    }

    pub fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { gl::GetShaderiv(self.0, gl::INFO_LOG_LENGTH, &mut needed_len) };

        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            gl::GetShaderInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }

        String::from_utf8_lossy(&v).into_owned()
    }

    pub fn delete(self) {
        unsafe { gl::DeleteShader(self.0) };
    }

    pub fn from_source(code: &str, shader_type: GLenum) -> Self {
        let shader = Shader::new(shader_type);
        shader.source(code);
        shader.compile();

        if !shader.compile_success() {
            panic!("{}", shader.info_log());
        }

        shader
    }

    pub fn get_handle(&self) -> u32 {
        self.0
    }

    pub fn get_type(&self) -> GLenum {
        self.1
    }
}

pub struct Program(u32);
impl Program {
    pub fn new() -> Self {
        unsafe {
            let shader_program = gl::CreateProgram();
            assert_ne!(shader_program, 0);
            Self(shader_program)
        }
    }

    pub fn attach_shader(&self, shader: &Shader) {
        unsafe {
            gl::AttachShader(self.0, shader.0);
        }
    }

    pub fn link(&self) {
        unsafe {
            gl::LinkProgram(self.0);
        }
    }

    pub fn link_success(&self) -> bool {
        let mut success = 0;
        unsafe { gl::GetProgramiv(self.0, gl::LINK_STATUS, &mut success) };
        success == i32::from(gl::TRUE)
    }

    pub fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { gl::GetProgramiv(self.0, gl::INFO_LOG_LENGTH, &mut needed_len) };

        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            gl::GetProgramInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }

        String::from_utf8_lossy(&v).into_owned()
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.0);
        }
    }

    pub fn from_vert_frag(vert_code: &str, frag_code: &str) -> Self {
        let program = Program::new();

        // create the shaders
        let vert = Shader::from_source(vert_code, gl::VERTEX_SHADER);
        let frag = Shader::from_source(frag_code, gl::FRAGMENT_SHADER);

        program.attach_shader(&vert);
        program.attach_shader(&frag);
        program.link();

        if !program.link_success() {
            panic!("{}", program.info_log());
        }

        vert.delete();
        frag.delete();

        program
    }

    pub fn get_handle(&self) -> u32 {
        self.0
    }
}

pub struct Object {
    buffer_objects: HashMap<GLenum, BufferObject>,
    array_object: ArrayObject,
}

impl Object {
    pub fn new() -> Self {
        Self {
            buffer_objects: HashMap::new(),
            array_object: ArrayObject::new(),
        }
    }

    pub fn bind(&self) {
        self.array_object.bind();
    }

    pub fn buffer_data(&mut self, buffer_type: GLenum, data: &[u8], usage: GLenum) -> &BufferObject {
        let buffer = self.buffer_objects.entry(buffer_type).or_insert(BufferObject::new());
        buffer.bind(buffer_type);
        
        unsafe {
            gl::BufferData(
                buffer_type,
                size_of_val(data) as isize,
                data.as_ptr().cast(),
                usage,
            );
        }

        buffer
    }

    pub fn get_buffer_object(&self, buffer_type: GLenum) -> Option<&BufferObject> {
        self.buffer_objects.get(&buffer_type)
    }

    pub fn bind_buffer_object(&self, buffer_type: GLenum) {
        self.buffer_objects.get(&buffer_type).and_then(|buffer| {buffer.bind(buffer_type); Option::Some(())});
    }

    pub fn vertex_attribute(index: u32, size: i32, data_type: GLenum, normalized: u8, stride: i32, pointer: usize, enable: bool) {
        unsafe {
            gl::VertexAttribPointer(
                index,
                size,
                data_type,
                normalized,
                stride,
                pointer as *const _,
            );

            if enable  {
                gl::EnableVertexAttribArray(index);
            }
        }
    }

    pub fn vertex_attribute_simple(size: i32, data_type: GLenum) {
        let data_size = match data_type {
            gl::FLOAT => 4,
            gl::INT => 4,
            gl::UNSIGNED_INT => 4,
            gl::SHORT => 2,
            gl::UNSIGNED_SHORT => 2,
            gl::BYTE => 1,
            gl::UNSIGNED_BYTE => 1,
            gl::DOUBLE => 8,
            _ => panic!("Unhandled GL type in vertex_attribute_simple: {}", data_type),
        };

        Self::vertex_attribute(0, size, data_type, gl::FALSE, data_size * size, 0, true);
    }
    
    pub fn enable_vertex_attribute(index: u32) {
        unsafe {
            gl::EnableVertexAttribArray(index);
        }
    }
}