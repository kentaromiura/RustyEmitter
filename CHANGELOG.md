Version 2.0.0
=============

- The EventData type has been changed from `HashMap<String, String>` to a more generic `HashMap<String, T>`
- the emit signature has been changed to `fn emit(&mut self, event_name: String, event_data: & mut EventData<T>)` to allow EventData to be modified by the callback
