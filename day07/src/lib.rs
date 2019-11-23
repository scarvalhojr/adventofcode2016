use std::collections::HashSet;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

pub struct IP7 {
    components: Vec<IP7Component>,
}

struct IP7Component {
    sequence: String,
    is_hypernet: bool,
}

impl IP7Component {
    fn is_abba(&self) -> bool {
        self.sequence
            .chars()
            .collect::<Vec<_>>()
            .windows(4)
            .any(|window| {
                window[0] == window[3]
                    && window[1] == window[2]
                    && window[0] != window[1]
            })
    }

    fn get_all_aba(&self) -> Vec<(char, char)> {
        self.sequence
            .chars()
            .collect::<Vec<_>>()
            .windows(3)
            .filter(|window| window[0] == window[2] && window[0] != window[1])
            .map(|window| (window[0], window[1]))
            .collect()
    }
}

impl IP7 {
    fn supports_tls(&self) -> bool {
        self.components
            .iter()
            .any(|comp| comp.is_abba() && !comp.is_hypernet)
            && !self
                .components
                .iter()
                .any(|comp| comp.is_abba() && comp.is_hypernet)
    }

    fn supports_ssl(&self) -> bool {
        let hypernet_abas = self
            .components
            .iter()
            .filter(|comp| comp.is_hypernet)
            .flat_map(|comp| comp.get_all_aba())
            .collect::<HashSet<_>>();
        self.components
            .iter()
            .filter(|comp| !comp.is_hypernet)
            .any(|comp| {
                comp.get_all_aba()
                    .iter()
                    .any(|&(c1, c2)| hypernet_abas.contains(&(c2, c1)))
            })
    }
}

pub fn part1(ips: &[IP7]) -> usize {
    ips.iter().filter(|ip| ip.supports_tls()).count()
}

pub fn part2(ips: &[IP7]) -> usize {
    ips.iter().filter(|ip| ip.supports_ssl()).count()
}

impl FromStr for IP7 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = Vec::new();
        let mut seq = String::new();
        let mut is_hypernet = false;
        for ch in s.chars() {
            if (ch == '[' && !is_hypernet) || (ch == ']' && is_hypernet) {
                components.push(IP7Component {
                    sequence: seq.clone(),
                    is_hypernet,
                });
                seq.clear();
                is_hypernet = !is_hypernet;
            } else if ch != '[' && ch != ']' {
                seq.push(ch);
            } else {
                return Err(Error::new(ErrorKind::InvalidData, "Invalid IP7"));
            }
        }
        if !seq.is_empty() {
            components.push(IP7Component {
                sequence: seq,
                is_hypernet,
            });
        }
        Ok(IP7 { components })
    }
}
