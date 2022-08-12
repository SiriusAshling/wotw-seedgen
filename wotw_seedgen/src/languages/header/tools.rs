use ansi_term::{Style, Colour};
use rustc_hash::FxHashMap;

use crate::{util::{
    self,
    constants::{HEADER_INDENT, NAME_COLOUR, UBERSTATE_COLOUR}, UberState,
}, Header, Item, item::{UberStateOperator, Command}};

use super::{HeaderContent, VResolve};

pub type Identifier = String;
/// Perform a set of checks on the given [`Header`]s, including parsing them and checking for collisions in the used uberStates.
/// 
/// Will also print information about what uberStates are still free to use
/// 
/// Returns `true` if all checks passed
pub fn validate_headers(headers: Vec<(Identifier, String)>) -> bool {
    let mut output = String::new();

    let mut occupation_map = Vec::new();

    let length = headers.len();
    output += &format!("{}", Style::new().italic().paint(format!("validating {} header{}\n", length, if length == 1 { "" } else { "s" })));

    let mut passed = Vec::new();
    let mut failed = Vec::new();

    for (identifier, header) in headers {
        let mut identifier = identifier.to_string();
        match validate_header(header) {
            Ok((occupied, excludes)) => {
                occupation_map.push((identifier, occupied, excludes));
            },
            Err(err) => {
                util::add_trailing_spaces(&mut identifier, HEADER_INDENT);
                failed.push(format!("{}  {}\n", NAME_COLOUR.paint(identifier), err));
            },
        }
    }

    for index in 0..occupation_map.len() {
        let (header, occupied, excludes) = &occupation_map[index];
        let mut collision_message = String::new();

        'outer: for uber_state in occupied {
            // special cases because this system is not holding up to modern header logic
            if uber_state.identifier.uber_group == 9 && (
                uber_state.identifier.uber_id == 0 && ["250", "251", "999"].contains(&&uber_state.value[..])
                || uber_state.identifier.uber_id == 999 && uber_state.value == "200"
                || uber_state.identifier.uber_id == 150
            ) {
                continue;
            }

            for (other_header, other_occupied, _) in &occupation_map {
                if header == other_header || excludes.contains(other_header) {
                    continue;
                }
                if let Some(collision) = other_occupied.iter().find(|&other| {
                    let generic = uber_state.value.is_empty() || other.value.is_empty();
                    uber_state == other || (generic && uber_state.identifier == other.identifier)
                }) {
                    collision_message = format!("Collision between used state {} and {} using {}",
                        UBERSTATE_COLOUR.paint(uber_state.code().to_string()),
                        NAME_COLOUR.paint(other_header),
                        UBERSTATE_COLOUR.paint(collision.code().to_string())
                    );
                    break 'outer;
                }
            }
        }

        if collision_message.is_empty() {
            let mut occupied_summary = String::new();
            let mut last_value = i32::MIN;
            let mut range = false;

            for uber_state in occupied {
                if let Ok(value) = uber_state.value.parse::<i32>() {
                    if value == last_value + 1 {
                        range = true;
                    } else if range {
                        for _ in 0..2 { occupied_summary.pop(); }
                        occupied_summary += &format!("{}, ", UBERSTATE_COLOUR.paint(format!("..{}", last_value)));
                        range = false;
                    }
                    last_value = value;
                    if range {
                        continue;
                    }
                } else if range {
                    for _ in 0..2 { occupied_summary.pop(); }
                    occupied_summary += &format!("{}, ", UBERSTATE_COLOUR.paint(format!("..{}", last_value)));
                    range = false;
                }

                occupied_summary += &format!("{}, ", UBERSTATE_COLOUR.paint(uber_state.code().to_string()));
            }

            for _ in 0..2 { occupied_summary.pop(); }

            if range {
                occupied_summary += &format!("{}", UBERSTATE_COLOUR.paint(format!("..{}", last_value)));
            }

            let mut name = header.clone();
            util::add_trailing_spaces(&mut name, HEADER_INDENT);

            if occupied_summary.is_empty() {
                passed.push(format!("{}  --\n", NAME_COLOUR.paint(name)));
            } else {
                passed.push(format!("{}  uses {}\n", NAME_COLOUR.paint(name), occupied_summary));
            }
        } else {
            let mut name = header.clone();
            util::add_trailing_spaces(&mut name, HEADER_INDENT);
            failed.push(format!("{}  {}\n", NAME_COLOUR.paint(name), collision_message));
        }
    }

    let failed_length = failed.len();
    let valid = failed_length == 0;
    if !valid {
        output += &format!("{}", Colour::Red.paint(format!("\n{}/{} failed\n", failed_length, length)));

        for failed in failed {
            output += &failed;
        }
    }
    let passed_length = passed.len();
    if passed_length > 0 {
        output += &format!("{}", Colour::Green.paint(format!("\n{}/{} passed\n", passed_length, length)));

        for passed in passed {
            output += &passed;
        }
    }

    let mut check_free = |description, range, condition: fn(&UberState, i32) -> bool |
    {
        let mut first = None;
        for index in range {
            let occupied = occupation_map.iter().any(|(_, states, _)| states.iter().any(|state| condition(state, index)));
            if occupied {
                if let Some(first_value) = first {
                    let last = index - 1;
                    output += &format!("Free {description}: {first_value}..{last}\n");
                    first = None;
                }
            } else if first.is_none() {
                first = Some(index);
            }
        }
        if let Some(first_value) = first {
            output += &format!("Free {description}: {first_value}..\n");
        }
    };

    check_free("9|0", 1..1000, |state: &UberState, index: i32|
        state.identifier.uber_group == 9
        && state.identifier.uber_id == 0
        && state.value == index.to_string()
    );
    check_free("integer", 1..100, |state: &UberState, index: i32|
        state.identifier.uber_group == 9
        && i32::from(state.identifier.uber_id) == index
    );
    check_free("boolean", 100..150, |state: &UberState, index: i32|
        state.identifier.uber_group == 9
        && i32::from(state.identifier.uber_id) == index
    );
    check_free("float", 150..175, |state: &UberState, index: i32|
        state.identifier.uber_group == 9
        && i32::from(state.identifier.uber_id) == index
    );

    println!("{}", output);
    valid
}

