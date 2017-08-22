use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

// -- Define traits --

pub trait Flushable{
  fn flush_observers(&self);
}

pub trait Observer : Flushable{
  fn notify(&self);
  fn send_to_opengl(&self);
}

// -- Define structs --

pub struct Uniform<T>{
  pub handle: i32,
  pub value: Cell<T>,
  pub observers: Vec<Rc<Observer>>,
  pub calculation: RefCell<Box<Fn() -> T>>
}

// -- Implementations for generic structs --

impl<T> Uniform<T>{
  pub fn set(&self, new_value: T){
    self.value.set(new_value);    // 1. Set the new value
    for obs in &self.observers {  // 2. Notify observers
      obs.notify();
    }
  }
}

impl<T> Flushable for Uniform<T>{
  fn flush_observers(&self){
    for obs in &self.observers {  // Send all observers to opengl...
      obs.send_to_opengl();
    }
  }
}

// -- Type specific implementations --

impl Observer for Uniform<f32>{
  fn notify(&self){
    let new_value = (self.calculation.borrow())();
    println!("[NOTIFY] Setting uniform {0} value to: {1}", self.handle, new_value);
    self.set(new_value);
  }

  fn send_to_opengl(&self){
    println!("gl::Uniform1f({0}, {1});", self.handle, self.value.get()); // replace by actual gl call
    self.flush_observers();
  }
}

impl Uniform<f32>{
  pub fn new(handle: i32, value: f32) -> Uniform<f32>{
    Uniform{
      handle: handle,
      value: Cell::new(value),
      observers: vec![],
      calculation: RefCell::new(Box::new(|| 0.0))
    }
  }

  pub fn with_observers(handle: i32, value: f32, observers: Vec<Rc<Observer>>) -> Uniform<f32>{
    Uniform{
      handle: handle,
      value: Cell::new(value),
      observers: observers,
      calculation: RefCell::new(Box::new(|| 0.0))
    }
  }
}

// -- Unit tests --

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn uniform_notifies_observers() {
    let wrapped_u1 = {
      let u1 = Uniform::new(1, 1.0);
      Rc::new(u1)
    };
    let wrapped_u2 = {
      let u2 = Uniform::with_observers(1, 1.0, vec![wrapped_u1.clone()] );
      Rc::new(u2)
    };
    
    wrapped_u2.set(7.0);

    assert_eq!(wrapped_u2.value.get(), 7.0);
  }
}