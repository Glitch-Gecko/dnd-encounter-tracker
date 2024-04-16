use crate::user_input;
use serde::{Serialize, Deserialize};
use std::fs;
use crate::stat_search;
use titlecase::titlecase;
use colored::*;
use rand::Rng;

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
/// Grabs stats from [stat_search] and rolls for a hit and damage against a character's AC
///
pub fn attack() {
    // Loads encounter file if it exists
    let content = fs::read_to_string("./files/encounter.json").unwrap_or_else(|_| "[]".to_string());
    let characters: Vec<Character> = serde_json::from_str(&content).unwrap_or_else(|_| {
        println!("Error parsing JSON. Starting with an empty list.");
        Vec::new()
    });

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

        println!("Enter the number of the attacking creature (can't be a player), or type \"done\" to return:");
        let input: String = user_input::input();
        println!();
        if input == "done" {
            break;
        }
        let attacker = input.parse::<usize>().unwrap();
        
        // Restarts function if a player is entered
        if &characters[attacker-1].character_type == "Player" {
            attack();
        }

        println!("Enter the number of the attacked creature, or type \"done\" to return:");
        let input: String = user_input::input();
        println!();
        if input == "done" {
            break;
        }
        let attacked = input.parse::<usize>().unwrap();

        // Displays attacks based on the character's type in the encounter file
        stat_search::print_attacks(&characters[attacker-1].character_type);
        println!("Enter the number of the attack, or type \"done\" to return:");


        let input: String = user_input::input();
        let attack_number = input.parse::<usize>().unwrap();
        println!();
        if input == "done" {
            break;
        }

        // Actually loads the attack, resetting if it's invalid
        let attack_var = stat_search::get_attack(&characters[attacker-1].character_type, attack_number);
        if attack_var.damage_type == "Null" {
            attack();
        }

        // Rolls for attack and damage, comparing it to target's AC
        let attack_roll = rand::thread_rng().gen_range(1..21) + attack_var.attack_modifier;
        if attack_roll >= characters[attacked-1].ac {
            attack_string_1 = format!("Rolled a {} against {}/{}'s AC of {}, and hit", attack_roll, characters[attacked-1].character_type, characters[attacked-1].name, characters[attacked-1].ac);
            let mut damage_roll = rand::thread_rng().gen_range(1..attack_var.damage_dice[1]+1);
            for _ in 0..attack_var.damage_dice[0]-1 {
                damage_roll += rand::thread_rng().gen_range(1..attack_var.damage_dice[1]+1);
            }
            attack_string_2 = format!("This dealt {}+{} = {} {} damage", damage_roll, attack_var.damage_bonus, damage_roll+attack_var.damage_bonus, attack_var.damage_type);
        } else {
            attack_string_1 = format!("Rolled a {} against {}/{}'s AC of {}, and missed", attack_roll, characters[attacked-1].character_type, characters[attacked-1].name, characters[attacked-1].ac);
            attack_string_2 = "Null".to_string();
        }
    }
}

///
/// Loads encounter file and allows for edits, then saves any modifications to the file
///
pub fn edit_creature() {
    // Loads encounter file
    let content = fs::read_to_string("./files/encounter.json").unwrap_or_else(|_| "[]".to_string());
    let mut characters: Vec<Character> = serde_json::from_str(&content).unwrap_or_else(|_| {
        println!("Error parsing JSON. Starting with an empty list.");
        Vec::new()
    });


    loop {
        print_creatures(&characters);
        println!("Enter the number of a creature to edit, or type \"done\" to return: ");
        let input: String = user_input::input();
        println!();

        if input == "done" {
            println!("Finished adding characters\n");
            break;
        }

        let number = input.parse::<usize>().unwrap();

        println!("Editing {}/{}\n", characters[number-1].character_type, characters[number-1].name);
        println!("Fields:");
        println!("| 1. Name: {} | 2. Creature race: {} | 3. AC: {} | 4. HP: {} | 5. Initiative: {} |", characters[number-1].name, characters[number-1].character_type, characters[number-1].ac, characters[number-1].hp, characters[number-1].initiative);
        println!("\nEnter the number of the field to edit:");
        let input: String = user_input::input();
        println!();
        
        match input.as_str() {
            "1" => {
                println!("Enter new name:");
                characters[number-1].name = titlecase(&user_input::input());
            },
            "2" => {
                println!("Enter new race:");
                characters[number-1].character_type = titlecase(&user_input::input());
            },
            "3" => {
                println!("Enter new AC:");
                characters[number-1].ac = user_input::input().parse::<i32>().unwrap();
            },
            "4" => {
                println!("Enter new HP:");
                characters[number-1].hp = user_input::input().parse::<i32>().unwrap();
            },
            "5" => {
                println!("Enter new initiative:");
                characters[number-1].initiative = user_input::input().parse::<i32>().unwrap();
            }
            _ => {
                println!("Invalid input!");
                edit_creature();
            }
        }
        
        // Saves new file contents
        let json_characters = serde_json::to_string_pretty(&characters).unwrap();
        std::fs::write("files/encounter.json", json_characters).expect("Unable to write to file");
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
        println!("Current encounter:");
        for creature in characters {
            if creature.character_type == "Player" {
                let string = format!("{}. PC/{}", number, creature.name);
                print!("| {} ", string.blue());
            } else {
                let string = format!("{}. {}/{}", number, creature.character_type, creature.name);
                print!("| {} ", string.red());
            }
            number +=1;
        }
        println!("|\n");
    }
}

