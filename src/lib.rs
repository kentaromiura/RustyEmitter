use std::collections::HashMap;

pub type EventData<T> = HashMap<String, T>;
pub type EventCallback<'callback, T> = |& mut EventData<T>|:'callback;


pub trait Events<'callback, T> {
  fn on(&mut self, event_name: String, callback: EventCallback<'callback, T>);
  fn off(&mut self, event_name: String);
  fn emit(&mut self, event_name: String, EventData: & mut EventData<T>);
}

pub struct Emitter<'callback, T> {
  events : HashMap<String, Vec<EventCallback<'callback, T>>>
}

impl<'callback, T> Emitter<'callback, T>{
  pub fn new() -> Emitter<'callback, T> {
    Emitter {
      events: HashMap::new()
    }
  }
}

impl<'callback, T> Events<'callback, T> for Emitter<'callback, T> {
  fn on(&mut self, event_name: String, callback: EventCallback<'callback, T>) {
    if self.events.contains_key(&event_name) {
      self.events.get_mut(&event_name).push(callback);
    } else {
      let vec = vec![callback];
      self.events.insert(event_name, vec);
    }

  }
  fn off(&mut self, event_name: String) {
    self.events.remove(&event_name);
  }
  fn emit(&mut self, event_name: String, event_data: & mut EventData<T>) {
    match self.events.find_mut(&event_name) {
      Some(callbacks) => {
        for callback in callbacks.iter_mut() {
            (*callback)(event_data);
        }
      }
      _ => {;}
    }
  }
}
