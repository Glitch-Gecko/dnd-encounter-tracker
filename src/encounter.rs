use crate::user_input;
use serde::{Serialize, Deserialize};
use std::fs;
use crate::stat_search;
use titlecase::titlecase;
use colored::*;
use rand::Rng;
use std::path::PathBuf;
extern crate shellexpand;

///
/// Character struct used for printing basic stats on the main menu
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    pub name: String,
    pub character_type: String,
    pub ac: i32,
    pub hp: i32,
    pub initiative: i32,
}

///
/// Loads encounter file if it exists
///
fn load_encounter_file() -> Vec<Character> {
    let expanded_path = shellexpand::tilde("~/.config/dnd-encounter-tracker/encounter.json").into_owned();
    let path = PathBuf::from(expanded_path);
    let content = fs::read_to_string(&path).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&content).unwrap_or_else(|_| {
        println!("Error parsing JSON. Starting with an empty list.");
        Vec::new()
    })
}

///
/// Saves encounter file to ~/.config/dnd-encounter-tracker/encounter.json and creates directory if it doesn't exist
///
fn save_encounter_file(characters: &mut Vec<Character>) {
    // Sorts characters by initiative (doesn't take dex into account)
    characters.sort_by_key(|char| -char.initiative);

    // Saves changes to encounter file, and creates the folder structure
    let json_characters = serde_json::to_string_pretty(&characters).unwrap();
    let expanded_path = shellexpand::tilde("~/.config/dnd-encounter-tracker").into_owned();
    let path = PathBuf::from(expanded_path);
    std::fs::create_dir_all(&path).expect("Failed to create directory");
    let expanded_path = shellexpand::tilde("~/.config/dnd-encounter-tracker/encounter.json").into_owned();
    let path = PathBuf::from(expanded_path);
    std::fs::write(&path, json_characters).expect("Unable to write to file");
}

fn input_break_check(input: &str) -> usize {
    match input {
        "done" => 0,
        "0" => 0,
        _ => 1
    }
}

///
/// Grabs stats from [stat_search] and rolls for a hit and damage against a character's AC
///
pub fn attack() {
    let characters = load_encounter_file();
    
    // Initializes damage output strings to null
    let mut attack_string_1 = "Null".to_string();
    let mut attack_string_2 = "Null".to_string();

    loop {
        print_creatures(&characters);

        // Checks if attack strings exist, and prints them if they do
        if attack_string_1 != "Null" {
            println!("{}", attack_string_1);
            if attack_string_2 != "Null" {
                println!("{}\n", attack_string_2);
            } else {println!();}
        }

        println!("Enter the number of the attacking creature (can't be a player), or type \"0\" to return:");
        let attacker: usize = user_input::usize_input();
        if input_break_check(attacker.to_string().as_str()) == 0 || attacker > characters.len() || &characters[attacker-1].character_type == "Player" {
            break;
        }

        println!("\nEnter the number of the attacked creature, or type \"0\" to return:");
        let attacked: usize = user_input::usize_input();
        if input_break_check(attacked.to_string().as_str()) == 0 || attacked > characters.len() {
            break;
        }

        // Displays attacks based on the character's type in the encounter file
        println!();
        let attack_count = stat_search::print_attacks(&characters[attacker-1].character_type);

        println!("Enter the number of the attack, or type \"0\" to return:");
        let attack_number: usize = user_input::usize_input();
        println!();
        if input_break_check(attack_number.to_string().as_str()) == 0 || attack_number > attack_count {
            break;
        }

        // Actually loads the attack, resetting if it's invalid
        let attack_var = stat_search::get_attack(&characters[attacker-1].character_type, attack_number);
        if attack_var.damage_type == "Null" {
            break;
        }

        // Rolls for attack and damage, comparing it to target's AC
        let attack_roll = rand::thread_rng().gen_range(1..21) + attack_var.attack_modifier;
        if attack_roll - attack_var.attack_modifier == 20 {
            attack_string_1 = format!("Rolled a nat 20 and dealt a critical hit!");
            let mut damage_roll = rand::thread_rng().gen_range(1..attack_var.damage_dice[1]+1);
            for _ in 0..attack_var.damage_dice[0] {
                damage_roll += rand::thread_rng().gen_range(1..attack_var.damage_dice[1]+1);
            }
            attack_string_2 = format!("This dealt {}+{} = {} {} damage", damage_roll, attack_var.damage_bonus, damage_roll+attack_var.damage_bonus, attack_var.damage_type);
        } else {
            if attack_roll >= characters[attacked-1].ac {
                attack_string_1 = format!("Rolled a {}+{} = {} against {}/{}'s AC of {}, and hit", attack_roll-attack_var.attack_modifier, attack_var.attack_modifier, attack_roll, characters[attacked-1].character_type, characters[attacked-1].name, characters[attacked-1].ac);
                let mut damage_roll = rand::thread_rng().gen_range(1..attack_var.damage_dice[1]+1);
                for _ in 0..attack_var.damage_dice[0]-1 {
                    damage_roll += rand::thread_rng().gen_range(1..attack_var.damage_dice[1]+1);
                }
                attack_string_2 = format!("This dealt {}+{} = {} {} damage", damage_roll, attack_var.damage_bonus, damage_roll+attack_var.damage_bonus, attack_var.damage_type);
            } else {
                attack_string_1 = format!("Rolled a {}+{} = {} against {}/{}'s AC of {}, and missed", attack_roll-attack_var.attack_modifier, attack_var.attack_modifier, attack_roll, characters[attacked-1].character_type, characters[attacked-1].name, characters[attacked-1].ac);
                attack_string_2 = "Null".to_string();
            }
        }
    }
}