///
/// Function used to remove a creature from the encounter list
///
pub fn remove_creature() {
    // Loads encounter file
    let content = fs::read_to_string("./files/encounter.json").unwrap_or_else(|_| "[]".to_string());
    let mut characters: Vec<Character> = serde_json::from_str(&content).unwrap_or_else(|_| {
        println!("Error parsing JSON. Starting with an empty list.");
        Vec::new()
    });
    
    loop {
        print_creatures(&characters);
        println!("Enter the number of a creature to remove, or type \"done\" to return: ");
        let input: String = user_input::input();
        println!();

        if input == "done" {
            println!("Finished adding characters\n");
            break;
        }

        let number = input.parse::<usize>().unwrap();

        characters.remove(number-1);

        // Saves changes to encounter file
        let json_characters = serde_json::to_string_pretty(&characters).unwrap();
        std::fs::write("files/encounter.json", json_characters).expect("Unable to write to file");
    }
}

/// 
/// Function used to apply damage to a creature based on the creature's number (refer to [print_creatures])
///
pub fn damage_creature() {
    // Loads encounter file
    let content = fs::read_to_string("./files/encounter.json").unwrap_or_else(|_| "[]".to_string());
    let mut characters: Vec<Character> = serde_json::from_str(&content).unwrap_or_else(|_| {
        println!("Error parsing JSON. Starting with an empty list.");
        Vec::new()
    });
    
    loop {
        print_creatures(&characters);
        println!("Enter the number of a creature to damage, or type \"done\" to return: ");
        let input: String = user_input::input();
        println!();
        if input == "done" {
            println!("Finished adding characters\n");
            break;
        }
        let number = input.parse::<usize>().unwrap();

        println!("Damaging {}/{}", characters[number-1].character_type, characters[number-1].name);
        println!("Enter damage dealt (negatives are used for healing):");
        let damage = user_input::input().parse::<i32>().unwrap();

        characters[number-1].hp -= damage;

        // Saves changes in encounter file
        let json_characters = serde_json::to_string_pretty(&characters).unwrap();
        std::fs::write("files/encounter.json", json_characters).expect("Unable to write to file");
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
    let ac = user_input::input().parse::<i32>().unwrap();

    // Note that hp is not used, but is necessary for the Character struct
    let hp = 999999;
    println!("\nEnter {}'s rolled initiative:", name);
    let initiative = user_input::input().parse::<i32>().unwrap();
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
    // Loads encounter file
    let content = fs::read_to_string("./files/encounter.json").unwrap_or_else(|_| "[]".to_string());
    let mut characters: Vec<Character> = serde_json::from_str(&content).unwrap_or_else(|_| {
        println!("Error parsing JSON. Starting with an empty list.");
        Vec::new()
    });

    loop {
        print_creatures(&characters);
        println!("Add either a (m)onster or (p)layer, or type \"done\" to return: ");
        let input: String = user_input::input();
        println!();

        if input == "done" {
            println!("Finished adding characters\n");
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

    // Sorts characters by initiative (doesn't take dex into account)
    characters.sort_by_key(|char| -char.initiative);

    // Saves changes to encounter file
    let json_characters = serde_json::to_string_pretty(&characters).unwrap();
    std::fs::write("files/encounter.json", json_characters).expect("Unable to write to file");
}
