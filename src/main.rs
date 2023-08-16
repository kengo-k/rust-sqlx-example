mod foo;

mod inner_mod {
    pub fn hello() {
        println!("Hello");
    }

    pub mod inner_inner_mod {
        pub fn world() {
            println!("World");
        }
    }

    #[derive(Debug)]
    pub struct Person {
        pub name: String,
    }

    impl Person {
        pub fn new(name: String) -> Self {
            Self { name }
        }

        pub fn show(&self) {
            println!("{:?}", self);
        }
    }
}

use crate::foo::common::MyEnum;
use inner_mod::inner_inner_mod::world;
use inner_mod::Person;

fn main() {
    inner_mod::hello();
    world();
    let john = Person::new("John".to_string());
    john.show();
    foo::bar::util::print_my_enum(&MyEnum::A);
}
