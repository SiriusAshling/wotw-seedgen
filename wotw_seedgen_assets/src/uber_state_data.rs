#[cfg(feature = "loc_data")]
use crate::LocDataEntry;
#[cfg(feature = "state_data")]
use crate::StateDataEntry;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    io,
};
use wotw_seedgen_data::UberIdentifier;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UberStateData {
    pub name_lookup: FxHashMap<String, FxHashMap<String, Vec<UberStateAlias>>>,
    pub id_lookup: FxHashMap<UberIdentifier, UberStateDataEntry>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UberStateAlias {
    pub uber_identifier: UberIdentifier,
    pub value: Option<u8>,
}
impl Display for UberStateAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.uber_identifier)?;
        if let Some(value) = self.value {
            write!(f, " >= {}", value)?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UberStateDataEntry {
    pub name: String,
    pub rando_name: Option<String>,
    pub default_value: UberStateValue,
    pub readonly: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UberStateValue {
    Boolean(bool),
    Integer(i32),
    Float(f32),
}
impl UberStateData {
    pub fn from_reader<R: io::Read>(reader: R) -> serde_json::Result<Self> {
        let mut uber_state_data = Self::default();
        let dump: Dump = serde_json::from_reader(reader)?;
        for (group, dump_group) in dump.groups {
            let group_map = uber_state_data
                .name_lookup
                .entry(dump_group.name.clone())
                .or_default();

            for (member, dump_member) in dump_group.states {
                let name = format!("{}.{}", dump_group.name, dump_member.name);

                group_map
                    .entry(dump_member.name)
                    .or_default()
                    .push(UberStateAlias {
                        uber_identifier: UberIdentifier::new(group, member),
                        value: None,
                    });

                let default_value = match dump_member.value_type {
                    ValueType::Boolean => UberStateValue::Boolean(dump_member.value != 0.),
                    ValueType::Byte | ValueType::Integer => {
                        UberStateValue::Integer(dump_member.value as i32)
                    }
                    ValueType::Float => UberStateValue::Float(dump_member.value),
                    ValueType::Unknown => continue,
                };

                uber_state_data.id_lookup.insert(
                    UberIdentifier::new(group, member),
                    UberStateDataEntry {
                        name,
                        rando_name: None,
                        default_value,
                        readonly: dump_member.readonly,
                    },
                );
            }
        }
        Ok(uber_state_data)
    }
    #[cfg(feature = "loc_data")]
    pub fn add_loc_data(&mut self, loc_data: Vec<LocDataEntry>) {
        for record in loc_data {
            self.add_rando_name(record.identifier, record.uber_identifier, record.value);
        }
    }
    #[cfg(feature = "state_data")]
    pub fn add_state_data(&mut self, state_data: Vec<StateDataEntry>) {
        for record in state_data {
            self.add_rando_name(record.identifier, record.uber_identifier, record.value);
        }
    }
    #[cfg(any(feature = "loc_data", feature = "state_data"))]
    fn add_rando_name(&mut self, name: String, uber_identifier: UberIdentifier, value: Option<u8>) {
        let (group, member) = name.split_once('.').expect("Invalid UberState name");
        self.name_lookup
            .entry(group.to_string())
            .or_default()
            .entry(member.to_string())
            .or_default()
            .push(UberStateAlias {
                uber_identifier,
                value,
            });
        self.id_lookup.get_mut(&uber_identifier).unwrap().rando_name = Some(name);
    }
}

#[derive(Deserialize)]
struct Dump {
    groups: FxHashMap<i32, DumpGroup>,
}
#[derive(Deserialize)]
struct DumpGroup {
    name: String,
    states: FxHashMap<i32, DumpMember>,
}
#[derive(Deserialize)]
struct DumpMember {
    name: String,
    readonly: bool,
    value: f32,
    value_type: ValueType,
}
#[derive(Deserialize)]
enum ValueType {
    Boolean,
    Byte,
    Integer,
    Float,
    Unknown,
}
