use std::sync::{Arc, RwLock};

type CountRef = Arc<RwLock<_Count>>;

#[derive(Clone)]
struct _Count {
    value: usize,
}

#[derive(Clone)]
struct Count(CountRef);

impl Count {
    pub fn new() -> Count {
        let count = _Count { value: 0 };
        Count(Arc::new(RwLock::new(count)))
    }
    pub fn increment(&self) -> usize {
        self.0
            .write()
            .expect("Failed to acquire a write lock on node")
            .value += 1;
        self.get()
    }
    pub fn get(&self) -> usize {
        self.0.read().unwrap().value
    }
}

#[derive(Clone)]
pub struct Method<T: Clone, E: Clone> {
    return_values: Vec<Option<T>>,
    errors: Vec<Option<E>>,
    method_name: String,
    call_number: u32,
    call: Count,
}

impl<T: Clone, E: Clone> Method<T, E> {
    pub fn new(method: &str) -> Method<T, E> {
        Method {
            method_name: String::from(method),
            return_values: vec![],
            errors: vec![],
            call_number: 0,
            call: Count::new(),
        }
    }
    pub fn returns(mut self, value: T) -> Method<T, E> {
        self.return_values.push(Some(value));
        self
    }
    pub fn throws_error(mut self, error: E) -> Method<T, E> {
        self.errors.push(Some(error));
        self
    }
    pub fn then(mut self) -> Method<T, E> {
        let call = self.call_number as usize;
        if self.return_values.len() <= call {
            self.return_values.push(None);
        }
        if self.errors.len() <= call {
            self.errors.push(None)
        }
        self.call_number += 1;
        self
    }
    fn handle_call(&self, return_value: Option<T>, error: Option<E>) -> Result<T, E> {
        let error_is_none = error.is_none();
        if error_is_none && return_value.is_none() {
            panic!(format!(
                "No tests return or error was set for a call to the \"{}\" method",
                self.method_name
            ))
        } else if error_is_none {
            Ok(return_value.unwrap())
        } else {
            Err(error.unwrap())
        }
    }
    pub fn call(self) -> Result<T, E> {
        let call = self.call.get();
        let error = self.errors.get(call).map(|e| e.clone());
        let return_value = self.return_values.get(call).map(|v| v.clone());
        self.call.increment();
        self.handle_call(return_value.unwrap(), error.unwrap())
    }
    pub fn call_ref(&self) -> Result<T, E> {
        let call = self.call.get();
        let error = self.errors.get(call).map(|e| e.clone());
        let return_value = self.return_values.get(call).map(|v| v.clone());
        self.call.increment();
        self.handle_call(return_value.unwrap(), error.unwrap())
    }
    pub fn call_mut(mut self) -> Result<T, E> {
        let error = self.errors.remove(0);
        let return_value = self.return_values.remove(0);
        self.handle_call(return_value, error)
    }
    pub fn call_mut_ref(&mut self) -> Result<T, E> {
        let error = self.errors.remove(0);
        let return_value = self.return_values.remove(0);
        self.handle_call(return_value, error)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
