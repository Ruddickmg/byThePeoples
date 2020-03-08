use std::collections::hash_map::HashMap;

mod client;
mod connection_pool;
mod transaction;

pub use client::MockClient as Client;
pub use connection_pool::MockConnectionPool as ConnectionPool;
pub use transaction::MockTransaction as Transaction;

struct Mocker<T, A, E> {
    return_values: HashMap<u32, T>,
    arguments: HashMap<u32, A>,
    errors: HashMap<u32, E>,
    method_name: String,
    call: u32,
    calls: u32,
}

impl<T, A, E> Mocker<T, A, E> {
    pub fn new(method: &str) -> Mocker<T, A, E> {
        Mocker {
            method_name: String::from(method),
            return_values: HashMap::new(),
            arguments: HashMap::new(),
            errors: HashMap::new(),
            call: 0,
            calls: 0,
        }
    }
    pub fn returns(mut self, value: T) -> Mocker<T, A, E> {
        self.return_values.insert(self.call, value);
        self
    }
    pub fn called_with(mut self, args: A) -> Mocker<T, A, E> {
        self.arguments.insert(self.call, args);
        self
    }
    pub fn throw_error(mut self, error: E) -> Mocker<T, A, E> {
        self.errors.insert(self.call, error);
        self
    }
    pub fn then(mut self) -> Mocker<T, A, E> {
        self.call += 1;
        self
    }
    fn call(mut self, args: Option<A>) -> Result<T, E> {
        let call = self.calls;
        let first: u32 = 0;
        let mut result: Result<T, E>;
        if let Some(arg) = args {
            if self.arguments.contains_key(&call) {
                if arg != self.arguments.remove(&call).unwrap() {
                    panic!(format!(
                        "Mismatched argument found in call #{} to the \"{}\" method",
                        call, self.method_name
                    ))
                };
            }
        }
        if self.errors.contains_key(&call) {
            result = Err(self.errors.remove(&call).unwrap())
        } else if self.return_values.contains_key(&call) {
            result = Ok(self.return_values.remove(&call).unwrap())
        } else if self.return_values.contains_key(&first) {
            result = Ok(self.return_values.remove(&first).unwrap())
        } else {
            panic!(format!(
                "No mock return or error was set for call #{} to the \"{}\" method",
                call, self.method_name
            ));
        };
        self.calls += 1;
        result
    }
}
