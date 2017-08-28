extern crate uniforms;
use uniforms::uniform::Uniform;
use uniforms::uniform::UniformObserver;
use uniforms::uniform::GlSendBehavior;
use uniforms::gl_sender::GlSender;
extern crate gl;
use gl::types::*;

// EXAMPLE APPLICATION
// Consider the following scenario for three instances of Uniform: u1, u2, and u3

//      u3
//     /  \
//   u1 -- u2

// u1 depends on u2 and u3    no observers
// u2 depends on u3           observer is u1
// u3 depends on nothing      observers are u1 and u2
#[cfg(not(test))]
fn main(){

  use std::rc::Rc;
  println!("setting up uniforms u1, u2 and u3 according to: ");
  println!("u1 = (u2 + u3)/2");
  println!("u2 = u3*3");

  fn create_uniform(handle: i32, observers: Vec<Rc<UniformObserver>>) -> Rc<Uniform<f32>>{
    let sender = GlSender::<GLfloat>::new(handle);
    let boxed_sender = Box::new(sender) as Box<GlSendBehavior<GLfloat>>;
    let uniform = Uniform::<GLfloat>::new( 1.0, observers, Box::new(|| 1.0), Some(boxed_sender));
    Rc::new(uniform)
  }

  let ru1 = create_uniform(1, vec![]);                          // depends on ru2 and ru3
  let ru2 = create_uniform(1, vec![ru1.clone()]);               // depends on ru3
  let ru3 = create_uniform(1, vec![ru1.clone(), ru2.clone()]);  // depends on noone

  let u1_calc = {
    let clone_u2 = ru2.clone();
    let clone_u3 = ru3.clone();
    Box::new(move || (clone_u2.value.get() + clone_u3.value.get())/2.0)
  };
  *ru1.calculation.borrow_mut() = u1_calc;

  let u2_calc = {
    let clone_u3 = ru3.clone();
    Box::new(move || clone_u3.value.get()*3.0)
  };
  *ru2.calculation.borrow_mut() = u2_calc;

  let val = 7.0;
  println!("setting uniform 3 to {}", val);
  ru3.set(val);

  println!("Uniform 1 now has value to {}", ru1.value.get());
  println!("Uniform 2 now has value to {}", ru2.value.get());
  println!("Uniform 3 now has value to {}", ru3.value.get());

  let val2 = 80.0;
  println!("setting uniform 2 to {}", val2);
  ru2.set(val2);

  println!("Uniform 1 now has value to {}", ru1.value.get());
  println!("Uniform 2 now has value to {}", ru2.value.get());
  println!("Uniform 3 now has value to {}", ru3.value.get());

  println!("done");
}