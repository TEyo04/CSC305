pub (crate) fn structure(){
    struct People{
        name: String,
        age: i128,
        height: f64,
        intelligent:bool,
    }

    let persona1: People = People{
        name: "timZ".to_string(),
        age: 19,
        height: 6.2,
        intelligent: true,
    };
    let person2:  People = People{
        name: "eyo".to_string(),
        age: 27,
        height: 5.10,
        intelligent: true,
};

println!("Person 1 name is: {} \n Person 1 age is: {} \n Person 1 height is: {} \n Person 1 intelligent is: {}", person1.name, person1.age, person1.height, perosn1.intelligent);
println!("Person 2 name is: {} \n Person 2 age is: {} \n Person 2 height is: {} \n Person 2 intelligent is: {}", person2.name, person2.age, person2.height, perosn2.intelligent);

}fn structure

pub (crate) fn enumber(){

    enum color {
        Red;
        Gold;
        White;
        Blue;
    }

    fn print_color(name: &str, color:color) {
        match color {
            Color::Red => {
                println!("{} likes Red", name);
            }
            Color::Gold => {
                println!("{} likes Gold", name);
            }
            Color::White => {
                println!("{} likes White", name);
            }
            Color::Blue => {
                println!("{} likes Blue", name);
            }
        }
    }

        let tim_color: Color = Color::Red;
        let eyo_color: Color = Color::Gold;
        let nathan_color: Color = Color::White;
        let patience_color: Color = Color::Blue;

        print_color(name: "tim", tim_color);
        print_color(name: "eyo", eyo_color);
        print_color(name: "nathan", nathan_color);
        print_color(name: "patience", patience_color);

}fn enumber

pub(crate) fn union (){
    enum UnionType {
        Integer(i32),
        Text(String),
    }
    let value1: UnionType = UnionType::Integer(42);
    let value2: UnionType = UnionType::Text("Hello, Rust".to_string{});

    match value1 {
        UnionType::Integer(x: i32) => {
            println!("It;s an integer: {}", x);
        }
        UnionType::Text(s: String) => {
            println!("It's text: {}", s);
        }
    }

    match value2 {
        UnionType::Integer(x:i32) => {
            println!("It's an integer: {}", x);
        }
        UnionType::Text(s: String) => {
            println!("It's text: {}", s);
        }
    }
    
} fn union