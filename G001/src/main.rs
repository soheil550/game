use std::io::stdin;
fn main() {
    println!("Hi, what's your name: ");
    let name = getting_username();
    println!("Hello {:?}", name);

    let visitor_list = ["bert", "steve", "fred"];
    let mut allow_them_in = false;
    for visitor in visitor_list {
        if visitor == name {
            allow_them_in = true;
        }
    }
    if allow_them_in {
        println!("Welcome to the Treehouse, {}", name);
    } else {
        println!("Sorry, you aren't on the list.");
    }
}

fn getting_username() -> String {
    /*
     * This function is for getting username from screen.
     */
    let mut username = String::new();
    stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    username.trim().to_lowercase()
}
