use std::cell::Cell;
use std::cell::RefCell;
use std::marker::Copy;

use traits::*;
use types::*;

// -- Define structs --

pub struct Uniform<T> {
  pub value: Cell<T>,                     // Stores the value for the uniform
  pub observers: ObserverCollection,      // Contains uniforms that observe this one
  pub calculation: CalculationType<T>,    // Calculates the value of the uniform
  pub gl_sender: GlSenderType<T>          // Sends the value to OpenGL, Boxed because of trait bound Sized is required by Option
}

// -- Implementation for struct --

impl<T> Uniform<T> where T: Copy{
  pub fn new(value: T, observers: ObserverCollection, calculation: Box<Fn() -> T>, gl_sender: GlSenderType<T>) -> Uniform<T>{
    Uniform{
      value: Cell::new(value),
      observers: observers,
      calculation: RefCell::new(calculation),
      gl_sender: gl_sender
    }
  }

  pub fn set(&self, new_value: T){
    self.value.set(new_value);
    self.notify_observers();
  }

  #[allow(dead_code)]
  pub fn set_and_send(&self, new_value: T){
    self.set(new_value);
    self.send_to_opengl();
  }

  pub fn set_calculation(&self, calculation: Box<Fn() -> T>){
    *self.calculation.borrow_mut() = calculation
  }
}

// -- Implementation of traits --

impl<T> UniformObserver for Uniform<T> where T: Copy{
  fn notify(&self){
    let new_value = (*self.calculation.borrow())();
    self.set(new_value);
  }

  fn send_to_opengl(&self){
    let val = self.value.get();
    match self.gl_sender.as_ref() { // without .as_ref() causes error: cannot move self out of borrowed content
      Some(sender) => sender.send_to_opengl(val),
      None => ()
    }
    self.send_observers_to_opengl();
  }
}

impl<T> UniformObservable for Uniform<T>{
  fn get_observers(&self) -> &ObserverCollection{
    &self.observers
  }
}


// -----------
// -- TESTS --
// -----------


#[cfg(test)]
mod tests{

  use super::*;
  use std::rc::Rc;

  // Note: we need to wrap uniforms in Rc in order to get multiple ownership.
  // This is because they need to be accessable through:
  // 1. Calculations of other uniforms that depend on it.
  // 2. Collection of observers in uniforms on which it depends.
  // 3. Anywhere in the application where access to a uniform is needed.

  impl Uniform<f32>{
    pub fn dummy_new() -> Rc<Uniform<f32>>{
      Uniform::with_observers(vec![])
    }

    pub fn with_observers(observers: ObserverCollection) -> Rc<Uniform<f32>>{
      let uniform = Uniform::<f32>{
        value: Cell::new(1.0),
        observers: observers,
        calculation: RefCell::new(Box::new(|| 1.0)),
        gl_sender: None
      };
      Rc::new(uniform)
    }
  }

  #[test]
  fn set_notifies_observers() {
    // Arrange
    let wrapped_u1 = Uniform::dummy_new();
    let wrapped_u2 = Uniform::with_observers(vec![wrapped_u1.clone()]);

    // setup calculation for uniform 1
    let u1_calc = {
      let clone_u2 = wrapped_u2.clone();
      Box::new(move || { clone_u2.value.get()/2.0 })
    };
    wrapped_u1.set_calculation(u1_calc);
    
    // Act
    wrapped_u2.set(7.0);

    // Assert
    assert_eq!(wrapped_u2.value.get(), 7.0);
    assert_eq!(wrapped_u1.value.get(), 3.5);
  }

  #[test]
  fn set_and_send_notifies_observers() {
    // Arrange
    let wrapped_u1 = Uniform::dummy_new();
    let wrapped_u2 = Uniform::with_observers(vec![wrapped_u1.clone()]);

    // setup calculation for uniform 1
    let u1_calc = {
      let clone_u2 = wrapped_u2.clone();
      Box::new(move || { clone_u2.value.get()/2.0 })
    };
    wrapped_u1.set_calculation(u1_calc);
    
    // Act
    wrapped_u2.set_and_send(7.0);

    // Assert
    assert_eq!(wrapped_u2.value.get(), 7.0);
    assert_eq!(wrapped_u1.value.get(), 3.5);
  }

  #[test]
  fn partial_uniform_sets_observers(){
    // Arrange
    let wrapped_u1 = Uniform::dummy_new();
    let wrapped_u2 = Uniform::dummy_new();
    let pu = Uniform::<f32>{
      value: Cell::new(1.0),
      observers: vec![wrapped_u1.clone(), wrapped_u2.clone()],
      calculation: RefCell::new(Box::new(||1.0)),
      gl_sender: None
    };

    let partial = Rc::new(pu);
    let cp1 = partial.clone();
    wrapped_u1.set_calculation(Box::new(move || cp1.value.get()/5.0));
    let cp2 = partial.clone();
    wrapped_u2.set_calculation(Box::new(move || cp2.value.get()*3.0));

    // Act
    partial.set(10.0);

    // Assert
    assert_eq!(wrapped_u1.value.get(), 2.0);
    assert_eq!(wrapped_u2.value.get(), 30.0);
  }

  #[test]
  fn uniform_calls_sender(){
    // Arrange
    static mut DUMMY_WAS_SENT: bool = false;
    struct DummySender{}
    impl GlSendBehavior<f32> for DummySender{
      #[allow(unused_variables)]
      fn send_to_opengl(&self, data: f32){
        unsafe{
          DUMMY_WAS_SENT = true;
        }
      }
    }
    let sender = DummySender{};
    let boxed_sender = Box::new(sender) as Box<GlSendBehavior<f32>>;
    let wrapped_sender = Some(boxed_sender);
    let uniform = Uniform::<f32>{
      value: Cell::new(1.0),
      observers: vec![],
      calculation: RefCell::new(Box::new(||1.0)),
      gl_sender: wrapped_sender
    };

    // Act
    uniform.send_to_opengl();

    // Assert
    unsafe{
      assert!(DUMMY_WAS_SENT);
    }
  }
}