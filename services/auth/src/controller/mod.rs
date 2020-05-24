// pub mod authorization;
pub mod credentials;
pub mod jwt;
pub mod password;




#[cfg(test)]
mod example {
    use super::*;
    use actix_rt;
    use serde::export::PhantomData;
    struct Phantom;
    struct InnerThing<'a> {
        phantom: PhantomData<&'a Phantom>,
    }

    impl<'a> InnerThing<'a> {
        async fn create() -> InnerThing<'a> {
            InnerThing {
                phantom: PhantomData,
            }
        }
    }

    struct Thing1<'a> {
        thing: InnerThing<'a>,
    }
    trait Thing<'a> {}
    impl<'a> Thing<'a> for Thing1 {}

    #[actix_rt::test]
    async fn example_of_problem() {

    }
}