fn main() {
    let cat = Pet::Cat;
    let gildong = Person {
        name: String::from("홍길동"),
        active: true,
    };

    meet(&cat, &gildong); // 에러. Person이 Greet을 구현한 것은 맞지만, T타입이 서로 같아야 하는데 Pet과 Person으로 다르기 때문
}

fn meet<T: Greet>(one: &T, another: &T) {
    println!("첫번째가 인사합니다 {}", one.greeting());
    println!("두번째가 인사합니다 {}", another.greeting());
}

trait Greet {
    fn greeting(&self) -> String;
}

enum Pet {
    Dog,
    Cat,
    Tiger,
}

impl Greet for Pet {
    fn greeting(&self) -> String {
        match &self {
            Pet::Dog => String::from("멍멍"),
            Pet::Cat => String::from("야옹"),
            Pet::Tiger => String::from("어흥"),
        }
    }
}

struct Person {
    name: String,
    active: bool,
}

impl Greet for Person {
    fn greeting(&self) -> String {
        String::from("안녕")
    }
}
