use super::{ Math::Matrix4x4,
	gl::{self, types::*}
};

use std::ffi::CString;
use std::ptr;
use std::str;

pub fn Load(vertexFilePath: &str, fragmentFilePath: &str) -> GLuint {

	let vertexCode: String = std::fs::read_to_string(vertexFilePath)
		.expect("Could not open shader file!");
	let fragmentCode: String = std::fs::read_to_string(fragmentFilePath)
		.expect("Could not open shader file!");

	// build and compile our shader program
	// ------------------------------------
	unsafe {
		// vertex shader
		let vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
		let c_str_vert = CString::new(vertexCode.as_bytes()).unwrap();
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
		let c_str_frag = CString::new(fragmentCode.as_bytes()).unwrap();
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
	println!("Shader, use");
	unsafe {
		for program in programs {
			gl::UseProgram(program);
		}
	}
}

pub fn SetMatrix4x4(program_id: GLuint, uniformName: &str, mat: Matrix4x4) {
	println!("Shader, set matrix 4x4");
	unsafe {
		println!("Shader, let location");
		let c_uniformName = CString::new(uniformName).unwrap();
		let location = gl::GetUniformLocation(program_id, c_uniformName.as_ptr());
		println!("Shader, gl::UniformMatrix4fv");
		gl::UniformMatrix4fv(location, 1, gl::FALSE, mat.array().as_ptr() as *const f32 as *const GLfloat);
		println!("hmm!!!");
	}
}