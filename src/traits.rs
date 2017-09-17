use std::rc::Rc;

#[allow(unused_variables)]
pub trait GlSendBehavior<T>{
  fn send_to_opengl(&self, data: T){
    // default behavior: do nothing
  }
}

pub trait UniformObserver {
  fn notify(&self);
  fn send_to_opengl(&self);
}

pub trait UniformObservable {
  fn get_observers(&self) -> &Vec<Rc<UniformObserver>>;
  fn notify_observers(&self) {
    for obs in self.get_observers() {
      obs.notify();
    }
  }
  fn send_observers_to_opengl(&self) {
    for obs in self.get_observers() {
      obs.send_to_opengl();
    }
  }
}