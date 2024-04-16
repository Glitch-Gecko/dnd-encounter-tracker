use std::fs;
use serde_json::Result;
use serde::{Deserialize, Serialize};
use crate::user_input;
use std::io;
use std::io::Write;
use crate::encounter;
use crate::encounter::Character;
use rand::Rng;
use titlecase::titlecase;

///
/// Creature struct used for storing all character stats
///
#[derive(Serialize, Deserialize, Debug)]
struct Creature {
    name: String,
    health: i32,
    armor_class: i32,
    initiative: i32,
    movement_speed: i32,
    str: i32,
    dex: i32,
    con: i32,
    int: i32,
    wis: i32,
    cha: i32,
    actions: Vec<Action>,
    abilities: Vec<Ability>,
}

///
/// Action struct used for storing attack action information
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Action {
    name: String,
    description: String,
    pub attack_modifier: i32,
    pub damage_dice: Vec<i32>,
    pub damage_bonus: i32,
    pub damage_type: String,
}

///
/// Ability struct used for storing other ability information
///
#[derive(Serialize, Deserialize, Debug)]
struct Ability {
    name: String,
    description: String,
}

///
/// Loads monster stats to be used in [encounter]
///
pub fn load_monster(monster_type: String) -> encounter::Character {
    // Reads statblock file
    let contents = fs::read_to_string("./files/statblocks.json").expect("Couldn't read statblock file");
    let creatures = parse_json(&contents).unwrap();

    // Checks if the creature exists
    if let Some(creature) = creatures.iter().find(|c| c.name.to_lowercase() == monster_type.to_lowercase()) {
        println!("\nEnter a name for the {}: ", creature.name);
        let name = titlecase(&user_input::input());
        let character_type = &creature.name;
        let ac = creature.armor_class;
        let hp = creature.health;

        // Uses RNG to roll for initiative
        let mut initiative = rand::thread_rng().gen_range(1..21) + creature.initiative;

        // Ensures initiative doesn't drop below 1
        if initiative <= 0 {initiative = 1}
        println!("\nAdded {}/{}, with rolled initiative {}\n", character_type, name, initiative);
        Character {
            name,
            character_type: character_type.to_string(),
            ac,
            hp,
            initiative,
        }
    } else {
        // Restarts function if the monster doesn't exist in the statblock file
        println!("\nInvalid monster! Use one of the below monsters:");
        print_monsters();
        println!("\nEnter monster type (type ls for a list of monsters):");
        load_monster(user_input::input())
    }
}

///
/// Prints available monsters in the statblock file
///
pub fn print_monsters() {
    let contents = fs::read_to_string("./files/statblocks.json").expect("Couldn't read statblock file");
    let creatures = parse_json(&contents).unwrap();
    println!("==========");
    println!("Available creatures:");
    print!("| ");
    for creature in &creatures {
        print!("{} | ", creature.name);
        io::stdout().flush().unwrap();
    }
    println!("\n==========");
}

///
/// Used in the main menu to display selected monster's actions and abilities
///
pub fn combat_stats(creature_stat: &str) {
    let contents = fs::read_to_string("./files/statblocks.json").expect("Couldn't read statblock file");
    let creatures = parse_json(&contents).unwrap();

    if let Some(creature) = creatures.iter().find(|c| c.name.to_lowercase() == creature_stat.to_lowercase()) {
    println!("\n==========");
    println!("Actions:");
    for action in &creature.actions {
        println!("----------");
        println!(" {}:", action.name);
        println!("Description: \"{}\"", action.description);
        println!("Attack roll modifier: +{}", action.attack_modifier);
        println!("Damage dice: {}d{}+{}", action.damage_dice[0], action.damage_dice[1], action.damage_bonus);
        println!("Damage type: {}", action.damage_type);
    }
    println!("==========");
    println!("Abilities:");
    for ability in &creature.abilities {
        println!("----------");
        println!(" {}:", ability.name);
        println!("Description: \"{}\"", ability.description);
    }
    println!("==========");
    } else {
        println!("\nCreature not found.\n");
    }
}

