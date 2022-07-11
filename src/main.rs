use std::{vec::Vec, collections::HashMap, borrow::Borrow, io::stdin};
use regex::Regex;

#[derive(Debug, Clone)]
struct Animal {
    name: String,
    characteristics: Vec<String>
}

#[derive(Debug, Clone)]
struct DesicionTreeNode {
    answer: Option<String>,
    characteristic: Option<String>,
    yes_branch: Option<Box<DesicionTreeNode>>,
    no_branch: Option<Box<DesicionTreeNode>>
}

// Creats a map of characteristic -> Variance
fn compute_variance(animals: &Vec<Animal>) -> HashMap<String, f64> {
    let mut counting_map: HashMap<String, f64> = HashMap::new();
    let observations = animals.len() as f64;

    let mut variance_map: HashMap<String, f64> = HashMap::new();

    // Counts the number of times each characteristic appears
    for animal in animals.iter() {
        for characteristic in animal.characteristics.iter() {
            let count = counting_map
                .entry(characteristic.to_string())
                .or_insert(0.0);
            *count += 1.0;
        }
    }

    // Compute variance of binomial distribution for each characteristic
    for (animal, count) in counting_map.iter() {
        let prob = *count / observations;
        let variance = prob * (1.0 - prob) * observations;
        variance_map.insert(animal.to_string(), variance);
    }

    return variance_map;
}

// Creates a sorted vector of all characteristics using the variance map
// Highest variance first
fn sorted_characteristics(variance_map: &HashMap<String, f64>) -> Vec<String> {
    let mut characteristics: Vec<String> = Vec::new();

    for characteristic in variance_map.keys() {
        characteristics.push(characteristic.to_string());
    }

    characteristics.sort_by_key(|i| {
        *variance_map.get(i).unwrap() as i64
    });

    characteristics.reverse();

    return characteristics;
}

// Returns all known observations. It is a sparse matrix
fn get_animals() -> Vec<Animal> {
    return vec![
        Animal {
            name: "Gato".to_string(),
            characteristics: Vec::from(["Es una mascota".to_string(), "Ronronea".to_string()])
        },
        Animal {
            name: "Perro".to_string(),
            characteristics: vec!["Es una mascota".to_string(), "Ladra".to_string()]
        },
        Animal {
            name: "León".to_string(),
            characteristics: vec!["Es un animal salvaje".to_string(), "Ruge".to_string(), "Tiene melenena".to_string()]
        },
        Animal {
            name: "Tigre".to_string(),
            characteristics: vec!["Es un animal salvaje".to_string(), "Ruge".to_string(), "Tiene manchas".to_string()]
        }
    ];
}

fn build_decision_tree(animals: &Vec<Animal>) -> DesicionTreeNode {
    if animals.len() == 1 {
        return DesicionTreeNode {
            answer: Some(animals[0].name.to_string()),
            characteristic: None,
            no_branch: None,
            yes_branch: None,
        }
    }

    if animals.len() == 0 {
        return DesicionTreeNode {
            answer: None,
            characteristic: None,
            no_branch: None,
            yes_branch: None,
        }  
    }

    let variance_map = compute_variance(animals);
    let sorted_characteristics = sorted_characteristics(variance_map.borrow());
    let node_characteristic = sorted_characteristics[0].to_string();
    
    let mut with_characteristic = animals.to_vec();
    with_characteristic.retain(|animal| animal.characteristics.contains(&node_characteristic));

    let mut without_characteristic = animals.to_vec();
    without_characteristic.retain(|animal| !animal.characteristics.contains(&node_characteristic));

    return DesicionTreeNode {
        answer: None,
        characteristic: Some(node_characteristic),
        yes_branch: Some(Box::new(build_decision_tree(with_characteristic.borrow()))),
        no_branch: Some(Box::new(build_decision_tree(without_characteristic.borrow())))
    }
}

fn read_bool_command_line() -> bool {
    let mut input_string = String::new();
    let yes_re = Regex::new(r"si|SI|Si|Sí|S|s|y|yes|YES").unwrap();
    let no_re = Regex::new(r"no|NO|No|n").unwrap();

    loop {
        stdin().read_line(&mut input_string)
            .ok()
            .expect("No se pudo leer correctamente.");

        if yes_re.is_match(input_string.borrow()) {
            return true
        }

        if no_re.is_match(input_string.borrow()) {
            return false
        }

        println!("No entendí correctamente. Intenta escribiendo si o no.");
    }
}

fn traverse_decision_tree(tree: DesicionTreeNode) -> Option<String> {
    let mut head = tree;
    loop {
        let answer = head.answer;
        let characteristic = head.characteristic;
        let yes_branch = head.yes_branch;
        let no_branch = head.no_branch;
        
        if answer.is_some() {
            return Some(answer.unwrap());
        }

        if characteristic.is_none() {
            return None
        }

        println!("¿El animal... {}? [Sí/No]", characteristic.unwrap());
        let has_characteristic = read_bool_command_line();

        // Walks yes branch
        match has_characteristic {
            true => head = *yes_branch.unwrap(),
            false => head = *no_branch.unwrap(),
        }
    }
}

fn main() {
    let animals = get_animals();
    let mut decision_tree: DesicionTreeNode;
    let mut guess: Option<String>;

    loop {
        decision_tree = build_decision_tree(animals.borrow());
        guess = traverse_decision_tree(decision_tree);
        
        if guess.is_none()  {
            println!("No conozco ese animal...");
        } else {
            println!("Tu animal es... {}", guess.unwrap());
        }


        println!("¿Quieres seguir jugando? [Sí/No]");
        let should_continue = read_bool_command_line();

        match should_continue {
            false => break,
            _ => {}
        }
    }
}

