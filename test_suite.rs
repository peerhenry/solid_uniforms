mod uniforms;
use uniforms::*;

#[test]
fn setting_uniform_notifies_observer(){
  let wrapped_u1 = Uniform::new_wrapped(0);
  let wrapped_u2 = Uniform::new_wrapped(1);

  { // set calc
    let cloned_u2 = wrapped_u2.clone();
    let mut borrowed_u1 = (*wrapped_u1).borrow_mut();
    borrowed_u1.calculation = Box::new( move || (*cloned_u2).borrow().value.get()/2.0 );
  }

  { // set observers
    let mut borrowed_u2 = (*wrapped_u2).borrow_mut();
    borrowed_u2.observers = vec![wrapped_u1.clone()];
  }

  {
    let borrowed_u2 = (*wrapped_u2).borrow();
    borrowed_u2.set(7.0);
    assert_eq!(borrowed_u2.value.get(), 7.0);
    let borrowed_u1 = (*wrapped_u1).borrow();
    assert_eq!(borrowed_u1.value.get(), 3.5);
  }
}