///
/// Used in the main menu to display selected monster's stats
///
pub fn print_attributes(creature_stat: &str) {
    let contents = fs::read_to_string("./files/statblocks.json").expect("Couldn't read statblock file");
    let creatures = parse_json(&contents).unwrap();

    if let Some(creature) = creatures.iter().find(|c| c.name.to_lowercase() == creature_stat.to_lowercase()) {
        print!("\n| STR: {} | DEX: {} | CON: {} | INT: {} | WIS: {} | CHA: {} |\n", creature.str, creature.dex, creature.con, creature.int, creature.wis, creature.cha);
    } 
}

///
/// Used in [encounter::attack] to display selected monster's attacks
///
pub fn print_attacks(creature_stat: &str) {
    let contents = fs::read_to_string("./files/statblocks.json").expect("Couldn't read statblock file");
    let creatures = parse_json(&contents).unwrap();

    if let Some(creature) = creatures.iter().find(|c| c.name.to_lowercase() == creature_stat.to_lowercase()) {
        let mut number = 1;
        print!("==========");
        for action in &creature.actions {
            println!("\n{}. {}", number, action.name);
            println!("Attack modifier: {}", action.attack_modifier);
            println!("Damage: {}d{}+{} {} damage", action.damage_dice[0], action.damage_dice[1], action.damage_bonus, action.damage_type);
            if number != creature.actions.len(){print!("----------");}

            number += 1;
        }
        println!("==========\n")
    }
}

///
/// Used to send the selected attack to [encounter::attack], returning null values if the attack or monster doesn't exist
///
pub fn get_attack(creature_stat: &str, attack_number: usize) -> Action {
    let contents = fs::read_to_string("./files/statblocks.json").expect("Couldn't read statblock file");
    let creatures = parse_json(&contents).unwrap();

    if let Some(creature) = creatures.iter().find(|c| c.name.to_lowercase() == creature_stat.to_lowercase()) {
        if attack_number > creature.actions.len() {
            Action {
                name: "Null".to_string(),
                description: "Null".to_string(),
                attack_modifier: 0,
                damage_dice: [0, 0].to_vec(),
                damage_bonus: 0,
                damage_type: "Null".to_string(),
            }
        } else {
            Action {
                name: creature.actions[attack_number-1].name.clone(),
                description: creature.actions[attack_number-1].description.clone(),
                attack_modifier: creature.actions[attack_number-1].attack_modifier,
                damage_dice: creature.actions[attack_number-1].damage_dice.clone(),
                damage_bonus: creature.actions[attack_number-1].damage_bonus,
                damage_type: creature.actions[attack_number-1].damage_type.clone(),
            }
        }
    } else {
        Action {
            name: "Null".to_string(),
            description: "Null".to_string(),
            attack_modifier: 0,
            damage_dice: [0, 0].to_vec(),
            damage_bonus: 0,
            damage_type: "Null".to_string(),
        }
    }
}

///
/// Function used to display the monster's statblock
///
pub fn statblocks() {
    let contents = fs::read_to_string("./files/statblocks.json").expect("Couldn't read statblock file");
    let creatures = parse_json(&contents).unwrap();

    print_monsters();

    println!("\nEnter a creature name to get its stats:");
    let name = user_input::input();

    if let Some(creature) = creatures.iter().find(|c| c.name.to_lowercase() == name.to_lowercase()) {
        println!("\n==========");
        println!("Stats for {}:", creature.name);
        println!("Health: {}", creature.health);
        println!("Armor Class: {}", creature.armor_class);
        println!("Initiative: {}", creature.initiative);
        println!("Movement Speed: {}", creature.movement_speed);
        print!("| STR: {} | DEX: {} | CON: {} | INT: {} | WIS: {} | CHA: {} |", creature.str, creature.dex, creature.con, creature.int, creature.wis, creature.cha);
        combat_stats(&name);
        println!();
    } else {
        println!("\nCreature not found.\n");
    }
}

///
/// Parses json data from a given input
///
fn parse_json(json_data: &str) -> Result<Vec<Creature>> {
    let creatures: Vec<Creature> = serde_json::from_str(json_data)?;
    Ok(creatures)
}
