pub mod parser;

use std::{
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use ansi_term::{Style, Colour};

use regex::Regex;

// Use some reexports?
use crate::util::{
    self, Zone,
    settings::Settings,
    uberstate::UberState,
    constants::{HEADER_INDENT, NAME_COLOUR, UBERSTATE_COLOUR},
};
use crate::world::graph::Graph;

fn is_hidden(header: &Path) -> Result<bool, String> {
    let file = fs::File::open(header).map_err(|err| format!("Failed to open header from {:?}: {}", header, err))?;
    let mut file = BufReader::new(file);

    let mut line = String::new();
    file.read_line(&mut line).map_err(|err| format!("Failed to read header from {:?}: {}", header, err))?;

    Ok(line.trim() == "#hide")
}

fn headers_in_directory(directory: &Path) -> Result<Vec<PathBuf>, String> {
    Ok(fs::read_dir(directory).map_err(|err| format!("Failed to read directory {:?}: {}", directory, err))?
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if let Some("wotwrh") = extension.to_str() {
                        return Some(path);
                    }
                }
            }
            None
        })
        .collect())
}

fn find_headers(show_hidden: bool) -> Result<Vec<PathBuf>, String> {
    let mut headers = headers_in_directory(Path::new("."))?;
    if let Ok(mut more) = headers_in_directory(Path::new("./headers")) {
        headers.append(&mut more);
    }

    if !show_hidden {
        headers = headers.iter()
            .map(|header| is_hidden(header).map(|hidden| if hidden { None } else { Some(header) }))
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .filter_map(|&header| header)
            .cloned()
            .collect::<Vec<_>>();
    }

    Ok(headers)
}

fn summarize_headers(headers: &[PathBuf]) -> Result<String, String> {
    let mut output = String::new();

    for header in headers {
        let mut name = header.file_stem().unwrap().to_string_lossy().into_owned();
        let header = fs::read_to_string(header).map_err(|err| format!("Error reading header from {:?}: {}", header, err))?;

        let mut description = "no description";

        for line in header.lines() {
            if let Some(desc) = line.trim_start().strip_prefix("///") {
                let desc = desc.trim();
                if desc.is_empty() {
                    continue;
                }
                if description == "no description" {
                    description = desc;
                } else {
                    description = desc;
                    break;
                }
            }
        }

        util::add_trailing_spaces(&mut name, HEADER_INDENT);

        output += &format!("{}  {}\n", NAME_COLOUR.paint(name), description);
    }

    Ok(output)
}

pub fn list() -> Result<(), String> {
    let mut output = String::new();

    let headers = find_headers(false)?;

    if headers.is_empty() {
        println!("No headers found");
        return Ok(());
    }

    let headers_length = headers.len();
    output += &format!("{}", Style::new().fg(Colour::Green).bold().paint(format!("{} header{} found\n\n", headers_length, if headers_length == 1 { "" } else { "s" })));

    output += &summarize_headers(&headers)?;
    output.push('\n');

    output += "Use 'headers <name>...' for details about one or more headers";

    println!("{}", output);
    Ok(())
}

pub fn inspect(headers: Vec<PathBuf>) -> Result<(), String> {
    let mut output = String::new();

    let hint = if headers.len() == 1 {
        let name = headers[0].file_stem().unwrap().to_string_lossy();
        format!("Use 'preset <name> -h {} ...' to add this and other headers to a preset", NAME_COLOUR.paint(name))
    } else {
        let mut arguments = headers.iter().fold(String::new(), |acc, header|
            format!("{}{} ", acc, header.file_stem().unwrap().to_string_lossy())
        );
        arguments.pop();

        format!("Use 'preset <name> -h {} ...' to add these headers to a preset", NAME_COLOUR.paint(arguments))
    };

    for mut header in headers {
        header.set_extension("wotwrh");
        let name = header.file_stem().unwrap().to_string_lossy();

        let contents = util::read_file(&header, "headers")?;

        let mut description = NAME_COLOUR.paint(format!("{} header:\n", name)).to_string();

        for line in contents.lines() {
            if let Some(desc) = line.trim_start().strip_prefix("///") {
                description.push_str(desc.trim());
                description.push('\n');
            }
        }

        if description.is_empty() {
            output += &Style::new().italic().paint("No description provided\n");
        } else {
            output += &description;
            output.push('\n');
        }
    }

    output += &hint;
    println!("{}", output);
    Ok(())
}

