use crate::open_file_first_arg;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::env::Args;
use crate::program3::CircuitNode::CentralPort;
extern crate derive_more;
use derive_more::{Display};
use std::fmt::{Display, Formatter, Error};
use std::convert::TryInto;

pub fn main(args: &mut Args, least_steps: bool) -> Result<u64, String> {
    open_file_first_arg(args).and_then(|reader| {
        fn read_line(
            input: Option<Result<String, io::Error>>, line: CircuitLine
        ) -> Result<Vec<CircuitInstruction>, String> {
            match input {
                None => Err(format!("Empty line {}", line)),
                Some(Err(err)) => Err(format!("Failed to read line {}: {}", line, err)),
                Some(Ok(line)) => CircuitInstruction::parse_str(line.as_str()),
            }
        }

        let mut circuit = Circuit::new();

        let mut lines = reader.lines();
        let line1_instructions = read_line(lines.next(), CircuitLine::_1)?;
        circuit.update(line1_instructions.iter(), CircuitLine::_1)?;

        let line2_instructions = read_line(lines.next(), CircuitLine::_2)?;
        let crossings = circuit.update(line2_instructions.iter(), CircuitLine::_2)?;

//        println!("{}", circuit.render());
        let zero = Coord(0, 0);
        crossings.iter()
            .map(|crossing|
                if least_steps { crossing.steps() }
                else { crossing.coord.manhattan_distance(&zero) }
            )
            .min()
            .ok_or(String::from("No crossings!"))
    })
}

#[derive(Debug)]
enum CircuitDirection { Left, Right, Up, Down }
impl CircuitDirection {
    fn from_char(c: char) -> Result<CircuitDirection, String> {
        match c {
            'L' | 'l' => Ok(CircuitDirection::Left),
            'R' | 'r' => Ok(CircuitDirection::Right),
            'U' | 'u' => Ok(CircuitDirection::Up),
            'D' | 'd' => Ok(CircuitDirection::Down),
            _ => Err(format!("Unknown direction '{}'", c))
        }
    }

    fn move_mut(&self, c: &mut Coord) {
        match self {
            CircuitDirection::Left => c.0 -= 1,
            CircuitDirection::Right => c.0 += 1,
            CircuitDirection::Up => c.1 += 1,
            CircuitDirection::Down => c.1 -= 1,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Coord(i64, i64);
impl Coord {
    fn manhattan_distance(&self, c2: &Coord) -> u64 {
        // |p1 - q1| + |p2 - q2|
        ((self.0 - c2.0).abs() + (self.1 - c2.1).abs()).try_into().unwrap_or(u64::max_value())
    }
}
impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_fmt(format_args!("({}, {})", self.0, self.1))
    }
}

#[derive(Debug)]
struct Bounds { min: Coord, max: Coord }

#[derive(Display, Eq, PartialEq, Copy, Clone, Debug)]
enum CircuitLine { _1, _2 }

#[derive(Debug, Copy, Clone)]
enum CircuitNode {
    CentralPort,
    Line { line: CircuitLine, steps: u32 },
    Crossing { steps_l1: u32, steps_l2: u32 }
}

#[derive(Debug)]
struct CircuitInstruction(CircuitDirection, u32);
impl CircuitInstruction {
    fn parse_str(s: &str) -> Result<Vec<CircuitInstruction>, String> {
        s.split(",").map(Circuit::parse_instruction).collect()
    }
}

struct CircuitCrossingAt { coord: Coord, steps_l1: u32, steps_l2: u32 }
impl CircuitCrossingAt {
    fn steps(&self) -> u64 { u64::from(self.steps_l1) + u64::from(self.steps_l2) }
}

#[derive(Debug)]
struct Circuit {
    nodes: HashMap<Coord, CircuitNode>
}
impl Circuit {
    fn new() -> Circuit {
        let mut c = Circuit { nodes: HashMap::new() };
        c.nodes.insert(Coord(0, 0), CentralPort);
        c
    }

    fn update<
        'a,
        I : Iterator<Item = &'a CircuitInstruction>
    >(&mut self, instructions: I, line: CircuitLine) -> Result<Vec<CircuitCrossingAt>, String> {
        let mut coord = Coord(0, 0);
        let mut steps = 0u32;
        let mut crossings = Vec::<CircuitCrossingAt>::new();
        for instruction in instructions {
//            println!("start: {:?}, instruction: {:?}", coord, instruction);
            for _ in 1..=instruction.1 {
                instruction.0.move_mut(&mut coord);
                steps += 1;
//                println!("current: {:?}", coord);
                let node = self.nodes.entry(coord).or_insert(CircuitNode::Line { line: line, steps: steps });
                match *node {
                    CentralPort => return Err(format!("{} crosses central port at {}", line, coord)),
                    CircuitNode::Crossing { .. } => {},
                    CircuitNode::Line { line: existing_line, steps: existing_line_steps } => {
                        if line != existing_line {
                            let current_line_1 = line == CircuitLine::_1;
                            let steps_l1 = if current_line_1 { steps } else { existing_line_steps };
                            let steps_l2 = if current_line_1 { existing_line_steps } else { steps };
                            *node = CircuitNode::Crossing { steps_l1, steps_l2 };
                            crossings.push(CircuitCrossingAt { coord, steps_l1, steps_l2 });
                        }
                    },
                }
            }
        }
        Ok(crossings)
    }

    fn bounds(&self) -> Bounds {
        let mut min_x = 0i64;
        let mut max_x = 0i64;
        let mut min_y = 0i64;
        let mut max_y = 0i64;
        for coord in self.nodes.keys() {
            if coord.0 < min_x { min_x = coord.0 }
            if coord.0 > max_x { max_x = coord.0 }
            if coord.1 < min_y { min_y = coord.1 }
            if coord.1 > max_y { max_y = coord.1 }
        }
        Bounds { min: Coord(min_x, min_y), max: Coord(max_x, max_y) }
    }

    #[allow(dead_code)]
    fn render(&self) -> String {
        let bounds = self.bounds();
        let mut s = String::new();
        for y in (bounds.min.1..=bounds.max.1).rev() {
            for x in bounds.min.0..=bounds.max.0 {
                let character = match self.nodes.get(&Coord(x, y)) {
                    Some(CentralPort) => 'O',
                    Some(CircuitNode::Line { line: CircuitLine::_1, .. }) => '1',
                    Some(CircuitNode::Line { line: CircuitLine::_2, .. }) => '2',
                    Some(CircuitNode::Crossing { .. }) => 'X',
                    None => '.'
                };
                s.push(character);
            }
            s.push('\n');
        }
        s
    }

    fn parse_instruction(s: &str) -> Result<CircuitInstruction, String> {
        let mut chars = s.chars();
        match chars.next() {
            None => Err(String::from("Empty instruction!")),
            Some(c) => {
                let direction = CircuitDirection::from_char(c)?;
                let number_str = chars.as_str();
                let steps = number_str.parse::<u32>()
                    .map_err(|err| format!("Can't parse '{}' as number: {}", number_str, err))?;
                Ok(CircuitInstruction(direction, steps))
            },
        }
    }
}