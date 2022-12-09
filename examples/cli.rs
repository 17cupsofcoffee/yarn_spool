use std::error::Error;
use std::io::BufRead;

use prost::Message;
use yarn_spool::{expand_substitutions, load_string_table, Dialogue, DialogueEvent, Program};

fn main() -> Result<(), Box<dyn Error>> {
    // A Yarn script compiles down into bytecode (.yarnc files) and
    // string tables (.csv files). These are seperated to allow for
    // localization.
    //
    // To generate the inputs for this example, run:
    // ysc compile examples/script.yarn --output-name=examples/generated/script
    //
    // Once you've compiled the script, you can load it in Rust via
    // yarn_spool's API. Bytecode is represented as a Program, and
    // a string table is represented as a HashMap<String, String>.

    let program_bytes = std::fs::read("examples/generated/script.yarnc")?;
    let string_table_bytes = std::fs::read("examples/generated/script-Lines.csv")?;

    let program = Program::decode(program_bytes.as_slice())?;
    let string_table = load_string_table(&string_table_bytes);

    // To run programs, you need to load them into a Dialogue.

    let mut dialogue = Dialogue::new();
    dialogue.add_program(&program);

    let mut input = String::new();

    // To run the dialogue, call the 'advance' method. This will
    // return when an event occurs that your game may want to
    // handle (or when the program completes).

    while let Some(event) = dialogue.advance() {
        match event {
            // A 'Line' event means that there is a new line of dialogue to display.
            //
            // Depending on your UI, you may want to stop advancing until there is
            // some kind of player input.
            DialogueEvent::Line => {
                let line = dialogue.current_line();

                // To get the text for a line, look up the ID in the string table.
                let raw_text = &string_table[&line.id].text;

                // If the text includes variables, you will need to substitute them
                // in.
                let text = expand_substitutions(raw_text, &line.substitutions);

                // TODO: Markup parsing is not supported yet.

                println!("{}", text);
            }

            // A 'Command' event means that the script wants to execute a command.
            //
            // What these commands do is up to your game engine - a common
            // example is to have a 'wait' command that pauses the
            // dialogue for a set amount of time.
            DialogueEvent::Command => {
                println!("<<{}>>", dialogue.current_command());
            }

            // An 'Options' event means that the script is now waiting for the player
            // to select an option. Until you tell the Dialogue which option has
            // been selected, calling 'advance' will fail.
            DialogueEvent::Options => {
                for opt in dialogue.current_options() {
                    // The process for building the player-facing string is the same as
                    // for a normal line.

                    let raw_text = &string_table[&opt.line.id].text;
                    let text = expand_substitutions(raw_text, &opt.line.substitutions);

                    println!("{}) {}", opt.index, text);
                }

                let option = read_line(&mut input)?;

                dialogue.set_selected_option(option);
            }
        }
    }

    Ok(())
}

fn read_line(buf: &mut String) -> Result<usize, Box<dyn Error>> {
    buf.clear();

    std::io::stdin().lock().read_line(buf)?;

    let value = buf.trim().parse()?;

    Ok(value)
}
