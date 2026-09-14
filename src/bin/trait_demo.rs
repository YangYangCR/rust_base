pub enum TypeEnum {
    Type1,
    Type2,
    Type3,
}

struct Dog {
    name: String,
    age: u16,
    weight: Option<u16>,
    is_anim: bool,
    type_enum: TypeEnum,
}

impl Dog {
    // 参数中有self, 使用 let dog = Dog{} ; dog.show()
    fn show(&self) {}
    //参数中没有self ,则使用Dog::new 类似于静态方法
    fn new(data: String) {}
}

struct Cat {
    name: String,
}

trait Eat {
    fn eat(&self) -> Option<String>;
}

trait Run {
    fn run(&self) -> Result<String, String>;
}

impl Eat for Dog {
    fn eat(&self) -> Option<String> {
        println!("{} eat {} age", self.name, self.age);
        Option::from("Dog".to_string())
    }
}

impl Run for Cat {
    fn run(&self) -> Result<String, String> {
        Result::Ok(self.name.to_string())
    }
}

// ? 用法 要求 函数返回 Result 或 Option
fn main() {
    let dog = Dog {
        name: String::from("dh"),
        age: 16,
        weight: Some(20),
        is_anim: true,
        type_enum: TypeEnum::Type1,
    };
    dog.eat();
    dog.show();
    Dog::new(String::from("dahuang"));
    let weight = match dog.weight {
        Some(weight) => {
            println!("Weight is {}", weight);
            weight
        }
        None => {
            println!("weight is None");
            return;
        }
    };
    println!("age is {}", weight);

    if dog.is_anim {
        println!("Dog is Anim");
    }

    match dog.type_enum {
        TypeEnum::Type1 => {
            println!("Dog is Type1");
        }
        TypeEnum::Type2 | TypeEnum::Type3 => {
            print!("Dog is Type2 or Type3");
        }
    }

    match dog.eat() {
        Some(ss) => println!("{}", ss),
        None => (),
    }
}

//  cat.run()? 相当于
//     match cat.run() {
//         Ok(s) => Result::Ok(s),
//         Err(e) => Result::Err(e),
//     }
fn test_wen() -> Result<String, String> {
    let cat = Cat {
        name: String::from("cat"),
    };
    return cat.run();
}
