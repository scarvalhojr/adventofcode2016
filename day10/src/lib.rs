use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use Destination::*;
use Instruction::*;

pub type Value = u32;
pub type ID = usize;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Destination {
    Bot(ID),
    OutputBin(ID),
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Instruction {
    BotInput(Value, ID),
    BotOutput(ID, Destination, Destination),
}

struct Bot {
    low: Option<Value>,
    high: Option<Value>,
}

impl Bot {
    fn new(value: Value) -> Self {
        Self {
            low: Some(value),
            high: None,
        }
    }

    fn receive(&mut self, new_value: Value) {
        match (self.low, self.high) {
            (None, None) => {
                self.low = Some(new_value);
            }
            (Some(curr_value), None) => {
                if curr_value <= new_value {
                    self.high = Some(new_value);
                } else {
                    self.high = Some(curr_value);
                    self.low = Some(new_value);
                }
            }
            _ => {
                panic!("Bot overflow");
            }
        }
    }

    fn give(&mut self) -> Option<(Value, Value)> {
        match (self.low, self.high) {
            (Some(low), Some(high)) => {
                self.low = None;
                self.high = None;
                Some((low, high))
            }
            _ => None,
        }
    }

    fn has_markers(&self, marker_val1: Value, marker_val2: Value) -> bool {
        match (self.low, self.high) {
            (Some(low), Some(high)) => {
                (marker_val1 == low && marker_val2 == high)
                    || (marker_val2 == low && marker_val1 == high)
            }
            _ => false,
        }
    }
}

pub fn execute(
    instructions: &[Instruction],
    marker_val1: Value,
    marker_val2: Value,
) -> Option<(ID, Value)> {
    // Only the first three output bins matter
    let mut bins = vec![None; 3];
    let mut bots = HashMap::new();
    let mut marker_bot = None;
    let mut executed = HashSet::new();

    while executed.len() < instructions.len() {
        for (instr_num, instr) in instructions
            .iter()
            .enumerate()
            .filter(|(instr_num, _)| !executed.contains(instr_num))
        {
            match &instr {
                BotInput(value, bot_id) => {
                    bots.entry(*bot_id)
                        .and_modify(|bot| bot.receive(*value))
                        .or_insert(Bot::new(*value));
                }
                BotOutput(bot_id, low_dest, high_dest) => {
                    if let Some((low_val, high_val)) = bots.get_mut(bot_id).and_then(|bot| bot.give()) {
                        match low_dest {
                            Bot(receiver_id) => {
                                bots.entry(*receiver_id)
                                    .and_modify(|bot| bot.receive(low_val))
                                    .or_insert(Bot::new(low_val));
                            }
                            OutputBin(bin) => {
                                bins.get_mut(*bin)
                                    .map(|v| *v = Some(low_val));
                            }
                        };
                        match high_dest {
                            Bot(receiver_id) => {
                                bots.entry(*receiver_id)
                                    .and_modify(|bot| bot.receive(high_val))
                                    .or_insert(Bot::new(high_val));
                            }
                            OutputBin(bin) => {
                                bins.get_mut(*bin)
                                    .map(|v| *v = Some(high_val));
                            }
                        };
                    } else {
                        continue;
                    }
                }
            };
            if let Some((&bot_id, _)) = bots.iter().find(|(_, bot)| {
                bot.has_markers(marker_val1, marker_val2)
            }) {
                marker_bot = Some(bot_id);
            }
            executed.insert(instr_num);
            break;
        }
    }
    marker_bot.and_then(|bot_id| {
        bins.into_iter()
            .collect::<Option<Vec<_>>>()
            .map(|values| (bot_id, values.iter().product()))
    })
}

impl FromStr for Destination {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures =
            Regex::new(r"^(bot (?P<bot>\d+))|(output (?P<output>\d+))$")
                .unwrap()
                .captures(s)
                .ok_or_else(|| "Invalid destination")?;

        if let Some(bot_str) = captures.name("bot") {
            bot_str
                .as_str()
                .parse()
                .map(|id| Bot(id))
                .map_err(|e| e.to_string())
        } else {
            captures
                .name("output")
                .unwrap()
                .as_str()
                .parse()
                .map(|id| OutputBin(id))
                .map_err(|e| e.to_string())
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = Regex::new(concat!(
            r"^(value (?P<value>\d+) goes to bot (?P<receiver>\d+))",
            r"|(bot (?P<bot>\d+) gives ",
            r"low to (?P<low>(bot|output) \d+) and ",
            r"high to (?P<high>(bot|output) \d+))$",
        ))
        .unwrap()
        .captures(s)
        .ok_or_else(|| "Invalid instruction")?;

        if let Some(value_str) = captures.name("value") {
            let value = value_str
                .as_str()
                .parse::<Value>()
                .map_err(|e| e.to_string())?;
            let bot = captures
                .name("receiver")
                .unwrap()
                .as_str()
                .parse::<ID>()
                .map_err(|e| e.to_string())?;
            Ok(BotInput(value, bot))
        } else {
            let bot = captures
                .name("bot")
                .unwrap()
                .as_str()
                .parse::<ID>()
                .map_err(|e| e.to_string())?;
            let low_dest = captures.name("low").unwrap().as_str().parse()?;
            let high_dest = captures.name("high").unwrap().as_str().parse()?;
            Ok(BotOutput(bot, low_dest, high_dest))
        }
    }
}
