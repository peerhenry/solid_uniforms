use std::marker::PhantomData;
use gl;
use gl::types::{GLint, GLfloat};
use uniform::GlSendBehavior;
use cgmath::{Matrix, Matrix3, Matrix4};

// -- OpenGL Send Behavior --

pub struct GlSender<T>{
  pub handle: GLint,
  phantom: PhantomData<T> // marker to allow the struct to be generic
}

#[allow(non_camel_case_types)]
impl<T> GlSender<T>{
  pub fn new(handle: GLint) -> GlSender<T>{
    GlSender::<T>{
      handle: handle,
      phantom: PhantomData
    }
  }
}

#[allow(unused_variables, non_camel_case_types)]
impl GlSendBehavior<GLfloat> for GlSender<GLfloat>{
  fn send_to_opengl(&self, value: GLfloat){
    unsafe{ 
      gl::Uniform1f(self.handle, value);
    }
  }
}

#[allow(unused_variables, non_camel_case_types)]
impl GlSendBehavior<Matrix3<GLfloat>> for GlSender<Matrix3<GLfloat>>{
  fn send_to_opengl(&self, value: Matrix3<GLfloat>){
    unsafe{
      gl::UniformMatrix4fv(self.handle, 1, gl::FALSE, value.as_ptr());
    }
  }
}

#[allow(unused_variables, non_camel_case_types)]
impl GlSendBehavior<Matrix4<GLfloat>> for GlSender<Matrix4<GLfloat>>{
  fn send_to_opengl(&self, value: Matrix4<GLfloat>){
    unsafe{
      gl::UniformMatrix4fv(self.handle, 1, gl::FALSE, value.as_ptr());
    }
  }
}