///
/// Loads encounter file and allows for edits, then saves any modifications to the file
///
pub fn edit_creature() {
    let mut characters = load_encounter_file();

    loop {
        print_creatures(&characters);
        println!("Enter the number of a creature to edit, or type \"0\" to return: ");
        let number: usize = user_input::usize_input();
        if input_break_check(number.to_string().as_str()) == 0 || number > characters.len() {
            break;
        }

        println!("{}", format!("╔{:═<35}╗", "═"));
        println!("{}", format!("║{:^35}║", format!("Editing {}/{}", characters[number-1].character_type, characters[number-1].name).bold()));
        println!("{}", format!("╟{:─<35}╢", "─"));
        println!("{}", format!("║{:^35}║", format!("1. Name: {}", characters[number-1].name)));
        println!("{}", format!("╟{:┄<35}╢", "┄"));
        println!("{}", format!("║{:^35}║", format!("2. Race: {}", characters[number-1].character_type)));
        println!("{}", format!("╟{:┄<35}╢", "┄"));
        println!("{}", format!("║{:^35}║", format!("3. AC: {}", characters[number-1].ac)));
        println!("{}", format!("╟{:┄<35}╢", "┄"));
        println!("{}", format!("║{:^35}║", format!("4. HP: {}", characters[number-1].hp)));
        println!("{}", format!("╟{:┄<35}╢", "┄"));
        println!("{}", format!("║{:^35}║", format!("5. Initiative: {}", characters[number-1].initiative)));
        println!("{}", format!("╚{:═<35}╝", "═"));
        println!("\nEnter the number of the field to edit:");
        let input: usize = user_input::usize_input();
        if input_break_check(input.to_string().as_str()) == 0 || input > 5 {
            break;
        }
        
        println!();
        match input {
            1 => {
                println!("Enter new name:");
                characters[number-1].name = titlecase(&user_input::input());
            },
            2 => {
                println!("Enter new race:");
                characters[number-1].character_type = titlecase(&user_input::input());
            },
            3 => {
                println!("Enter new AC:");
                characters[number-1].ac = user_input::int_input();
            },
            4 => {
                println!("Enter new HP:");
                characters[number-1].hp = user_input::int_input();
            },
            5 => {
                println!("Enter new initiative:");
                characters[number-1].initiative = user_input::int_input();
            }
            _ => {
                println!("Invalid input!");
                break;
            }
        }
        // Saves new file contents
        save_encounter_file(&mut characters);
    }
}

