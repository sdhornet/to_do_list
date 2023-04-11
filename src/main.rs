use std::io;

//#[derive(Debug)]
struct ToDoEntry {
    item_title: String,
    item_description: String,
}

fn main() {
    let mut items_list: Vec<ToDoEntry> = Vec::new();
    loop {
        menu_print();

        //create a buffer to hold the input
        let mut response = String::new();
        //read from stdin
        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line.");

        // match response.as_str().trim_end() {
        //     "1" => add_item(&mut items_list),
        //     "5" => println!("Exiting..."),
        //     _ => println!("Coming Soon!"),
        // };
        //println!("{}", response);

        let response = response.trim();

        if response == "1" {
            println!("Enter an item title.");
            let mut title = String::new();
            let _ = io::stdin().read_line(&mut title);

            println!("Enter a description of the item.");
            let mut description = String::new();
            let _ = io::stdin().read_line(&mut description);

            let item_entry = ToDoEntry {
                item_title: title,
                item_description: description,
            };

            items_list.push(item_entry);
        } else if response == "2" {
            for item in &items_list {
                println!("title: {} desc: {}", item.item_title, item.item_description);
            }
        } else if response == "3" {
            println!("Enter the item you want to display.");
            let mut display_ind = String::new();
            io::stdin()
                .read_line(&mut display_ind)
                .expect("Failed to read line.");

            let test: usize = match display_ind.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            if let Some(_) = items_list.get(test) {
                println!(
                    "title: {} desc: {}",
                    items_list[test].item_title, items_list[test].item_description
                );
            } else {
                println!("Error");
            }
        } else if response == "4" {
            println!("Enter the item you want to delete.");
            let mut delete_ind = String::new();
            io::stdin()
                .read_line(&mut delete_ind)
                .expect("Failed to read line.");

            let test: usize = match delete_ind.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            if let Some(_) = items_list.get(test) {
                items_list.remove(test);
            } else {
                println!("Error");
            }
        } else if response == "5" {
            println!("Quitting...");
            break;
        } else {
            println!("Make a valid choice.");
            continue;
        }
    }
}

fn menu_print() {
    println!("Please make a selection:");
    println!("1. Add a new item.");
    println!("2. List all items.");
    println!("3. Display an item.");
    println!("4. Delete an item.");
    println!("5. Exit.");
}
