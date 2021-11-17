
trait Animal {
    fn name(&self) -> &'static str;
}

struct Dog;
struct Cat;

impl Animal for Cat {
    fn name(&self) -> &'static str {
        "Cat"
    }
}

impl Animal for Dog {
    fn name(&self) -> &'static str {
        "Dog"
    }
}

// fn name(animal: &impl Animal) -> &'static str {
//     animal.name()
// }

fn name<T: Animal>(animal: &T) -> &'static str {
    animal.name()
}

#[test]
fn should_print_animal_name() {
    let cat = Cat;
    println!("{}", name(&cat));
    println!("{}", name(&cat));
}
