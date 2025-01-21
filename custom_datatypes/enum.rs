// The "enum" word allows us to create types of different variants. Any variant which is valid in 
// struct is also valid as an enum


enum WebEvent {

    // An enum variant thats unit-like
    PageLoad,
    PageUnload,

    // enum variant thats like tuple structs
    KeyPress(char),
    Paste(String),

    // or like c structures
    Click {x: i64, y: i64},
}

// A function that takes in WebEvent enum as the input
// Returns nothing

fn inspect (event:WebEvent) {


    match event {

        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),

        WebEvent::KeyPress(c) => println!("Pressed {}", c),
        WebEvent::Paste(s) => println!("Pasted {}", s),

        WebEvent::Click {x,y} => {
            println!("CLicked ar x = {} and y = {} ", x, y);
        },
    }
    
}


fn main(){

    let pressed = WebEvent::KeyPress('x');
    //  to owned allows the varible to play with the memory, change values and stuff.
    // Originaly let str = "hello" is basically &str which means it borrows the data and cant modify it.
    // & means reference to a memory owned by someonw else. 

    let pasted = WebEvent::Paste("my_text".to_owned());
    let click = WebEvent::Click{ x: 20, y:25};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}