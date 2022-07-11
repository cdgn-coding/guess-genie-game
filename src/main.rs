use std::{vec::Vec, collections::HashMap, borrow::Borrow, io::{stdin, stdout, Write}};
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

    characteristics.sort_by(|a, b| {
        variance_map.get(a).unwrap().partial_cmp(variance_map.get(b).unwrap()).unwrap()
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
        },
        Animal {
            name: "Oso".to_string(),
            characteristics: vec!["Es un animal salvaje".to_string(), "Ruge".to_string(), "Es muy peludo".to_string()]
        },
        Animal {
            name: "Oveja".to_string(),
            characteristics: vec!["Tiene lana".to_string()]
        },
        Animal {
            name: "Tortuga".to_string(),
            characteristics: vec!["Tiene caparazon".to_string(), "Pone huevos".to_string(), "Es lento".to_string()]
        },
        Animal {
            name: "Caballo".to_string(),
            characteristics: vec!["Es muy veloz".to_string(), "Relincha".to_string(), "Es muy fuerte".to_string()]
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
        print!("Tú: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input_string)
            .ok()
            .expect("No se pudo leer correctamente.");

        if yes_re.is_match(input_string.borrow()) {
            return true
        }

        if no_re.is_match(input_string.borrow()) {
            return false
        }

        println!("Genio: No entendí correctamente. Intenta escribiendo si o no.");
    }
}

fn traverse_decision_tree(tree: DesicionTreeNode) -> (Option<String>, Vec<String>) {
    let mut head = tree;
    let mut characteristics: Vec<String> = Vec::new();

    loop {
        let answer = head.answer;
        let characteristic = head.characteristic;
        let yes_branch = head.yes_branch;
        let no_branch = head.no_branch;
        let characteristic_string: String;
        
        if answer.is_some() {
            return (Some(answer.unwrap()), characteristics);
        }

        if characteristic.is_none() {
            return (None, characteristics);
        }

        characteristic_string = characteristic.unwrap();

        println!("Genio: ¿El animal... {}? [Sí/No]", characteristic_string);
        let has_characteristic = read_bool_command_line();

        // Walks yes branch
        match has_characteristic {
            true => {
                head = *yes_branch.unwrap();
                characteristics.push(characteristic_string);
            },
            false => head = *no_branch.unwrap(),
        }
    }
}

fn meet_new_animal() -> String {
    println!("Genio: ¿Qué animal es?");
    let mut input_string = String::new();

    print!("Tú: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input_string)
        .ok()
        .expect("No se pudo leer correctamente.");

    return input_string.trim().to_string();
}

fn meet_new_characteristic(guessed_animal: &String, true_animal: &String) -> String {
    println!("Genio: ¿Qué característica tiene {}, que lo diferencie de {}?", true_animal, guessed_animal);
    let mut input_string = String::new();

    print!("Tú: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input_string)
        .ok()
        .expect("No se pudo leer correctamente.");

    return input_string.trim().to_string();
}

fn meet_animal_with_given_characteristics(animals: &mut Vec<Animal>, characteristics: Vec<String>) {
    let new_animal_name = meet_new_animal();

    let new_animal = Animal {
        name: new_animal_name,
        characteristics: characteristics,
    };

    animals.push(new_animal);
}

fn meet_animal_with_new_characteristic(animals: &mut Vec<Animal>, characteristics: &mut Vec<String>, guessed_animal: &String) {
    let new_animal_name = meet_new_animal();

    let new_animal_characteristic = meet_new_characteristic(
        guessed_animal.borrow(),
        new_animal_name.borrow()
    );
    
    characteristics.push(new_animal_characteristic);

    let new_animal = Animal {
        name: new_animal_name,
        characteristics: characteristics.to_vec(),
    };

    animals.push(new_animal);
}

fn main() {
    let mut animals = get_animals();
    let mut decision_tree: DesicionTreeNode;
    let mut guess: Option<String>;
    let mut characteristics: Vec<String>;

    println!("Genio: Soy un programa genio...");
    println!("Genio: Piensa en un animal y lo adivinaré.");

    loop {
        decision_tree = build_decision_tree(animals.borrow());
        (guess, characteristics) = traverse_decision_tree(decision_tree);
        
        if guess.is_none()  {
            println!("Genio: No conozco ese animal...");
            meet_animal_with_given_characteristics(&mut animals, characteristics);
            println!("Genio: Lo recordaré.");
        } else {
            let guessed_animal = guess.unwrap();
            println!("Genio: ¿Tu animal es... {}? [Sí/No]", guessed_animal);
            let is_correct = read_bool_command_line();

            if !is_correct {
                println!("Genio: No conozco ese animal...");
                meet_animal_with_new_characteristic(
                    &mut animals,
                    &mut characteristics,
                    &guessed_animal,
                );

                println!("Genio: Lo recordaré.");
            } else {
                println!("Genio: ¡Bien! Lo sabía.");
            }
        }

        println!("Genio: ¿Quieres seguir jugando? [Sí/No]");
        let should_continue = read_bool_command_line();

        match should_continue {
            false => break,
            _ => {}
        }
    }
}
