use crate::open_file_first_arg;
use std::io::BufRead;
use std::collections::HashMap;
use std::env::Args;
use crate::program3::CircuitNode::CentralPort;

fn main(args: &mut Args) {
//    open_file_first_arg(args).and_then(|reader| {
//        let line1 = reader.lines().next();
//        let line2 = reader.lines().next();
//    });
    unimplemented!()
}

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
}

#[derive(Eq, PartialEq, Hash)] struct Coord(i64, i64);
enum CircuitNode { CentralPort, Line1, Line2 }
struct CircuitInstruction(CircuitDirection, u32);

struct Circuit {
    nodes: HashMap<Coord, CircuitNode>
}
impl Circuit {
    fn new() -> Circuit {
        let mut c = Circuit { nodes: HashMap::new() };
        c.nodes.insert(Coord(0, 0), CentralPort);
        c
    }

    // R998,U494,L814,D519,R407,U983,R307,D745,R64,D29,L935,D919,L272,D473,R689,U560,L942,U264,R816,U745,R209,U227,R241,U111,L653,D108,R823,U254,L263,U987,L368,D76,R665,D646,L759,U425,L581,D826,R829,D388,L234,U33,L48,U598,L708,D764,L414,D75,L163,U802,L183,U893,L486,U947,L393,D694,L454,D600,R377,U312,R89,D178,L652,D751,R402,D946,R213,U985,R994,D336,R573,D105,L442,U965,R603,U508,L17,U191,L37,U678,L506,U823,R878,D709,L348,U167,L355,U314,L164,D672,L309,U895,R358,D769,R869,U598,R63,D68,R105,U133,R357,U588,L154,D631,L939,D235,R506,D885,R958,D896,L195,U292,L952,D616,L824,D497,R99,D121,R387,D155,L70,U580,L890,D368,L910,U645,L786,U977,R9,U781,L454,U783,L382,U321,L195,U196,L239,U764,R18,D71,R97,U77,L803,U963,L704,U94,L511,U747,L798,D905,L679,D135,R455,U650,R947,U14,L722,D245,L490,D183,L276,U559,L901,D767,R827,U522,L380,U494,R402,U70,R589,D582,R206,U756,L989,U427,L704,D864,R885,D9,R872,U454,R912,U752,R197,U304,L728,U879,R456,D410,L141,U473,R246,U498,R443,D297,R333,D123,R12,D665,R684,D531,R601,D13,L260,U60,R302,D514,R416,D496,L562,D334,L608,U74,R451,U251,R961,U166,L368,U146,R962,U973,R120,U808,R480,D536,L690,D958,R292,U333,R656,U305,R46,U831,L756,D907,L638,D969,L445,U541,R784,U148,R338,D264,R72,D637,R759,D709,L611,D34,R99,U305,R143,D191,R673,D753,R387,U994,R720,D896,R95,U703,L499,D453,R96,U808,L485,U127,L856,U357,L543,U382,R411,U969,L532,U64,R303,U457,L412,D140,R146,D67,R147,D681,L1,D994,L876,D504,R46,U683,L992,U640,L663,D681,L327,U840,R543,U97,R988,U792,R36
    fn update_from_str(&mut self, s: &str) -> Result<(), String> {
        let instructions_res: Result<Vec<CircuitInstruction>, String> =
            s.split(",").map(Circuit::parse_instruction).collect();
        let mut coord = Coord(0, 0);
        for instruction in instructions_res? {
            for idx in 1..instruction.1 {
                unimplemented!()
//                self.nodes.entry("")
            }
        }
        unimplemented!()
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