#[warn(unused_assignments)]
// #![allow(non_snake_case)]
use std::{thread, time::Duration};
use doe::clipboard::set_clipboard;
use doe::keyboard::keyboard::key_press;
use doe::keyboard::keyboard::key_release;
use std::collections::HashMap;
use doe::keyboard::KeyCode;
use std::io;
use rdev::{listen, Event};
use std::sync::{Arc, Mutex};

// use dioxus::prelude::*;

// fn App(cx: Scope) -> Element {
//     cx.render(rsx! {
//         div {
//             "Hello, world!"
//         }
//     })
// }


fn main() {

//hashmap

// Shared array
let shared_array = Arc::new(Mutex::new(Vec::new()));
let array_clone = Arc::clone(&shared_array);


let callback = move |event: Event| {
    // println!("My callback {:?}", event);
    match event.name {
        Some(string) => {
            let mut array = array_clone.lock().unwrap();
            // println!("String pressed: {}", string);
            if &string == "\u{8}"  {
                array.pop();
            }
            else if &string == " " || &string == "\r" {
                array.clear();
            }
            // else if &string == ";" {
            //     array.clear();
            //     array.push(string);
            // }
            else if string.chars().all(char::is_alphanumeric) || &string == "," || &string == "." || &string == ":" || &string == "~" || &string == ";" {
                array.push(string);
            }
        },
        None => (),
    }
  
};
// This will block.
let handle = thread::spawn(move || {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}); 
let shared_map: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
let map1 = Arc::clone(&shared_map);

fn parse_snippet(snippet: String) -> Vec<(usize, usize)> {
    let mut inside_var = false;
    let mut start_index: usize = 0;
    let mut positions: Vec<(usize, usize)> = Vec::new();

    for (index, c) in snippet.chars().enumerate() {
        if c == '{' && snippet.chars().nth(index + 1).unwrap_or(' ') == '{' {
            inside_var = true;
            start_index = index;
        } else if c == '}' && snippet.chars().nth(index - 1).unwrap_or(' ') == '}' && inside_var {
            let variable_length = index - start_index - 2; // Subtract 2 for the double braces
            positions.push((start_index, variable_length + 1));
            inside_var = false;
        }
    }
    positions

}
fn replace_variables(snippet: &str, positions: Vec<(usize, usize)>, replacements: Vec<&str>) -> String {
    let mut result = snippet.to_string();
    for (pos, (start, length)) in positions.iter().enumerate() {
        let end = start + length + 2; // +2 to account for the double curly braces
        let var_start = snippet.char_indices().nth(*start).map_or(0, |(pos, _)| pos);
        let var_end = snippet.char_indices().nth(end).map_or_else(|| snippet.len(), |(pos, _)| pos);
        let var_slice = &snippet[var_start..var_end];
        println!("End: {}, var_start: {}, var_end: {}, var_slice: {}", end, var_start, var_end, var_slice );
        if replacements.len() > pos {
            result = result.replace(var_slice, replacements[pos]);
        }
        else {
            result = result.replace(var_slice, &var_slice[2..(var_slice.len()-2)]);
        }
    }
    result
}


let handle1 = thread::spawn(move || {
    let mut current_length = 0;
    loop {
        let mut array: std::sync::MutexGuard<'_, Vec<String>> = shared_array.lock().unwrap();
        let map = map1.lock().unwrap();

        if array.len() == current_length {
            continue;
        }
        current_length = array.len();
        let user_type: String = ((*array).join("")).to_string();

        let split_user_type: Vec<&str> =  user_type.split(":~").collect();

        let key_part: String;
        let var_part: Vec<&str>;
        if split_user_type.len() > 1 {
            key_part = split_user_type[1].to_string();
            var_part = split_user_type[0].split(",").collect();
            
        } 
        else {
            key_part = user_type.clone();
            var_part = vec![];
        }


        if  map.contains_key(&key_part) {
            let snip = map.get(&key_part);
            match snip {
                Some(color) => {
                    let positions = parse_snippet(color.clone());
                    for (start, length) in positions.clone() {
                        println!("Variable starts at index {} with length {}", start, length);
                    }
                    println!("VARIABLES READ: {:?}", var_part);
                    let result = replace_variables( color, positions, var_part);
                    println!("Result after replacement: {}", result);

                    set_clipboard(result).unwrap()
                },
                None => continue,
            }
            let string_to_match: &str = &user_type;
            let string_to_match_length: usize = string_to_match.len() + 2;
            // set_clipboard(snip).unwrap();
            // println!("{}, {}", {{"a"}}, {{"b"}})
            
            // thread::sleep(Duration::from_millis(2));
            key_press(KeyCode::SHIFT);
            for _i in 0..string_to_match_length {
                key_press(KeyCode::LEFT_ARROW);
                key_release(KeyCode::LEFT_ARROW);
                thread::sleep(Duration::from_millis(2));
            }
            
            // for _i in 0..string_to_match_length {
            //     key_press(KeyCode::RIGHT_ARROW);
            //     key_release(KeyCode::RIGHT_ARROW);
            //     thread::sleep(Duration::from_millis(6));
            // }
            key_press(KeyCode::INSERT);
            key_release(KeyCode::SHIFT);
            key_release(KeyCode::INSERT);
            array.clear();
    }
        // thread::sleep(Duration::from_secs(3));
    }
});
// dioxus_desktop::launch(App);


let map2 = Arc::clone(&shared_map);

let input_thread = thread::spawn(move || {
    
    loop {
        
        let mut input = String::new();
        println!("Please write a text to expand like .mom>=yo mama fat and ugly: ");
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                    let mut map: std::sync::MutexGuard<'_, HashMap<String,String>> = map2.lock().unwrap();
                    // Process the input here
                    let input_array: Vec<&str> = input.split("~").collect();
                
                    if input_array.len() != 2 {
                        println!("Num args got: {}, Nums args expected: 2", input_array.len());
                        // continueand also
                    }
                    else {
                        map.insert(input_array[0].trim().to_string(), input_array[1].trim().to_string());
                        println!("You entered: {:?}", [input_array[0].trim().to_string(), input_array[1].trim().to_string()]);
                    }
                }
                Err(error) => {
                    println!("Error reading input: {}", error);
                }
            }
             thread::sleep(Duration::from_millis(50));

        }
    });


handle.join().unwrap();
handle1.join().unwrap();
input_thread.join().unwrap();

// Now you can access the shared array here

}


// coco -k omer -v "console.log(var, bar)"
// "console.log($var, $bar)" Which variables do you want to keep.
// a

// omer:var1,var2

// omer
// "console.log(var, bar)"
// var1:omer
// "console.log(var1, bar)"
// var1,bar1:omer
// "console.log(var1, bar1"
// _,bar1:omer
// "console.log(var,bar1)"

// omer
// "console.log($var)"
// omer:"hello world"


// omer==console.log(var)


// var1:omer


// var1,var2,var3:omer
// "for $i in range($x),,"
