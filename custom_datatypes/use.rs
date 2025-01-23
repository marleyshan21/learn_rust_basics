// "use" declaration is used so that manual scoping is not needed
/*
This means that everytime we would have to give the full path to import. 
if we use "use", we bring them to the local scope directly. 

Its like using import in other languages

*/




#![allow(dead_code)]


enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student, 
    Teacher,
}

fn main(){

    // Using manual scoiping: basically giving the entire path
    // have to wrtie Stage::Beginner everytime.
    
    //  let stage = Stage::Beginner
    // let role = Stage::Role


    // LEts use the "use" word everytime
    // manually scoping
    use crate::Stage::{Beginner, Advanced};
    // automatically use all the names inside Role
    use crate::Role::*;

    // Equivalent to Stage::Beginner
    let stage = Beginner;
    let role = Teacher;

    match stage {
        Beginner => println!("Chose beginner"),
        Advanced => println!("Chose advanced"),
    }

    match role {
        Student => println!("A student"),
        Teacher => println!("A teacher"),
    }
}

