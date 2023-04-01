use std::io;
use std::io::Write;

//use std::collections::HashMap;
#[derive(Debug)]
struct ToDoEntry {
    item_title: String,
    item_description: String
}

fn main() {
    let mut items_list: Vec<ToDoEntry> = Vec::new();

    menu_print();
    let response = user_input();

    // let (it, id) = add_item();
    // println!("Title: {}", it);
    // println!("Description: {}", id);

    match response.as_str().trim_end() {
        "1" => add_item(&mut items_list),
        "5" => println!("Exiting..."),
        _ => println!("Coming Soon!"),
    };

    for data in &items_list {
        println!("{:#?}", data);
    }

    //println!("{}", items_list[0]);

}

fn add_item(items_list: &mut Vec<ToDoEntry>) {
    println!("Enter an item title.");
    let title = user_input();

    println!("Enter a description of the item.");
    let description = user_input();

    let item_entry = ToDoEntry {
        item_title: title,
        item_description: description
    };

    items_list.push(item_entry);

}







fn menu_print() {
    println!("Please make a selection:");
    println!("1. Add a new item.");
    println!("2. List all items.");
    println!("3. Display an item.");
    println!("4. Delete an item.");
    println!("5. Exit.");
}

fn user_input() -> String{
    //print pseudo prompt
    print!("> ");
    let _ = io::stdout().flush();

    //create a buffer to hold the input
    let mut input = String::new();

    //read from stdin
    let _= io::stdin().read_line(&mut input);

    //print the input
    return input;
}