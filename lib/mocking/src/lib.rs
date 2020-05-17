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
    pub fn increment(&self) {
        self.0
            .write()
            .expect("Failed to acquire a write lock on node")
            .value += 1;
    }
    pub fn get(&self) -> usize {
        self.0.read().unwrap().value
    }
}

#[derive(Clone)]
pub struct Method<T: Clone + std::fmt::Debug, E: Clone + std::fmt::Debug> {
    return_values: Vec<Option<T>>,
    errors: Vec<Option<E>>,
    method_name: String,
    call_number: Count,
}

impl<T: Clone + std::fmt::Debug, E: Clone + std::fmt::Debug> Method<T, E> {
    fn handle_call(&self) -> Result<T, E> {
        let call = self.call_number.get();
        self.call_number.increment();
        let error = self.errors.get(call).map_or(None, |e| e.clone());
        let return_value = self.return_values.get(call).map_or(None, |v| v.clone());
        self.panic_if_out_of_bounds(&return_value, &error);
        match return_value {
            Some(value) => Ok(value),
            None => Err(error.unwrap()),
        }
    }
    fn panic_if_out_of_bounds<O, S>(&self, a: &Option<O>, b: &Option<S>) {
        if a.is_none() && b.is_none() {
            panic!(format!(
                "No value or error was found to return from call to {}",
                self.method_name
            ))
        }
    }
    pub fn new(method: &str) -> Method<T, E> {
        Method {
            method_name: String::from(method),
            return_values: vec![],
            errors: vec![],
            call_number: Count::new(),
        }
    }
    pub fn returns(&mut self, value: T) -> &mut Method<T, E> {
        self.return_values.push(Some(value));
        self.errors.push(None);
        self
    }
    pub fn throws_error(&mut self, error: E) -> &mut Method<T, E> {
        self.errors.push(Some(error));
        self.return_values.push(None);
        self
    }
    pub fn times_called(&self) -> usize {
        self.call_number.get()
    }
    pub fn call(&self) -> Result<T, E> {
        self.handle_call()
    }
    pub fn call_mut(&mut self) -> Result<T, E> {
        self.handle_call()
    }
}

#[cfg(test)]
mod mocker_tests {
    use super::*;

    #[derive(Clone, Eq, PartialEq, Debug)]
    struct TestStruct;

    struct Container<T: Clone + std::fmt::Debug> {
        pub method: Method<T, &'static str>,
    }

    impl<T: Clone + std::fmt::Debug> Container<T> {
        pub fn new() -> Container<T> {
            Container {
                method: Method::new("test container method"),
            }
        }
        pub fn get(&self) -> Result<T, &'static str> {
            self.method.call()
        }
        pub fn get_mut(&mut self) -> Result<T, &'static str> {
            self.method.call_mut()
        }
    }

    #[test]
    fn it_returns_pre_defined_values_when_call_is_called() {
        let test_value = TestStruct;
        let mut container: Container<&TestStruct> = Container::new();
        container.method.returns(&test_value);
        let result = container.get().unwrap();
        assert_eq!(result, &test_value);
    }

    #[test]
    fn it_returns_pre_defined_values_when_call_mut_is_called() {
        let test_value = TestStruct;
        let mut container: Container<&TestStruct> = Container::new();
        container.method.returns(&test_value);
        let result = container.get_mut().unwrap();
        assert_eq!(result, &test_value);
    }

    #[test]
    fn it_returns_pre_defined_errors_when_call_is_called() {
        let error_value = "test error";
        let mut container: Container<&TestStruct> = Container::new();
        container.method.throws_error(&error_value);
        let error = container.get().err().unwrap();
        assert_eq!(error, error_value);
    }

    #[test]
    fn it_returns_pre_defined_errors_when_call_mut_is_called() {
        let error_value = "test error";
        let mut container: Container<&TestStruct> = Container::new();
        container.method.throws_error(&error_value);
        let error = container.get_mut().err().unwrap();
        assert_eq!(error, error_value);
    }

    #[test]
    fn it_returns_multiple_values_from_call() {
        let test_value = TestStruct;
        let test_value2 = TestStruct;
        let mut container: Container<&TestStruct> = Container::new();
        container.method.returns(&test_value).returns(&test_value2);
        let _result = container.get().unwrap();
        let result2 = container.get().unwrap();
        assert_eq!(result2, &test_value2);
    }

    #[test]
    fn it_returns_multiple_values_from_call_mut() {
        let test_value = TestStruct;
        let test_value2 = TestStruct;
        let mut container: Container<&TestStruct> = Container::new();
        container.method.returns(&test_value).returns(&test_value2);
        let _result = container.get_mut().unwrap();
        let result2 = container.get_mut().unwrap();
        assert_eq!(result2, &test_value2);
    }

    #[test]
    fn it_returns_errors_and_values_from_call() {
        let test_value = TestStruct;
        let test_error = "test error";
        let mut container = Container::<&TestStruct>::new();
        container
            .method
            .returns(&test_value)
            .throws_error(test_error);
        let result = container.get().unwrap();
        let error = container.get().err().unwrap();
        assert_eq!(result, &test_value);
        assert_eq!(error, test_error);
    }

    #[test]
    fn it_returns_errors_and_values_from_call_mut() {
        let test_value = TestStruct;
        let test_error = "test error";
        let mut container = Container::<&TestStruct>::new();
        container
            .method
            .returns(&test_value)
            .throws_error(test_error);
        let result = container.get_mut().unwrap();
        let error = container.get_mut().err().unwrap();
        assert_eq!(result, &test_value);
        assert_eq!(error, test_error);
    }

    #[test]
    #[should_panic]
    fn it_panics_if_call_is_called_and_there_are_no_values_to_return() {
        Container::<TestStruct>::new().get();
    }

    #[test]
    #[should_panic]
    fn it_panics_if_call_mut_is_called_and_there_are_no_values_to_return() {
        Container::<TestStruct>::new().get_mut();
    }

    #[test]
    fn it_records_the_number_of_times_the_call_method_was_called() {
        let test_value = TestStruct;
        let test_value2 = TestStruct;
        let mut container: Container<&TestStruct> = Container::new();
        container.method.returns(&test_value).returns(&test_value2);
        let _result = container.get().unwrap();
        let result2 = container.get().unwrap();
        assert_eq!(container.method.times_called(), 2);
    }

    #[test]
    fn it_records_the_number_of_times_the_call_mut_method_was_called() {
        let test_value = TestStruct;
        let test_value2 = TestStruct;
        let mut container: Container<&TestStruct> = Container::new();
        container.method.returns(&test_value).returns(&test_value2);
        let _result = container.get_mut().unwrap();
        let result2 = container.get_mut().unwrap();
        assert_eq!(container.method.times_called(), 2);
    }
}
