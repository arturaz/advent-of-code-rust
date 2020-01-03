use crate::open_file_first_arg;
use std::env::Args;
use std::io::BufRead;
use std::collections::HashMap;
use std::rc::Rc;

pub fn main1(args: &mut Args) -> Result<u32, String> {
    main_orbits(args).map(|o| o.count())
}

pub fn main2(args: &mut Args) -> Result<u32, String> {
    let orbits = main_orbits(args)?;
    let you = OrbitPlanet(Rc::from("YOU"));
    let san = OrbitPlanet(Rc::from("SAN"));
    let you_orbits = orbits.get(&you)?;
    let san_orbits = orbits.get(&san)?;
    let parent = orbits.common_parent(&you, &san)?;
//    println!("you={:?}\nsan={:?}\nparent={:?}", you_orbits, san_orbits, parent);
    Ok(you_orbits.distance + san_orbits.distance - 2 - (parent.distance - 1) * 2)
}

fn main_orbits(args: &mut Args) -> Result<Orbits, String> {
    let reader = open_file_first_arg(args)?;
    let relationships_res: Result<Vec<Relationship>, String> =
        reader.lines().map(|res|
            res
                .map_err(|err| format!("Failed reading lines: {}", err))
                .and_then(|s| Relationship::parse(&s))
        ).collect();

    let orbits = Orbits::new(relationships_res?.into_iter())?;
    Ok(orbits)
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct OrbitPlanet(Rc<str>);

#[derive(Debug, Clone, Eq, PartialEq)]
enum OrbitPoint { CenterOfMass, Planet(OrbitPlanet) }

struct Relationship { parent: OrbitPoint, child: OrbitPlanet }
impl Relationship {
    fn parse(s: &str) -> Result<Relationship, String> {
        let mut split = s.splitn(2, ")");
        let parent_str = split.nth(0).ok_or(format!("Can't find part 0 in '{}'", s))?;
        let parent =
            if parent_str == "COM" { OrbitPoint::CenterOfMass }
            else { OrbitPoint::Planet(OrbitPlanet(Rc::from(parent_str))) };
        let child = split.nth(0).ok_or(format!("Can't find part 1 in '{}'", s))?;
        Ok(Relationship { parent, child: OrbitPlanet(Rc::from(child)) })
    }
}

#[derive(Debug, Clone)]
struct MaybeOrbitsAround { orbits: OrbitPoint, distance: Option<u32> }

#[derive(Debug)]
struct OrbitsAround { orbits: OrbitPoint, distance: u32 }

struct Orbits {
    map: HashMap<OrbitPlanet, OrbitsAround>
}
impl Orbits {
    fn new(relationships: impl IntoIterator<Item = Relationship>) -> Result<Orbits, String> {
        type OrbitsInner = HashMap<OrbitPlanet, MaybeOrbitsAround>;
        let mut orbits = OrbitsInner::new();
        for relationship in relationships {
            if let Some(existing_parent) = orbits.get(&relationship.child) {
                return Err(format!(
                    "{:?} already orbits {:?} but wants to orbit {:?}!",
                    relationship.child, existing_parent, relationship.parent
                ));
            } else {
                orbits.insert(
                    relationship.child,
                    MaybeOrbitsAround { orbits: relationship.parent, distance: None }
                );
            }
        }

        fn distance_for(o: &mut OrbitsInner, p: &OrbitPlanet) -> Result<u32, String> {
            let entry = o.get(p).ok_or_else(|| format!("Can't find {:?}!", p))?;
            if let Some(distance) = entry.distance { return Ok(distance) }

            let distance = match entry.orbits {
                OrbitPoint::CenterOfMass => 1,
                OrbitPoint::Planet(ref planet) => {
                    let planet_cloned = (*planet).clone();
                    1 + distance_for(o, &planet_cloned)?
                },
            };
            o.get_mut(p).unwrap().distance = Some(distance);
            Ok(distance)
        }

        // Can't iterate the keys and mutate them at the same time, because mutation might invalidate
        // the reference. Using https://docs.rs/im/14.1.0/im/ would probably help here.
        let cloned_orbits: Vec<_> = orbits.iter().map(|e| (e.0.clone(), e.1.orbits.clone())).collect();
        let mut orbits_final = Orbits { map: HashMap::new() };
        for (child, parent) in cloned_orbits {
            let distance = distance_for(&mut orbits, &child)?;
            orbits_final.map.insert(
                child.clone(),
                OrbitsAround { orbits: parent, distance }
            );
        }
        Ok(orbits_final)
    }

    fn count(&self) -> u32 {
        self.map.iter().fold(0u32, |sum, (_, orbits_around)| sum + orbits_around.distance)
    }

    fn get(&self, key: &OrbitPlanet) -> Result<&OrbitsAround, String> {
        self.map.get(key).ok_or_else(|| format!("Unknown {:?}", key))
    }

    fn parent_of<'a>(&self, p: &'a OrbitPoint) -> Result<(&'a OrbitPlanet, &OrbitsAround), String> {
        match p {
            OrbitPoint::CenterOfMass =>
                Err(String::from("Center of mass does not have a parent!")),
            OrbitPoint::Planet(planet) =>
                self.get(planet).map(|orbits_around| (planet, orbits_around)),
        }
    }

    fn common_parent(&self, key1: &OrbitPlanet, key2: &OrbitPlanet) -> Result<&OrbitsAround, String> {
        let mut orbits_around1 = self.get(key1)?;
        let mut orbits_around2 = self.get(key2)?;

        while orbits_around1.orbits != orbits_around2.orbits {
//            println!("a={:?} b={:?}", orbits_around1, orbits_around2);
            if orbits_around1.distance > orbits_around2.distance {
                let (_, b) = self.parent_of(&orbits_around1.orbits)?;
                orbits_around1 = b;
            } else {
                let (_, b) = self.parent_of(&orbits_around2.orbits)?;
                orbits_around2 = b;
            }
        }
        Ok(orbits_around1)
    }
}