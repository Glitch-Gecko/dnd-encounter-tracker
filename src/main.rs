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
fn print_creatures(position: usize, round: usize) {
    let creatures = load_encounter();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", format!("╔{:═<70}╗", "═"));
    println!("{}", format!("║{:^70}║", format!("Current round: {round}").bold()));
    println!("{}", format!("╟{:─<70}╢", "─"));
    println!("{}", format!("║{:^70}║", format!("Creatures:").bold()));
    println!("{}", format!("║{:70}║", " "));
    
    // Variable used for determining selected character
    let mut selector = 1;
    // Used for determining whether or not to print actions (defaults to player)
    let mut creature_stat = "Player".to_string();
    for creature in creatures {
        // True if it's the character's 'turn', false otherwise
        if selector == position {
            if creature.character_type == "Player" {
                println!("{}", format!("║{:^109}║", format!("{} {} {}", "-->".bright_yellow(), format!("{} - {}, AC: {}", creature.initiative, creature.name, creature.ac).bright_blue(), "<--".bright_yellow()).bold()));
            } else {
                println!("{}", format!("║{:^109}║", format!("{} {} {}", "-->".bright_yellow(), format!("{} - {}/{}, AC: {}, HP: {}", creature.initiative, creature.character_type, creature.name, creature.ac, creature.hp).bright_red(), "<--".bright_yellow()).bold()));

                // Changes variable to whatever the selected creature type is, allowing actions to be displayed below 
                creature_stat = creature.character_type.clone();
            }
        } else {
            if creature.character_type == "Player" {
                println!("{}", format!("║{:^70}║", format!("{} - {}, AC: {}", creature.initiative, creature.name, creature.ac).bright_blue()));
            } else {
                println!("{}", format!("║{:^70}║", format!("{} - {}/{}, AC: {}, HP: {}", creature.initiative, creature.character_type, creature.name, creature.ac, creature.hp).bright_red()));
            }
        }
        selector += 1;
    }

    // Prints actions if the selected character is not a player
    if creature_stat != "Player" {
        println!("{}", format!("╟{:─<70}╢", "─"));
        stat_search::print_attributes(&creature_stat);
        println!("{}", format!("╚{:═<70}╝", "═"));
        stat_search::combat_stats(&creature_stat);
    } else {
        println!("{}", format!("╚{:═<70}╝", "═"));
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
    print_creatures(position, round);
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
        // Checks if encounter file exists, calls initialization function if it doesn't
        let expanded_path = shellexpand::tilde("~/.config/dnd-encounter-tracker/encounter.json").into_owned();
        let path = PathBuf::from(expanded_path);
        let path = Path::new(&path);
        if path.exists() {
            print_creatures(position, round);
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

            println!("Enter a command! Type h for help menu: ");
            let input: String = user_input::input();
            println!();
            if input == "quit" || input == "done" {
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
                    print_creatures(position, round);
                },
                "e" => {
                    encounter::edit_creature();
                    print_creatures(position, round);
                },
                "r" => {
                    encounter::remove_creature();
                    print_creatures(position, round);
                },
                "c" => {
                    print_creatures(position, round);
                },
                "d" => {
                    encounter::damage_creature();
                    print_creatures(position, round);
                },
                "t" => {
                    encounter::attack();
                    print_creatures(position, round);
                },
                "h" => {println!("Commands:
a: add creature
c: clear screen
d: damage creature
e: edit stats
n: next character
p: previous character
r: remove character
s: stat search
t: attack action
");},
                _ => {
                    println!("Invalid command!");
                }
            }
        }
        // Moves to next character after a loop break
        position += 1;
    }
}
