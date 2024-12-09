use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::string::String;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum RuleDirectionEnum {
    Preceding,
    Following,
}

type RuleRelation = (u16, RuleDirectionEnum);
type RuleSet = HashMap<u16, Vec<RuleRelation>>;

#[derive(Debug)]
struct PageLimits {
    // (index of rule, direction of rule)
    rule_queue: VecDeque<(usize, RuleDirectionEnum)>,
    page_order_index: usize,
    min_valid_index: Option<usize>,
    max_valid_index: Option<usize>,
}

type EvaluatedInstructionResults = HashMap<u16, PageLimits>;

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

    let (a, b) = (vals[0].parse::<u16>(), vals[1].parse::<u16>());

    let mut add_relation = |key: u16, relation_to_push: (u16, RuleDirectionEnum)| -> () {
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
        let first_value = a.unwrap();
        let second_value = b.unwrap();

        let first_to_second_relation = (second_value, RuleDirectionEnum::Following);
        let second_to_first_relation = (first_value, RuleDirectionEnum::Preceding);

        let _ = add_relation(first_value, first_to_second_relation);
        let _ = add_relation(second_value, second_to_first_relation);
    }
}

fn evaluate_instructions(
    rule_set: &mut RuleSet,
    mut lines: impl Iterator<Item = String>,
) -> Vec<u16> {
    let mut valid_instruction_centers: Vec<u16> = Vec::new();

    while let Some(line) = lines.next() {
        let mut evaluated_page_rules: EvaluatedInstructionResults = HashMap::new();
        let parsed_line = line
            .split(',')
            .into_iter()
            .filter_map(|str| -> Option<u16> { str.parse::<u16>().ok() })
            .collect::<Vec<u16>>();

        let num_pages_in_instruction = parsed_line.len();

        // initialize evaluated page rules
        for (i, page_number) in parsed_line.clone().iter().enumerate() {
            let new_result_entry = PageLimits {
                rule_queue: VecDeque::new(),
                page_order_index: i.clone(),
                max_valid_index: Some(num_pages_in_instruction),
                min_valid_index: Some(0),
            };

            evaluated_page_rules.insert(page_number.clone(), new_result_entry);
        }

        // decode parsed line here utilizing rule set
        for (i, page_number) in parsed_line.iter().enumerate() {
            let Some(page_relations) = rule_set.get(page_number) else {
                println!("Error, no relationships found for {}!", page_number);
                continue;
            };

            for (related_page, relation_direction) in page_relations.iter() {
                let result_to_modify = evaluated_page_rules.get_mut(&related_page);

                match result_to_modify {
                    Some(result) => {
                        let result_to_push = (i.clone(), relation_direction.clone());

                        match result_to_push.1 {
                            RuleDirectionEnum::Preceding => {
                                // if the index is impossible, set the value to NONE to indicate an impossible condition
                                if i < 1 {
                                    result.max_valid_index = None
                                } else {
                                    // a number must follow the LOWEST POSSIBLE MAXIMUM ACROSS ALL RULES
                                    let this_max_index = i - 1;

                                    match &result.max_valid_index {
                                        Some(current) => {
                                            if current.clone() > this_max_index {
                                                result.max_valid_index = Some(this_max_index);
                                            }
                                        }
                                        None => {
                                            result.max_valid_index = Some(this_max_index);
                                        }
                                    }
                                }
                            }
                            RuleDirectionEnum::Following => {
                                // if the index is impossible, set the value to NONE to indicate an impossible condition
                                if i == num_pages_in_instruction {
                                    result.min_valid_index = None
                                } else {
                                    // a number must follow the HIGHEST POSSIBLE MINIMUM ACROSS ALL RULES
                                    let this_min_index = i + 1;

                                    match &result.min_valid_index {
                                        Some(current) => {
                                            if current.clone() < this_min_index {
                                                result.min_valid_index = Some(this_min_index);
                                            }
                                        }
                                        None => {
                                            result.min_valid_index = Some(this_min_index);
                                        }
                                    }
                                }
                            }
                        }

                        result.rule_queue.push_back(result_to_push);
                    }
                    None => {}
                }
            }
        }

        let mut is_line_valid = true;
        for (page_number, PageLimits) in evaluated_page_rules.iter() {
            let has_valid_limits =
                PageLimits.max_valid_index.is_some() && PageLimits.min_valid_index.is_some();
            if has_valid_limits {
                let does_meet_min =
                    PageLimits.page_order_index >= PageLimits.min_valid_index.unwrap();
                let does_meet_max =
                    PageLimits.page_order_index <= PageLimits.max_valid_index.unwrap();
                if does_meet_min && does_meet_max {
                } else {
                    is_line_valid = false;
                }
            } else {
                is_line_valid = false;
            }
        }
        if is_line_valid {
            let middle_page_value = parsed_line[num_pages_in_instruction / 2];
            valid_instruction_centers.push(middle_page_value);
        }
    }

    return valid_instruction_centers;
}

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

    let valid_middle_pages = evaluate_instructions(&mut rule_set, remaining_lines);

    let sum = valid_middle_pages.iter().sum::<u16>();
    println!("Evaluated instructions: {:?}", sum);
}
