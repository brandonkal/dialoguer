extern crate dialoguer;

use dialoguer::theme::ColoredTheme;
use dialoguer::{Input, KeyPrompt};

fn main() {
    let rv = KeyPrompt::with_theme(&ColoredTheme::default())
        .with_text("Do you want to continue?")
        .items(&['y', 'n', 'p'])
        .default(1)
        .interact()
        .unwrap();
    if rv == 'y' {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
        return;
    }

    let input: String = Input::new().with_prompt("Your name").interact().unwrap();
    println!("Hello {}!", input);
}
