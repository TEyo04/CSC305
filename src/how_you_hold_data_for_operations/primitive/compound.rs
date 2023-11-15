//the compounds can hold multiple values
pub (crate) fn tuple () {
    let person: (&str, i32, f64, char, bool) = ("timothy", 12, 12.4, A, true); // defining a tuple with a string, integer,float and char
    //
    let name: &str = person.0;
    let age: i32 = person.1;
    let height: f64 = person.2;
    let bloodtype: char = person.3;
    let beauty: bool = person.4;
        println!("{:?}", name);
        println!("{:?}", age);
        println!("{:?}", height);
        println!("{:?}", bloodtype);
        println!("{:?}", beauty);
}

pub (create) fn array (){
    // it only stores one data type
    //assigning items into the array
    let people_name: [&str; 3] = ["timmy", "nathan", "titi"];
    let people_age: [i32; 3] = [12,2,3];
    let people_height: [f64; 3] = [6.3,5.4,5.6];
    let people_bloodtype: [char;3] = ['A', 'B', 'A'];
    let people_beauty:  [bool;3] = [true,true,false];
    //assigning variables from our array to people in tuple
    let person1: (&str, i32, f64, char, bool) = (
        people_name[0],
        people_age[0],
        people_height[0],
        people_bloodtype[0],
        people_beauty[0],
        );
    let person2: (&str, i32, f64, char, bool) = (
        people_name[2],
        people_name[2],
        people_height[2],
        people_bloodtype[2],
        people_beauty[2],
    );

    println:(
        "1st person's details: Name: {}, Age: {}, Height: {}, Blood Type: {}, Beauty: {}",
        person1.0, person1.1, person1.2, persona1.3, person1.4
);

    println:(
        "2nd person's details: Name: {}, Age: {}, Height: {}, Blood Type: {}, Beauty: {}",
        person1.0 person2.1, person2.2, person2.3, person2.4
);

    println:(
         "3rd person's details: Name: {}, Age: {}, Height: {}, Blood Type: {}, Beauty: {}",
            person3.0 person3.1, person3.2, person3.3, person3.4
);

} fn array

pub (crate) fn slicer(){
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice =: &[i32] = &numbers [2...9];
    println!("Original Array: {:?}", numbers);
    println!("Slice: {:?}", slice);
}