pub fn validate(path: Option<PathBuf>) -> Result<(), String> {
    let mut output = String::new();

    let headers = match path {
        Some(path) => vec![path],
        None => find_headers(true)?,
    };

    let mut occupation_map = Vec::new();

    let length = headers.len();
    output += &format!("{}", Style::new().italic().paint(format!("validating {} header{}\n", length, if length == 1 { "" } else { "s" })));

    let mut passed = Vec::new();
    let mut failed = Vec::new();

    for header in headers {
        let contents = util::read_file(&header, "headers")?;
        let mut name = header.file_stem().unwrap().to_string_lossy().into_owned();

        match parser::validate_header(&header, &contents) {
            Ok((occupied, excludes)) => {
                occupation_map.push((name, occupied, excludes));
            },
            Err(err) => {
                util::add_trailing_spaces(&mut name, HEADER_INDENT);
                failed.push(format!("{}  {}\n", NAME_COLOUR.paint(name), err))
            },
        }
    }

    for index in 0..occupation_map.len() {
        let (header, occupied, excludes) = &occupation_map[index];
        let mut collision_message = String::new();

        'outer: for uber_state in occupied {
            for (other_header, other_occupied, _) in &occupation_map {
                if header == other_header || excludes.contains_key(other_header) {
                    continue;
                }
                if let Some(collision) = other_occupied.iter().find(|&other| {
                    let generic = uber_state.value.is_empty() || other.value.is_empty();
                    uber_state == other || (generic && uber_state.identifier == other.identifier)
                }) {
                    collision_message = format!("Collision between used state {} and {} using {}",
                        UBERSTATE_COLOUR.paint(format!("{}", uber_state)),
                        NAME_COLOUR.paint(other_header),
                        UBERSTATE_COLOUR.paint(format!("{}", collision))
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

                occupied_summary += &format!("{}, ", UBERSTATE_COLOUR.paint(format!("{}", uber_state)));
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
    if failed_length > 0 {
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

    println!("{}", output);
    Ok(())
}

fn where_is(pattern: &str, world_index: usize, seeds: &[String], graph: &Graph, settings: &Settings) -> Result<String, String> {
    let re = Regex::new(&format!(r"^({})$", pattern)).map_err(|err| format!("Invalid regex {}: {}", pattern, err))?;

    for mut line in seeds[world_index].lines() {
        if let Some(index) = line.find("//") {
            line = &line[..index];
        }
        line = line.trim();

        if line.is_empty() || line.starts_with("Flags") || line.starts_with("Spawn") {
            continue;
        }

        let mut parts = line.splitn(3, '|');
        let uber_group = parts.next().unwrap();
        let uber_id = parts.next().ok_or_else(|| format!("failed to read line {} in seed", line))?;
        let pickup = parts.next().ok_or_else(|| format!("failed to read line {} in seed", line))?;

        if re.is_match(pickup) {
            if uber_group == "12" {  // if multiworld shared
                let actual_pickup = format!(r"8\|12\|{}\|bool\|true", uber_id);

                let mut other_worlds = (0..seeds.len()).collect::<Vec<_>>();
                other_worlds.remove(world_index);

                for other_world_index in other_worlds {
                    let actual_zone = where_is(&actual_pickup, other_world_index, seeds, graph, settings)?;
                    if &actual_zone != "Unknown" {
                        let player_name = settings.players.get(other_world_index).map(|name| name.clone()).unwrap_or_else(|| format!("Player {}", other_world_index + 1));

                        return Ok(format!("{}'s {}", player_name, actual_zone));
                    }
                }
            } else if uber_group == "3" && (uber_id == "0" || uber_id == "1") {
                return Ok(String::from("Spawn"));
            } else {
                let uber_state = UberState::from_parts(uber_group, uber_id)?;
                if let Some(node) = graph.nodes.iter().find(|&node| node.uber_state() == Some(&uber_state)) {
                    if let Some(zone) = node.zone() {
                        return Ok(zone.to_string());
                    }
                }
            }
        }
    }

    Ok(String::from("Unknown"))
}

fn how_many(pattern: &str, zone: Zone, world_index: usize, seeds: &[String], graph: &Graph) -> Result<Vec<UberState>, String> {
    let mut locations = Vec::new();
    let re = Regex::new(&format!(r"^({})$", pattern)).map_err(|err| format!("Invalid regex {}: {}", pattern, err))?;

    for mut line in seeds[world_index].lines() {
        if let Some(index) = line.find("//") {
            line = &line[..index];
        }
        line = line.trim();

        if line.is_empty() || line.starts_with("Flags") || line.starts_with("Spawn") {
            continue;
        }

        let mut parts = line.splitn(3, '|');
        let uber_group = parts.next().unwrap();
        let uber_id = parts.next().ok_or_else(|| format!("failed to read line {} in seed", line))?;
        let pickup = parts.next().ok_or_else(|| format!("failed to read line {} in seed", line))?;

        let uber_state = UberState::from_parts(uber_group, uber_id)?;
        if graph.nodes.iter().any(|node| node.zone() == Some(zone) && node.uber_state() == Some(&uber_state)) {
            if re.is_match(pickup) {
                locations.push(uber_state);
            } else {  // if multiworld shared
                let mut pickup_parts = pickup.split('|');
                if pickup_parts.next() != Some("8") { continue; }
                if pickup_parts.next() != Some("12") { continue; }
                let share_id = pickup_parts.next().unwrap();
                let share_state = format!("12|{}|", share_id);

                let mut other_worlds = (0..seeds.len()).collect::<Vec<_>>();
                other_worlds.remove(world_index);

                'outer: for other_world_index in other_worlds {
                    let other_seed = &seeds[other_world_index];

                    for other_seed_line in other_seed.lines() {
                        if let Some(mut actual_pickup) = other_seed_line.strip_prefix(&share_state) {
                            if let Some(index) = actual_pickup.find("//") {
                                actual_pickup = &actual_pickup[..index];
                            }
                            actual_pickup = actual_pickup.trim();

                            if re.is_match(actual_pickup) {
                                locations.push(uber_state);
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(locations)
}

fn read_args(seed: &str, start_index: usize) -> Option<usize> {
    let mut depth: u8 = 1;
    for (index, char) in seed[start_index..].chars().enumerate() {
        if char == '(' { depth += 1; }
        else if char == ')' { depth -= 1; }
        if depth == 0 {
            return Some(start_index + index);
        }
    }

    return None;
}

pub fn postprocess(seeds: &mut Vec<String>, graph: &Graph, settings: &Settings) -> Result<(), String> {
    let clone = seeds.clone();

    for (world_index, seed) in seeds.into_iter().enumerate() {
        let mut last_index = 0;
        loop {
            if let Some(mut start_index) = seed[last_index..].find("$WHEREIS(") {
                start_index += last_index;
                last_index = start_index;

                let after_bracket = start_index + 9;

                if let Some(end_index) = read_args(seed, after_bracket) {
                    let pattern = seed[after_bracket..end_index].trim();

                    let zone = where_is(pattern, world_index, &clone, graph, settings)?;
                    seed.replace_range(start_index..=end_index, &zone);

                    continue;
                }
            }
            break;
        }

        last_index = 0;
        loop {
            if let Some(mut start_index) = seed[last_index..].find("$HOWMANY(") {
                start_index += last_index;
                last_index = start_index;

                let after_bracket = start_index + 9;

                if let Some(end_index) = read_args(seed, after_bracket) {
                    let mut args = seed[after_bracket..end_index].splitn(2, ',');
                    let zone = args.next().unwrap().trim();
                    let zone: u8 = zone.parse().map_err(|_| format!("expected numeric zone, got {}", zone))?;
                    let zone = Zone::from_id(zone).ok_or_else(|| format!("invalid zone {}", zone))?;
                    let pattern = args.next().unwrap_or("").trim();

                    let locations = how_many(pattern, zone, world_index, &clone, graph)?;
                    let locations = locations.into_iter().map(|uber_state| uber_state.to_string()).collect::<Vec<_>>();
                    let locations = locations.join(",").replace('|', ",");

                    let sysmessage = format!("$[15|4|{}]", locations);

                    seed.replace_range(start_index..=end_index, &sysmessage);

                    continue;
                }
            }
            break;
        }
    }

    Ok(())
}
