use std::collections::HashMap;

pub type EventData<T> = HashMap<String, T>;
//pub type EventCallback<'callback, T> = |& mut EventData<T>|:'callback;
pub type EventCallback<'callback, T> = FnMut(& mut EventData<T>) -> () + 'callback;

pub trait Events<'callback, T> {
  fn on(&mut self, event_name: String, callback: &'callback mut EventCallback<'callback, T>);
  fn off(&mut self, event_name: String);
  fn emit(&mut self, event_name: String, EventData: & mut EventData<T>);
}

pub struct Emitter<'callback, T: 'callback> {
  events : HashMap<String, Vec<&'callback mut EventCallback<'callback, T>>>
}

impl<'callback, T> Emitter<'callback, T>{
  pub fn new() -> Emitter<'callback, T> {
    Emitter {
      events: HashMap::new()
    }
  }
}

impl<'callback, T> Events<'callback, T> for Emitter<'callback, T> {
  fn on(&mut self, event_name: String, callback: &'callback mut EventCallback<'callback, T>) {
    if self.events.contains_key(&event_name) {
      match self.events.get_mut(&event_name) {
          Some(callbacks) => callbacks.push(callback),
          None => (),
      }
    } else {
      let vec = vec![callback];
      self.events.insert(event_name, vec);
    }

  }
  fn off(&mut self, event_name: String) {
    self.events.remove(&event_name);
  }
  fn emit(&mut self, event_name: String, event_data: & mut EventData<T>) {
    match self.events.get_mut(&event_name) {
      Some(callbacks) => {
        for callback in callbacks.iter_mut() {
            (*callback)(event_data);
        }
      }
      _ => {;}
    }
  }
}