pub fn validate_header(contents: String) -> Result<(Vec<UberState>, Vec<String>), String> {
    let mut default_parameters = FxHashMap::default();

    let header = Header::parse(contents, &mut rand::thread_rng())
        .map_err(|errors| errors.verbose_display())?;
    header.fill_parameters(&mut default_parameters)?;
    let build = header.clone().build(default_parameters.clone())?;

    let mut occupied_states = vec![];

    for content in header.contents {
        match content {
            HeaderContent::Timer(timer) => {
                occupied_states.push(UberState {
                    identifier: timer.counter,
                    value: "++".to_string(),  // represent a timer so that the sort will put it alongside + and - commands
                });
            },
            HeaderContent::Pickup(pickup) => {
                if pickup.skip_validation { continue }

                let pickup = pickup.resolve(&default_parameters)?;
                if pickup.trigger.identifier.uber_group == 9 {
                    occupied_states.push(pickup.trigger.clone());
                }

                match pickup.item {
                    Item::UberState(uber_state_item) if uber_state_item.uber_identifier.uber_group == 9 => {
                        if let UberStateOperator::Value(mut value) = uber_state_item.operator {
                            if value == "false" || value == "0" { continue }
                            if value == "true" { value = String::from("1"); }

                            occupied_states.push(UberState {
                                identifier: uber_state_item.uber_identifier,
                                value,
                            });
                        }
                    },
                    Item::Command(Command::StopEqual { uber_state } |
                    Command::StopGreater { uber_state } |
                    Command::StopLess { uber_state }) => {
                        if pickup.trigger.identifier.uber_group == 9 {
                            if uber_state.identifier.uber_group == 9 {
                                occupied_states.push(uber_state);
                            }
                        } else {
                            return Err(format!("stop command on {} stops a multipickup outside of uber group 9. This may interact unpredictably with other headers.", pickup.trigger.code()));
                        }
                    }
                    _ => {},
                }
            },
            _ => {},
        }
    }

    occupied_states.sort_unstable();
    occupied_states.dedup();

    // remove redundancies, the sort beforehand put all timers, + and - commands in front
    let mut index = 0;
    while let Some(current) = occupied_states.get_mut(index) {
        if current.value.starts_with(&['+', '-'][..]) || current.value.is_empty() {
            current.value = String::new();
            let clone = current.clone();

            occupied_states.retain(|other| other == &clone || other.identifier != clone.identifier);
        }
        index += 1;
    }

    occupied_states.dedup();

    Ok((occupied_states, build.excludes))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::files;

    #[test]
    fn validate() {
        let headers = files::find_headers().unwrap().into_iter()
            .map(|path| {
                let content = fs::read_to_string(&path).unwrap();
                let identifier = path.file_stem().unwrap().to_string_lossy().to_string();
                (identifier, content)
            }).collect();
        assert!(super::validate_headers(headers), "validation failed");
    }
}
