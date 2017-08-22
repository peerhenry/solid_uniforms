mod uniforms;
use uniforms::*;

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
  // Create uniforms wrapped in Rc<RefCell<>>
  let wrapped_u1 = Uniform::new_wrapped(0);
  let wrapped_u2 = Uniform::new_wrapped(1);
  let wrapped_u3 = Uniform::new_wrapped(2);

  // set calculation for u1
  {
    // u1 will be (u2+u3)/2
    let cloned_u2 = wrapped_u2.clone(); // clone uniforms that u1 will depend on.
    let cloned_u3 = wrapped_u3.clone();
    // These two closure simply get the uniform value
    let get_u2_val = move || (*cloned_u2).borrow().value.get(); // give ownership to closures
    let get_u3_val = move || (*cloned_u3).borrow().value.get();
    let calc_u1 = move || ( get_u2_val() + get_u3_val() )/2.0; // move getters to calculation
    let mut borrowed_u1 = (*wrapped_u1).borrow_mut();
    borrowed_u1.calculation = Box::new( calc_u1 ); // set in mutably borrowed u1
  }

  // set calculation for u3
  {
    // u3 will be u2*3
    let cloned_u2 = wrapped_u2.clone();
    let get_u2_val = move || (*cloned_u2).borrow().value.get();
    let calc_u3 = move || get_u2_val()*3.0;
    let mut borrowed_u3 = (*wrapped_u3).borrow_mut();
    borrowed_u3.calculation = Box::new( calc_u3 );
  }

  // Set observers for u2
  {
    let mut borrowed_u2 = (*wrapped_u2).borrow_mut(); // setting observers requires mutable borrow
    borrowed_u2.set_observers(vec![wrapped_u1.clone(), wrapped_u3.clone()]);
  }

  // Set observers for u3
  {
    let mut borrowed_u3 = (*wrapped_u3).borrow_mut();
    borrowed_u3.set_observers(vec![wrapped_u1.clone()]);
  }

  // borrow to call set
  let borrowed_u2 = (*wrapped_u2).borrow();
  borrowed_u2.set(7.0);
  borrowed_u2.send_to_opengl();

  //let borrowed_u1 = (*wrapped_u1).borrow();
  //borrowed_u1.send_to_opengl();
}