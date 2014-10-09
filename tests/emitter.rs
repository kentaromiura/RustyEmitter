use event::Events;
use std::collections::HashMap;

#[path="../src/lib.rs"]
mod event;

#[test]
fn must_register_an_event_and_call_callback_passing_data() {
    let mut emitter = event::Emitter::new();
    let test = String::from_str("TEST");
    let data_inside = String::from_str("DATA INSIDE");

    emitter.on(test.clone(), |event:& mut HashMap<String, String>|{

        match event.find(&test) {
          Some(thing) => {
            if *thing != data_inside {
              fail!("the data inside the event is not what is expected, expected {}, get {}", data_inside, *thing);
            }
          },
          None => {
            fail!("the data expected wasn't found")
          },
        }
    });

    let mut data : HashMap<String, String> = HashMap::new();
    data.insert(test.clone(), data_inside.clone());
    emitter.emit(test.clone(), & mut data);
}

#[test]
fn must_register_multiple_events_and_call_all_the_callback_passing_a_copy_of_the_same_data() {
    let mut emitter = event::Emitter::new();
    let test = String::from_str("TEST");
    let data_inside = String::from_str("DATA INSIDE");

    emitter.on(test.clone(), |event:& mut HashMap<String, String>|{

        match event.find(&test) {
          Some(thing) => {
            if *thing != data_inside {
              fail!("the data inside the event is not what is expected, expected {}, get {}", data_inside, *thing);
            }
          },
          None => {
            fail!("the data expected wasn't found")
          },
        }
    });

    emitter.on(test.clone(), |event:& mut HashMap<String, String>|{
        match event.find(&test) {
          Some(thing) => {
            if *thing != data_inside {
              fail!("the data inside the event is not what is expected, expected {}, get {}", data_inside, *thing);
            }
          },
          None => {
            fail!("the data expected wasn't found")
          },
        }
    });

    let mut data : HashMap<String, String> = HashMap::new();
    data.insert(test.clone(), data_inside.clone());
    emitter.emit(test.clone(), & mut data);
}

#[test]
fn must_unregister_an_event() {
    let mut emitter = event::Emitter::new();
    let test = String::from_str("TEST");
    let data_inside = String::from_str("DATA INSIDE");

    emitter.on(test.clone(), |event: & mut HashMap<String, String>|{
        fail!("the emitter must never be called instead has been called with this parameters {}", event);
    });

    let mut data : HashMap<String, String> = HashMap::new();
    data.insert(test.clone(), data_inside.clone());
    emitter.off(test.clone());
    emitter.emit(test.clone(), & mut data);

}

#[test]
fn must_work_with_a_custom_event_data_and_allow_the_callback_to_modified_it(){
    struct Foo {
      bar : String
    };

    let mut emitter = event::Emitter::new();
    let test = String::from_str("TEST");
    let data_inside = String::from_str("DATA INSIDE");

    emitter.on(test.clone(), |event:& mut HashMap<String, Foo>|{
        match event.find(&test) {
          Some(thing) => {
            if (*thing).bar != data_inside {
              fail!("the data inside the event is not what is expected, expected {}, get {}", data_inside, (*thing).bar);
            }
          },
          None => {
            fail!("the data expected wasn't found")
          },
        }
        event.insert(String::from_str("Result"), Foo {bar: String::from_str("HAHAHA")});
    });

    let mut data : HashMap<String, Foo> = HashMap::new();
    data.insert(test.clone(), Foo {bar: data_inside.clone()});
    emitter.emit(test.clone(), & mut data);
    if data[String::from_str("Result")].bar != String::from_str("HAHAHA") { fail!("Doesn't pass back values") }
}
