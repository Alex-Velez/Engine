use super::{ Math::Matrix4x4,
	gl::{self, types::*}
};

use std::ffi::CString;
use std::ptr;
use std::str;

// Program Tracker (unsafe)
static mut PROGRAMS: Vec<u32> = Vec::new();

pub fn Load(shader: Shader) -> GLuint {
	// build and compile our shader program
	// ------------------------------------
	unsafe {
		// vertex shader
		let vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
		let c_str_vert = CString::new(shader.vertexCode.as_bytes()).unwrap();
		gl::ShaderSource(vertexShader, 1, &c_str_vert.as_ptr(), ptr::null());
		gl::CompileShader(vertexShader);

		// check for shader compile errors
		let mut success = gl::FALSE as GLint;
		let mut infoLog = Vec::with_capacity(512);
		infoLog.set_len(512 - 1); // subtract 1 to skip the trailing null character
		gl::GetShaderiv(vertexShader, gl::COMPILE_STATUS, &mut success);
		if success != gl::TRUE as GLint {
			gl::GetShaderInfoLog(vertexShader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
			println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&infoLog).unwrap());
		}

		// fragment shader
		let fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
		let c_str_frag = CString::new(shader.fragmentCode.as_bytes()).unwrap();
		gl::ShaderSource(fragmentShader, 1, &c_str_frag.as_ptr(), ptr::null());
		gl::CompileShader(fragmentShader);

		// check for shader compile errors
		gl::GetShaderiv(fragmentShader, gl::COMPILE_STATUS, &mut success);
		if success != gl::TRUE as GLint {
			gl::GetShaderInfoLog(fragmentShader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
			println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&infoLog).unwrap());
		}

		// link shaders
		let shaderProgram = gl::CreateProgram();
		gl::AttachShader(shaderProgram, vertexShader);
		gl::AttachShader(shaderProgram, fragmentShader);
		gl::LinkProgram(shaderProgram);

		// check for linking errors
        gl::GetProgramiv(shaderProgram, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(shaderProgram, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&infoLog).unwrap());
		}

		// delete shaders
		gl::DetachShader(shaderProgram, vertexShader);
		gl::DetachShader(shaderProgram, fragmentShader);
		gl::DeleteShader(vertexShader);
		gl::DeleteShader(fragmentShader);
		
		shaderProgram
	}
}

pub fn Use(programs: Vec<GLuint>) {
	unsafe {
		for program in programs {
			gl::UseProgram(program);
		}
	}
}

pub fn SetMatrix4x4(program_id: GLuint, uniformName: &str, mat: Matrix4x4) {
	unsafe {
		let location = gl::GetUniformLocation(program_id, uniformName.as_bytes()[0] as *const i8 as *const GLchar);
		gl::UniformMatrix4fv(location, 1 as GLsizei, gl::FALSE, mat.array().as_ptr() as *const f32 as *const GLfloat);
	}
}

pub struct Shader {
	pub vertexCode: String,
	pub fragmentCode: String,
	pub programID: u32,
}

impl Shader {
	pub fn new() -> Shader {
		let vertexCode: String = String::from(r#"
			#version 330 core
			layout (location = 0) in vec2 aPosition;
			layout (location = 1) in vec3 aColor;
			out vec4 vertexColor;
			void main() {
				vertexColor = vec4(aColor.rgb, 1.0);
				gl_Position = vec4(aPosition.xy, 0, 1.0);
			}
		"#);

		let fragmentCode: String = String::from(r#"
			#version 330 core
			out vec4 FragColor;
			in vec4 vertexColor;
			void main() {
				FragColor = vertexColor;
			}
		"#);


		let programID = unsafe {
			if PROGRAMS.is_empty() {
				PROGRAMS.push(0);
				0
			} else {
				let mut maybe: u32 = PROGRAMS.len() as u32;
				let unique = loop {
					if PROGRAMS.iter().any(|&i| i==maybe) {
						maybe += 1;
						continue;
					}
					break maybe;
				};
				PROGRAMS.push(unique);
				unique
			}
		};

		Shader { vertexCode, fragmentCode, programID }
	}

	pub fn from(vertexFilePath: &str, fragmentFilePath: &str) -> Shader {
		let vertexCode: String = std::fs::read_to_string(vertexFilePath)
			.expect("Could not open shader file!");

		let fragmentCode: String = std::fs::read_to_string(fragmentFilePath)
			.expect("Could not open shader file!");

		let programID = unsafe {
			if PROGRAMS.is_empty() {
				PROGRAMS.push(0);
				0
			} else {
				let mut maybe: u32 = PROGRAMS.len() as u32;
				let unique = loop {
					if PROGRAMS.iter().any(|&i| i==maybe) {
						maybe += 1;
						continue;
					}
					break maybe;
				};
				PROGRAMS.push(unique);
				unique
			}
		};

		Shader { vertexCode, fragmentCode, programID }
	}
}