///
/// Takes a vector of characters and prints each character in number order, allowing for selection in different functions
///
fn print_creatures(characters: &Vec<Character>) {
    // Escape code to clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    
    // This variable is used to actually number each creature
    let mut number = 1;
    if characters.len() != 0 {
        println!("{}", format!("╔{:═<35}╗", "═"));
        println!("{}", format!("║{:^35}║", format!("Current Encounter:").bold()));
        for creature in characters {
            if number == 1 {
                println!("{}", format!("╟{:─<35}╢", "─"));
            } else {
                println!("{}", format!("╟{:┄<35}╢", "┄"));
            }
            if creature.character_type == "Player" {
                let string = format!("{}. PC/{}", number, creature.name);
                println!("║{:^35}║", string.blue());
            } else {
                let string = format!("{}. {}/{}, {} HP", number, creature.character_type, creature.name, creature.hp);
                println!("║{:^35}║", string.red());
            }
            number +=1;
        }
        println!("{}\n", format!("╚{:═<35}╝", "═"));
    }
}

///
/// Function used to remove a creature from the encounter list
///
pub fn remove_creature() {
    let mut characters = load_encounter_file();
    
    loop {
        if characters.len() == 0 {
            break;
        }
        print_creatures(&characters);
        println!("Enter the number of a creature to remove, or type \"0\" to return: ");
        let number: usize = user_input::usize_input();
        if input_break_check(number.to_string().as_str()) == 0 || number > characters.len() {
            break;
        }
        println!();

        characters.remove(number-1);

        save_encounter_file(&mut characters);
    }
}

/// 
/// Function used to apply damage to a creature based on the creature's number (refer to [print_creatures])
///
pub fn damage_creature() {
    let mut characters = load_encounter_file();
    
    loop {
        print_creatures(&characters);
        println!("Enter the number of a creature to damage, or type \"0\" to return: ");
        let number: usize = user_input::usize_input();
        if input_break_check(number.to_string().as_str()) == 0 || number > characters.len() {
            break;
        }

        println!("\nDamaging {}/{}", characters[number-1].character_type, characters[number-1].name);
        println!("Enter damage dealt (negatives are used for healing):");
        let damage: i32 = user_input::int_input();
        characters[number-1].hp -= damage;

        save_encounter_file(&mut characters);
    }
}

///
/// Uses [stat_search] to display and load monster information
///
fn add_monster() -> Character {
    println!("Enter monster type (type ls for a list of monsters):");
    let input = user_input::input();
    match input.as_str() {
        "ls" => {println!();stat_search::print_monsters();println!();add_monster()},
        _ => {
            // Returns the monster loaded based on the user's input
            stat_search::load_monster(input)
        }
    }
}

///
/// Allows user input for a player character
///
fn add_player() -> Character {
    println!("Enter player name:");
    let name = titlecase(&user_input::input());
    println!("\nEnter {}'s AC:", name);
    let ac = user_input::int_input();

    // Note that hp is not used, but is necessary for the Character struct
    let hp = 999999;
    println!("\nEnter {}'s rolled initiative:", name);
    let initiative = user_input::int_input();
    println!("\nPlayer {} added!\n", name);

    // Returns the character
    Character {
        name,
        character_type: "Player".to_string(),
        ac,
        hp,
        initiative,
    }
}

///
/// Adds either a monster or player using [add_monster] and [add_player]
///
pub fn add_character() {
    let mut characters = load_encounter_file();

    loop {
        print_creatures(&characters);
        println!("Add either a (m)onster or (p)layer, or type \"done\" to return: ");
        let input: String = user_input::input();
        println!();
        if input_break_check(&input) == 0 {
            break;
        }

        match input.as_str() {
            "m" => {characters.push(add_monster());}
            "p" => {characters.push(add_player());}
            _ => {
                println!("Invalid command!");
            }
        }
    }
    save_encounter_file(&mut characters);
}
