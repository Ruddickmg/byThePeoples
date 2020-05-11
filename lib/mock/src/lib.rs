pub struct Method<T, E> {
    return_values: Vec<Option<T>>,
    errors: Vec<Option<E>>,
    method_name: String,
    call_number: u32,
    calls: u32,
}

impl<T, E> Method<T, E> {
    pub fn new(method: &str) -> Method<T, E> {
        Method {
            method_name: String::from(method),
            return_values: vec![],
            errors: vec![],
            call_number: 0,
            calls: 0,
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
    pub fn call(&mut self) -> Result<T, E> {
        let call: usize = self.calls as usize;
        let result: Result<T, E>;
        if let Some(error) = self.errors.remove(call) {
            result = Err(error);
        } else if let Some(return_value) = self.return_values.remove(call) {
            result = Ok(return_value);
        } else {
            panic!(format!(
                "No tests return or error was set for call #{} to the \"{}\" method",
                call, self.method_name
            ));
        };
        self.calls += 1;
        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
