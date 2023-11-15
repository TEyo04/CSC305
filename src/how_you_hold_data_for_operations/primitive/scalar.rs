//scalar types hold single values
pub (crate) fn boolean (){
    let hungry bool =true; //defining a boolean
    if hungry != true{
        println! ("i am hungry")
    }
    else{
        println!("i am not hungry")
    }
}

pub (crate) fn numeric(){
    let age: i32 = 12;
    let height: f64 = 3.45;

    if age > 10 && height > 5.5{
        println!("You are good to go")
    }
    else {
        println!("You are not good to go")
    }
}

pub (crate) fn never() -> ! {
    println!("This function would not return anything");
    panic!("Something went wrong")
}

pub (crate) fn textual () {
    // there are two textuals which are strings and char
    let name: &str = "Timothy";
    let blood_type: char = 'A';
    println! ("My name is {} and my blood type is {}", name, blood_type);
}