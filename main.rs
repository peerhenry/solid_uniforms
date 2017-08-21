use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Observer{
  fn notify(&self);
  fn send_to_opengl(&self);
}

struct Uniform<T>{
  handle: i32,
  value: Cell<T>,
  observers: Vec<Rc<RefCell<Observer>>>,
  calculation: Box<Fn() -> T>
}

impl Observer for Uniform<f32>{
  fn notify(&self){
    let new_value = (self.calculation)();
    self.value.set(new_value);
    println!("Setting uniform value to: {}", new_value);
  }

  fn send_to_opengl(&self){
    println!("gl::Uniform1f({0}, {1});", self.handle, self.value.get()); // replace by actual gl call
    for obs in &self.observers {  // Send all observers to opengl as well...
      let w_u: Rc<RefCell<Observer>> = obs.clone();
      let u = (*w_u).borrow();
      u.send_to_opengl();
    }
  }
}

impl<T> Uniform<T>{
  pub fn set(&self, new_value: T){
    self.value.set(new_value);    // 1. Set the new value
    for obs in &self.observers {  // 2. Notify observers
      let w_u: Rc<RefCell<Observer>> = obs.clone();
      let u = (*w_u).borrow();
      u.notify();
    }
  }

  pub fn set_observers(&mut self, new_observers: Vec<Rc<RefCell<Observer>>>){
    self.observers = new_observers;
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
}

fn main(){
  let u1 = Uniform::<f32>::new(0, 1.0);
  let wrapped_u1 = Rc::new(RefCell::new(u1)); // move it to an Rc
  let u2 = Uniform::<f32>::new(0, 1.0);
  let wrapped_u2 = Rc::new(RefCell::new(u2)); // move it to an Rc

  {
    let cloned_u2 = wrapped_u2.clone();
    // Need to borrow mutably to set calculation in uniform 1
    let mut borrowed_u1 = (*wrapped_u1).borrow_mut();
    borrowed_u1.calculation = Box::new(move || (*cloned_u2).borrow().value.get()/2.0 ); // the closure becomes owner of the cloned uniform 2
  }

  {
    // Need to borrow mutably to set observers in uniform 2
    let mut borrowed_u2 = (*wrapped_u2).borrow_mut();
    borrowed_u2.set_observers(vec![wrapped_u1.clone()]);
  }

  // borrow to call set
  let borrowed_u2 = (*wrapped_u2).borrow();
  borrowed_u2.set(7.0);
  borrowed_u2.send_to_opengl();

  let borrowed_u1 = (*wrapped_u1).borrow();
  borrowed_u1.send_to_opengl();
}