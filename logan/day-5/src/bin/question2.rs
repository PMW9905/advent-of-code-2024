use std::env;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::string::String;

// helper types and enums
type PageNumber = u16;
type PrintInstruction = Vec<PageNumber>;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum RuleDirectionEnum {
    Preceding,
    Following,
}

type RuleRelation = (PageNumber, RuleDirectionEnum);
type RuleSet = HashMap<PageNumber, Vec<RuleRelation>>;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Error: please supply a path to file.");
        return;
    }
    let file_path: String = String::from(args[1].clone());
    let Ok(input_file) = File::open(file_path) else {
        println!("Error opening file");
        return;
    };
    println!("File opened successfully");

    let (mut rule_set, remaining_lines) = parse_rules(input_file);

    let (sum_of_correct, sum_of_repaired) = evaluate_instructions(&mut rule_set, remaining_lines);

    println!("Evaluated instructions | correct: {}, repaired: {}", sum_of_correct, sum_of_repaired);
}

fn parse_rules(file: std::fs::File) -> (RuleSet, impl Iterator<Item = String>) {
    let reader: BufReader<File> = BufReader::new(file);
    let mut lines = reader.lines().flatten();

    let mut rule_set: RuleSet = HashMap::new();

    while let Some(line) = lines.next() {
        let is_end_of_rules_section = line.is_empty();

        if is_end_of_rules_section {
            break;
        }

        parse_rule(&mut rule_set, line);
    }

    return (rule_set, lines);
}

fn parse_rule(rule_set: &mut RuleSet, line: String) {
    let vals: Vec<&str> = line.split('|').collect();

    let (a, b) = (vals[0].parse::<PageNumber>(), vals[1].parse::<PageNumber>());

    let mut add_relation = |key: PageNumber, relation_to_push: (PageNumber, RuleDirectionEnum)| -> () {
        let rule_to_modify = rule_set.get_mut(&key);
        match rule_to_modify {
            Some(rule_relations) => {
                rule_relations.push(relation_to_push);
            }
            None => {
                let relations = Vec::from([relation_to_push]);
                rule_set.insert(key, relations);
            }
        }
    };

    if a.is_ok() && b.is_ok() {
        let first_value: PageNumber = a.unwrap();
        let second_value: PageNumber = b.unwrap();

        let first_to_second_relation = (second_value, RuleDirectionEnum::Following);
        let second_to_first_relation = (first_value, RuleDirectionEnum::Preceding);

        let _ = add_relation(first_value, first_to_second_relation);
        let _ = add_relation(second_value, second_to_first_relation);
    }
}

/**
 * Returns a tuple containing: (sum of middles for all initially correct instrucions, sum of middle pages of all repaired instructions)
 */
fn evaluate_instructions(
    rule_set: &mut RuleSet,
    lines: impl Iterator<Item = String>,
) -> (u16, u16) {
    let (valid_instructions, invalid_instructions) = categorize_instructions(rule_set, lines);
    // repair invalid instructions
    let repaired_instructions= repair_invalid_instructions(rule_set, invalid_instructions);
    
    // parse middle values of both valid and invalid sets
    let valid_instruction_centers: Vec<PageNumber> = valid_instructions.into_iter().map(|instruction| {instruction[instruction.len() / 2]}).collect();
    let repaired_instruction_centers: Vec<PageNumber> = repaired_instructions.into_iter().map(|instruction| {instruction[instruction.len() / 2]}).collect();

    // sum middle values
    let sum_of_correct = valid_instruction_centers.iter().sum::<PageNumber>();
    let sum_of_repaired = repaired_instruction_centers.iter().sum::<PageNumber>();
    
    (sum_of_correct, sum_of_repaired)
}

fn categorize_instructions (
    rule_set: &mut RuleSet,
    mut lines: impl Iterator<Item = String>,
) -> (Vec<PrintInstruction>, Vec<PrintInstruction>) {
    let mut valid_instructions: Vec<PrintInstruction> = Vec::new();
    let mut invalid_instructions: Vec<PrintInstruction> = Vec::new();

    while let Some(line) = lines.next() {
        let parsed_line_into_nums = line
            .split(',')
            .into_iter()
            .filter_map(|str| -> Option<PageNumber> { str.parse::<PageNumber>().ok() })
            .collect::<Vec<PageNumber>>();

        let mut is_line_valid = true;

        let valid_page_index_map = determine_instruction_ordering(rule_set, parsed_line_into_nums.clone());
        
        for (page_number, valid_index) in valid_page_index_map {
            if !(parsed_line_into_nums[valid_index] == page_number) {
                is_line_valid = false;
            };
        }

        if is_line_valid {
            valid_instructions.push(parsed_line_into_nums);
        } else {
            invalid_instructions.push(parsed_line_into_nums);
        }
    }

    (valid_instructions, invalid_instructions)
}

fn repair_invalid_instructions (rule_set: &mut RuleSet, invalid_instruction: Vec<PrintInstruction>) -> Vec<PrintInstruction> {
    let mut fixed_instructions: Vec<PrintInstruction> = Vec::new();

    for instruction in invalid_instruction {
        let valid_page_index_map = determine_instruction_ordering(rule_set, instruction.clone());

        let mut fixed_array: Vec<u16> = instruction.clone();

        for (page_number, valid_index) in valid_page_index_map {
            fixed_array[valid_index] = page_number;
        }

        fixed_instructions.push(fixed_array);
    }

    return fixed_instructions;
}

/**
 * Returns: HashMap of type <key: PageNumber, value: valid_index_position>
 */
fn determine_instruction_ordering (rule_set: &mut RuleSet, instruction: PrintInstruction) -> HashMap<PageNumber, usize> {
    let mut instruction_pages: HashSet<PageNumber> = HashSet::new();

    // initialize evaluated page rules
    for page_number in instruction.clone().iter() {
        instruction_pages.insert(page_number.clone());
    }

    let mut valid_page_index_map: HashMap<PageNumber, usize> = HashMap::new();

    for page_number in instruction.clone().iter() {
        let Some(page_relations) = rule_set.get(page_number) else {
            println!("Error, no relationships found for {}!", page_number);
            continue;
        };

        let mut preceding_instructions = 0;

        for (related_page, rule_direction) in page_relations.iter() {
            let does_instruction_contain_page = instruction_pages.contains(&related_page);

            if does_instruction_contain_page {
                match rule_direction {
                    RuleDirectionEnum::Following => {}
                    RuleDirectionEnum::Preceding => {
                        preceding_instructions += 1;
                    }
                }
            }

        }
        valid_page_index_map.insert(page_number.clone(), preceding_instructions);
    }

    valid_page_index_map
}