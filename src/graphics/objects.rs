pub struct VBOData {
    id: u32,
    vertex_data: Vec<f32>,
    vertex_size: i32,
    vertex_count: i32,
}

impl VBOData {
    pub fn new(vertex_data: Vec<f32>, vertex_size: i32) -> VBOData {
        let mut vbo = VBOData {
            id: 0,
            vertex_data: vertex_data.clone(),
            vertex_size,
            vertex_count: (vertex_data.len() as i32) / vertex_size,
        };

        // Generate a new VBO ID
        unsafe {
            gl::GenBuffers(1, &mut vbo.id);
        }

        // Bind the VBO and upload the vertex data
        vbo.bind();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vbo.vertex_data.len() * std::mem::size_of::<f32>()) as isize,
                vbo.vertex_data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }

        vbo
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }
}
pub struct IBOData {
    id: u32,
    index_data: Vec<u32>,
    index_count: i32,
}

impl IBOData {
    pub fn new(index_data: Vec<u32>) -> IBOData {
        let mut ibo = IBOData {
            id: 0,
            index_data: index_data.clone(),
            index_count: index_data.len() as i32,
        };

        // Generate a new IBO ID
        unsafe {
            gl::GenBuffers(1, &mut ibo.id);
        }

        // Bind the IBO and upload the index data
        ibo.bind();
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (ibo.index_data.len() * std::mem::size_of::<u32>()) as isize,
                ibo.index_data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }

        ibo
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }
}

pub struct VAOData {
    id: u32,
}

impl VAOData {
    pub fn new() -> VAOData {
        let mut vao = VAOData {
            id: 0,
        };

        // Generate a new VAO ID
        unsafe {
            gl::GenVertexArrays(1, &mut vao.id);
        }

        // Bind the VAO
        vao.bind();

        vao
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
}

