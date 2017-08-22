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
  pub observers: Vec<Rcim<Observer>>,
  pub calculation: Box<Fn() -> T>
}

// -- Define type aliases --

// Rcim: reference counting interior mutability
type Rcim<T> = Rc<RefCell<T>>;
type RcimUniform<T> = Rc<RefCell<Uniform<T>>>;

// -- Implementations for generic structs --

impl<T> Uniform<T>{
  pub fn set(&self, new_value: T){
    self.value.set(new_value);    // 1. Set the new value
    for obs in &self.observers {  // 2. Notify observers
      //let w_u: Rc<RefCell<Observer>> = obs.clone();
      let u = (*obs).borrow();
      u.notify();
    }
  }

  pub fn set_observers(&mut self, new_observers: Vec<Rc<RefCell<Observer>>>){
    self.observers = new_observers;
  }
}

impl<T> Flushable for Uniform<T>{
  fn flush_observers(&self){
    for obs in &self.observers {  // Send all observers to opengl...
      //let w_u: Rc<RefCell<Observer>> = obs.clone();
      let u = (*obs).borrow();
      u.send_to_opengl();
    }
  }
}

// -- Type specific implementations --

impl Observer for Uniform<f32>{
  fn notify(&self){
    let new_value = (self.calculation)();
    println!("[NOTIFY] Setting uniform {0} value to: {1}", self.handle, new_value);
    self.set(new_value);
  }

  fn send_to_opengl(&self){
    println!("gl::Uniform1f({0}, {1});", self.handle, self.value.get()); // replace by actual gl call
    self.flush_observers();
  }
}

impl Uniform<f32>{
  pub fn new(hanle: i32, value: f32) -> Uniform<f32>{
    Uniform{
      handle: hanle,
      value: Cell::new(value),
      observers: vec![],
      calculation: Box::new(|| 0.0)
    }
  }

  pub fn new_wrapped(handle: i32) -> RcimUniform<f32>{
    Rc::new(RefCell::new(Uniform::<f32>::new(handle, 1.0)))
  }

  pub fn new_wrapped_default() -> RcimUniform<f32>{
    Rc::new(RefCell::new(Uniform::<f32>::new(0, 1.0)))
  }
}