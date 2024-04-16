mod encounter;
mod stat_search;
mod user_input;

use std::fs;
use std::process::exit;
use std::path::Path;
use serde::{Deserialize, Serialize};
use colored::*;
use std::path::PathBuf;

///
/// Creature struct used for the main menu
///
#[derive(Serialize, Deserialize, Debug)]
struct Creature {
    name: String,
    character_type: String,
    ac: i32,
    hp: i32,
    initiative: i32
}
 
///
/// Parses data from json files for statblocks and encounters
///
fn parse_json(json_data: &str) -> Result<Vec<Creature>, serde_json::Error> {
    let creatures: Vec<Creature> = serde_json::from_str(json_data)?;
    Ok(creatures)
}

///
/// Loads encounter file using [parse_json]
///
fn load_encounter() -> Vec<Creature> {
    let expanded_path = shellexpand::tilde("~/.config/dnd-encounter-tracker/encounter.json").into_owned();
    let path = PathBuf::from(expanded_path);
    let contents = fs::read_to_string(&path).expect("Couldn't read encounter file");
    parse_json(&contents).unwrap()
}

///
/// Main menu that displays current encounter and selected character
///
fn print_creatures(position: usize) {
    let creatures = load_encounter();
    println!("\nCreatures:");
    
    // Variable used for determining selected character
    let mut selector = 1;
    // Used for determining whether or not to print actions (defaults to player)
    let mut creature_stat = "Player".to_string();
    for creature in creatures {
        // True if it's the character's 'turn', false otherwise
        if selector == position {
            if creature.character_type == "Player" {
                let string = format!("{} - {}, AC: {}", creature.initiative, creature.name, creature.ac);
                println!("{} {} {}", "-->".bright_yellow().bold(), string.bright_blue().bold(), "<--".bright_yellow().bold());
            } else {
                let string = format!("{} - {}/{}, AC: {}, HP: {}", creature.initiative, creature.character_type, creature.name, creature.ac, creature.hp);
                println!("{} {} {}", "-->".bright_yellow().bold(), string.bright_red().bold(), "<--".bright_yellow().bold());
                // Changes variable to whatever the selected creature type is, allowing actions to be displayed below 
                creature_stat = creature.character_type.clone();
            }
        } else {
            if creature.character_type == "Player" {
                let string = format!("{} - {}, AC: {}", creature.initiative, creature.name, creature.ac);
                println!("{}", string.blue());
            } else {
                let string = format!("{} - {}/{}, AC: {}, HP: {}", creature.initiative, creature.character_type, creature.name, creature.ac, creature.hp);
                println!("{}", string.red());
            }
        }
        selector += 1;
    }

    // Prints actions if the selected character is not a player
    if creature_stat != "Player" {
        stat_search::print_attributes(&creature_stat);
        stat_search::combat_stats(&creature_stat);
    }
    println!();
}

///
/// Only runs if encounter file does not exist or is empty. Forces initialization of encounter file
///
fn initial_startup_loop(round: usize, position: usize) {
    // Control character printed to clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Welcome to the D&D Combat Tracker!\n");
    println!("Get started by adding some characters:");
    encounter::add_character();
    let creatures = load_encounter();

    // Restarts function if no characters were added
    if creatures.len() == 0 {
        initial_startup_loop(round, position);
    }
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Current Round: {round}");
    print_creatures(position);
}

///
/// Main menu and loop for command input
///
fn main() {
    // Variable used for determining current round
    let mut round = 1;
    // Variable used for determining selected character
    let mut position = 1;
    println!("Welcome to the D&D Combat Tracker!\n");

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        // Checks if encounter file exists, calls initialization function if it doesn't
        let expanded_path = shellexpand::tilde("~/.config/dnd-encounter-tracker/encounter.json").into_owned();
        let path = PathBuf::from(expanded_path);
        let path = Path::new(&path);
        if path.exists() {
            println!("Current Round: {round}");
            print_creatures(position);
        } else {
            initial_startup_loop(round, position);
        }

        // Command loop, allowing user to type commands
        loop {
            let creatures = load_encounter();
            if creatures.len() == 0 {
                round = 1;
                position = 1;
                initial_startup_loop(round, position);
            }

            // Goes to next round if all characters have taken a turn
            if position == creatures.len()+1 {round+=1; position=0; break;}

            println!("Enter a command! Type h for help: ");
            let input: String = user_input::input();
            println!();
            if input == "quit" {
                println!("I hope you enjoyed using this program!");
                exit(0);
            }

            match input.as_str() {
                "n" => {break;},
                
                // Basically just makes sure there aren't any underflow errors
                "p" => {
                    if position == 1 {
                        position = creatures.len()-1;
                        if round == 1 {
                            round = 1
                        } else {
                            round -= 1;
                        }
                    } else {
                        position -=2;
                    }
                    break;
                },
                "s" => {stat_search::statblocks();},
                "a" => {
                    encounter::add_character();
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    println!("Current Round: {}", round);
                    print_creatures(position);
                },
                "e" => {
                    encounter::edit_creature();
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    println!("Current Round: {}", round);
                    print_creatures(position);
                },
                "r" => {
                    encounter::remove_creature();
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    println!("Current Round: {}", round);
                    print_creatures(position);
                },
                "c" => {
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    println!("Current Round: {}", round);
                    print_creatures(position);
                },
                "d" => {
                    encounter::damage_creature();
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    println!("Current Round: {}", round);
                    print_creatures(position);
                },
                "t" => {
                    encounter::attack();
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    println!("Current Round: {}", round);
                    print_creatures(position);
                },
                "h" => {println!("Commands:\n(n)ext character\n(p)revious character\n(s)tat search\n(a)dd creature\n(e)dit stats\n(d)amage creature\na(t)tack action\n(c)lear screen\n(r)emove character\n");},
                _ => {
                    println!("Invalid command!");
                }
            }
        }
        // Moves to next character after a loop break
        position += 1;
    }
}
