use std::cell::RefCell;
use std::rc::Rc;
use traits::*;

// The observers of the uniforms need to be inside an Rc (Reference counter), which allows multiple ownership.
pub type ObserverCollection = Vec<Rc<UniformObserver>>;

// GlSenderType is an Option because None is for partial uniforms (partial uniforms don't correspond 1-to-1 to uniforms on the shader)
// The SendBehavior is in a Box because its size is not known at compile time.
pub type GlSenderType<T> = Option<Box<GlSendBehavior<T>>>;

// In a RefCell to allow internal mutability. Functions just need to be inside a Box.
pub type CalculationType<T> = RefCell<Box<Fn() -> T>>;