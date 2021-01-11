use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::iter::once_with;
use std::rc::Rc;
use std::str::FromStr;

use Object::*;

const NUM_FLOORS: usize = 4;
type Floor = usize;
type Element = u8;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Object {
    Microchip(Element),
    Generator(Element),
}

impl Object {
    fn safe_with(&self, other: &Self) -> bool {
        match (self, other) {
            (Microchip(elem1), Generator(elem2)) if elem1 != elem2 => false,
            (Generator(elem1), Microchip(elem2)) if elem1 != elem2 => false,
            _ => true,
        }
    }

    fn element(&self) -> Element {
        match self {
            Microchip(elem) => *elem,
            Generator(elem) => *elem,
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Building {
    elevator: usize,
    floors: Vec<Vec<Object>>,
}

impl Building {
    fn canonical(mut self) -> Self {
        let mut next_element = 0;
        let mut mapping: HashMap<Element, Element> = HashMap::new();

        for floor in self.floors.iter_mut() {
            for object in floor.iter_mut() {
                let current_element = object.element();
                let replaced_elem = match mapping.get(&current_element) {
                    Some(elem) => *elem,
                    None => {
                        mapping.insert(current_element, next_element);
                        let replaced = next_element;
                        next_element += 1;
                        replaced
                    }
                };
                *object = match object {
                    Microchip(_) => Microchip(replaced_elem),
                    Generator(_) => Generator(replaced_elem),
                };
            }
            floor.sort_unstable();
        }
        self
    }

    fn all_at_top_floor(&self) -> bool {
        self.floors
            .iter()
            .take(self.floors.len() - 1)
            .all(|objects| objects.is_empty())
    }

    fn safe_floor(&self, floor: Floor) -> bool {
        let objects = &self.floors[floor];
        if objects.iter().all(|object| matches!(object, Microchip(_))) {
            true
        } else {
            objects.iter().all(|object| match object {
                Microchip(element) => objects.contains(&Generator(*element)),
                _ => true,
            })
        }
    }

    fn move_object(&mut self, object: Object, from: Floor, to: Floor) {
        let from_floor = &mut self.floors[from];
        from_floor
            .remove(from_floor.iter().position(|&obj| obj == object).unwrap());

        // It's important to keep the objects in order to avoid generating
        // different canonical states
        let to_floor = &mut self.floors[to];
        if let Some(index) = to_floor.iter().position(|&obj| obj > object) {
            to_floor.insert(index, object);
        } else {
            to_floor.push(object);
        }
    }

    fn move_to(
        &self,
        floor: Floor,
        object1: Object,
        opt_object2: Option<Object>,
    ) -> Option<Self> {
        let mut building = self.clone();
        if let Some(object2) = opt_object2 {
            if !object1.safe_with(&object2) {
                return None;
            }
            building.move_object(object2, self.elevator, floor);
        }
        building.move_object(object1, self.elevator, floor);
        if building.safe_floor(self.elevator) && building.safe_floor(floor) {
            building.elevator = floor;
            Some(building)
        } else {
            None
        }
    }

    fn possible_moves(&self) -> impl Iterator<Item = Self> + '_ {
        let mut destination_floor = Vec::new();
        if self.elevator > 0 {
            destination_floor.push(self.elevator - 1);
        }
        if self.elevator + 1 < self.floors.len() {
            destination_floor.push(self.elevator + 1);
        }

        let objects = &self.floors[self.elevator];

        destination_floor.into_iter().flat_map(move |floor| {
            objects
                .iter()
                .enumerate()
                .flat_map(move |(index, &object1)| {
                    objects[index + 1..]
                        .iter()
                        .map(move |&object2| {
                            self.move_to(floor, object1, Some(object2))
                        })
                        .chain(once_with(move || {
                            self.move_to(floor, object1, None)
                        }))
                        .filter_map(|building| building)
                })
        })
    }

    /// Calculate a minimum number of moves required to take all objects to
    /// the top floor by ignoring the restrictions of mixing incompatible
    /// microchips and generators
    fn min_remaining_moves(&self) -> usize {
        // count number of objects on each floor, ignoring the top floor
        let mut num_objects: Vec<_> = self
            .floors
            .iter()
            .take(self.floors.len() - 1)
            .map(|objects| objects.len())
            .collect();

        let first_moves = if self.elevator == self.floors.len() - 1 {
            // elevator is at the top; there's no special move for first object
            0
        } else {
            // elevator is below top floor; one of the objects on this floor
            // will be carried on all moves, until all objects are at the top
            num_objects[self.elevator] -= 1;

            // find first floor that is not empty; one of the objects on this
            // floor will be carried on the first trip to the top floor
            let first_non_empty =
                num_objects.iter().position(|&num| num > 0).unwrap();
            num_objects[first_non_empty] -= 1;

            if self.elevator > first_non_empty {
                // first go down to first non-empty floor, then up to top floor
                self.floors.len() - 1 + self.elevator - 2 * first_non_empty
            } else {
                // elevator is at the first non-empty floor, go straight to top
                self.floors.len() - 1 - self.elevator
            }
        };

        first_moves
            // all other objects require a round-trip from top floor
            + num_objects
                .iter()
                .zip((1..=num_objects.len()).rev())
                .map(|(&num_obj, dist)| 2 * dist * num_obj)
                .sum::<usize>()
    }

    fn add_new_element_pair(&self, floor: Floor) -> Option<Self> {
        let mut building = self.clone();
        let new_element = self
            .floors
            .iter()
            .filter_map(|objects| {
                objects.iter().map(|object| object.element()).max()
            })
            .max()
            .map(|max| max + 1)
            .unwrap_or(0);
        let objects = building.floors.get_mut(floor)?;
        objects.push(Microchip(new_element));
        objects.push(Generator(new_element));
        if building.safe_floor(floor) {
            Some(building)
        } else {
            None
        }
    }
}

#[derive(Eq)]
struct State {
    num_moves: usize,
    min_remaining_moves: usize,
    min_total_moves: usize,
    building: Rc<Building>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.min_total_moves
            .cmp(&other.min_total_moves)
            .then(self.min_remaining_moves.cmp(&other.min_remaining_moves))
            .reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.min_total_moves == other.min_total_moves
            && self.min_remaining_moves == other.min_remaining_moves
    }
}

impl State {
    fn new(num_moves: usize, building: Rc<Building>) -> Self {
        let min_remaining_moves = building.min_remaining_moves();
        let min_total_moves = num_moves + min_remaining_moves;
        Self {
            num_moves,
            min_remaining_moves,
            min_total_moves,
            building,
        }
    }
}

/// An A-star search that estimates, for each node reached, a minimum distance
/// to the solution by counting how many moves would be required to move all
/// objects to the top floor ignoring the microchip-generator safety rules.
fn find_min_moves(start: &Building) -> Option<usize> {
    let mut states: BinaryHeap<State> = BinaryHeap::new();
    let mut seen = HashSet::new();

    let start_ptr = Rc::new(start.clone().canonical());
    seen.insert(Rc::clone(&start_ptr));
    states.push(State::new(0, start_ptr));

    while let Some(state) = states.pop() {
        let num_moves = state.num_moves + 1;
        for next_building in state.building.possible_moves() {
            let building_ptr = Rc::new(next_building.canonical());
            if !seen.insert(Rc::clone(&building_ptr)) {
                continue;
            }
            if building_ptr.all_at_top_floor() {
                return Some(num_moves);
            }
            states.push(State::new(num_moves, building_ptr));
        }
    }
    None
}

pub fn part1(start: &Building) -> Option<usize> {
    find_min_moves(start)
}

pub fn part2(start: &Building) -> Option<usize> {
    find_min_moves(&start.add_new_element_pair(0)?.add_new_element_pair(0)?)
}

impl FromStr for Building {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let floor_regex = Regex::new(concat!(
            r"^The (?P<floor>[[:alpha:]]+) floor contains ",
            r"((?P<objects>.*)|(nothing relevant))\.$",
        ))
        .unwrap();
        let object_regex = Regex::new(concat!(
            r"((?P<chip_elem>\w+)-compatible microchip)",
            r"|((?P<gen_elem>\w+) generator)",
        ))
        .unwrap();

