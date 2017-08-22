mod uniforms;
use uniforms::*;
use std::rc::Rc;

// EXAMPLE APPLICATION
// Consider the following scenario for three instances of Uniform: u1, u2, and u3

//      u3
//     /  \
//   u1 -- u2

// u1 depends on u2 and u3    no observers
// u2 depends on nothing      observers are u1 and u3
// u3 depends on u2           observer is u1

#[cfg(not(test))]
fn main(){
  let ru1 = {
    let u1 = Uniform::new(1, 1.0);
    Rc::new(u1)
  };
  let ru3 = {
    let u3 = Uniform::with_observers(3, 1.0, vec![ru1.clone()]);
    Rc::new(u3)
  };
  let ru2 = {
    let u2 = Uniform::with_observers(2, 1.0, vec![ru1.clone(), ru3.clone()]);
    Rc::new(u2)
  };

  let u1_calc = {
    let clone_u2 = ru2.clone();
    let clone_u3 = ru3.clone();
    Box::new(move || (clone_u2.value.get() + clone_u3.value.get())/2.0)
  };
  *ru1.calculation.borrow_mut() = u1_calc;

  let u3_calc = {
    let clone_u2 = ru2.clone();
    Box::new(move || clone_u2.value.get()*3.0)
  };
  *ru3.calculation.borrow_mut() = u3_calc;

  ru2.set(7.0);
  ru2.send_to_opengl();
}