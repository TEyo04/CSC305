pub mod hold_data;
use hold_data::derived::user_defined;
use hold_data::primitive::compound;
use hold_data::primitive::scalar;


fn scalar_examples (){

    scalar::boolean();
    scalar::numeric();
    scalar::textual();
    let numb: i32 = 0;
    if numb>1 {
        scalar::never();
    }
    else {
        println!("We just avoided scalar never")
    }
}

fn compound_example(){
    compound::array();
    compound::tuple();
    compound::slicer();

}

fn compound_example(){
    compound::array();
    compound::tuple();
    compound::slicer();
}

fn user_defined_example(){
    user_defined::structure();
    user_defined::enumber();
    
}












































































































































































