use std::collections::HashMap;

pub type EventData = HashMap<String, String>;
pub type EventCallback<'callback> = |EventData|:'callback;


pub trait Events<'callback> {
  fn on(&mut self, eventName: String, callback: EventCallback<'callback>);
  fn off(&mut self, eventName: String);
  fn emit(&mut self, eventName: String, eventData: EventData);
}

pub struct Emitter<'callback> {
  events : HashMap<String, Vec<EventCallback<'callback>>>
}

impl<'callback> Emitter<'callback>{
  pub fn new() -> Emitter<'callback> {
    return Emitter {
      events : HashMap::new()
    }
  }
}

impl<'callback> Events<'callback> for Emitter<'callback> {
  fn on(&mut self, eventName: String, callback: EventCallback<'callback>) {
    //let vec : Vec<EventCallback<'callback>>;
    if self.events.contains_key(&eventName) {
      self.events.get_mut(&eventName).push(callback);
    } else {
      let vec = vec![callback];
      self.events.insert(eventName, vec);
    }

  }
  fn off(&mut self, eventName: String) {
    self.events.remove(&eventName);
  }
  fn emit(&mut self, eventName: String, eventData: EventData) {
    match self.events.find_mut(&eventName) {
      Some(callbacks) => {
        for callback in callbacks.mut_iter() {
            (*callback)(eventData.clone()); // it would be cool to have a way to talk back though.
        }
      }
      _ => {;}
    }
  }
}