        let elevator = 0;
        let mut floors = vec![vec![]; NUM_FLOORS];
        let mut element_map = HashMap::new();
        let mut next_element_id = 0;
        for line in s.lines() {
            let floor_cap = floor_regex
                .captures(line)
                .ok_or_else(|| format!("Invalid input: {}", line))?;
            let floor_index = match floor_cap.name("floor").unwrap().as_str() {
                "first" => Ok(0),
                "second" => Ok(1),
                "third" => Ok(2),
                "fourth" => Ok(3),
                f => Err(format!("Invalid floor number: {}", f)),
            }?;
            let mut floor_objects = Vec::new();
            if let Some(objects) = floor_cap.name("objects") {
                for object_cap in object_regex.captures_iter(objects.as_str()) {
                    if let Some(element) = object_cap.name("chip_elem") {
                        let element_id = *element_map
                            .entry(element.as_str())
                            .or_insert_with(|| {
                                let id = next_element_id;
                                next_element_id += 1;
                                id
                            });
                        floor_objects.push(Microchip(element_id));
                    } else if let Some(element) = object_cap.name("gen_elem") {
                        let element_id = *element_map
                            .entry(element.as_str())
                            .or_insert_with(|| {
                                let id = next_element_id;
                                next_element_id += 1;
                                id
                            });
                        floor_objects.push(Generator(element_id));
                    }
                }
            }
            floors[floor_index] = floor_objects;
        }

        Ok(Self { elevator, floors })
    }
}

use std::fmt::{Display, Formatter};
impl Display for Building {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (index, objects) in self.floors.iter().enumerate().rev() {
            if index == self.elevator {
                writeln!(f, "({}): {:?}", index + 1, objects)?;
            } else {
                writeln!(f, " {} : {:?}", index + 1, objects)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input =
            "The first floor contains a hydrogen-compatible microchip and a \
            lithium-compatible microchip.\n\
            The second floor contains a hydrogen generator.\n\
            The third floor contains a lithium generator.\n\
            The fourth floor contains nothing relevant.";
        let building = input.parse().unwrap();
        assert_eq!(find_min_moves(&building), Some(11));
    }
}
