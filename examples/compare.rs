use anyhow::Result;
use enum_dispatch::enum_dispatch;
use std::env;

fn main() -> Result<()> {
    println!("{:?}", env::current_dir()?);
    dotenv::from_filename("examples/.env").ok();

    let r#type = env::var("Animal")?;

    let cat = Cat("mimi".to_owned());
    let dog = Dog("wowo".to_owned());
    let animal: &dyn Animal = match r#type.as_str() {
        "cat" => &cat,
        "dog" => &dog,
        _ => unimplemented!("animal type: {}", r#type),
    };
    say_name_dyn(animal);

    match r#type.as_str() {
        "cat" => say_name_dyn(&Cat("mimi".to_owned())),
        "dog" => say_name_dyn(&Dog("wowo".to_owned())),
        _ => unimplemented!("animal type: {}", r#type),
    };

    let animal: Box<dyn Animal> = match r#type.as_str() {
        "cat" => Box::new(Cat("mimi".to_owned())),
        "dog" => Box::new(Dog("wowo".to_owned())),
        _ => unimplemented!("animal type: {}", r#type),
    };
    say_name_dyn_box(animal);

    match r#type.as_str() {
        "cat" => say_name_impl(Cat("mimi".to_owned())),
        "dog" => say_name_impl(Dog("wowo".to_owned())),
        _ => unimplemented!("animal type: {}", r#type),
    };

    match r#type.as_str() {
        "cat" => say_name_generics(Cat("mimi".to_owned())),
        "dog" => say_name_generics(Dog("wowo".to_owned())),
        _ => unimplemented!("animal type: {}", r#type),
    };

    let app_state = match r#type.as_str() {
        "cat" => AppState::Cat(Cat("mimi".to_owned())),
        "dog" => AppState::Dog(Dog("wowo".to_owned())),
        _ => unimplemented!("animal type: {}", r#type),
    };

    say_name_enum(app_state);
    Ok(())
}

#[enum_dispatch(Animal)]
enum AppState {
    Cat(Cat),
    Dog(Dog),
}

// impl std::ops::Deref for AppState {
//     type Target = dyn Animal;
//     fn deref(&self) -> &Self::Target {
//         match self {
//             AppState::Cat(cat) => cat,
//             AppState::Dog(dog) => dog,
//         }
//     }
// }

#[enum_dispatch]
trait Animal {
    fn name(&self) -> String;
}

struct Cat(String);

impl Animal for Cat {
    fn name(&self) -> String {
        format!("cat name: {}", self.0)
    }
}

struct Dog(String);

impl Animal for Dog {
    fn name(&self) -> String {
        format!("dog name: {}", self.0)
    }
}

#[allow(dead_code)]
fn say_name_dyn(animal: &dyn Animal) {
    println!("say_name_dyn [{}]", animal.name())
}

#[allow(dead_code)]
fn say_name_dyn_box(animal: Box<dyn Animal>) {
    println!("say_name_dyn_box [{}]", animal.name())
}

#[allow(dead_code)]
fn say_name_impl(animal: impl Animal) {
    println!("say_name_impl [{}]", animal.name())
}

#[allow(dead_code)]
fn say_name_generics<T: Animal>(animal: T) {
    println!("say_name_generics [{}]", animal.name())
}

#[allow(dead_code)]
fn say_name_enum(status: AppState) {
    // Normal
    // let name = match status {
    //     AppState::Cat(cat) => cat.0,
    //     AppState::Dog(dog) => dog.0,
    // };
    // println!("say_name_enum [{}]", name)

    // Deref
    // println!("say_name_enum [{}]", status.name());

    // `enum_dispatch`
    println!("say_name_enum [{}]", status.name())
}
