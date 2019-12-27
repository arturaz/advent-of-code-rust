use crate::open_file_first_arg;
use std::env::Args;
use std::io::BufRead;
use std::collections::HashMap;

fn main(args: &mut Args) -> Result<u32, String> {
    let reader = open_file_first_arg(args)?;
    let relationships_res: Result<Vec<Relationship>, String> =
        reader.lines().map(|res|
            res
                .map_err(|err| format!("Failed reading lines: {}", err))
                .and_then(|s| Relationship::parse(&s))
        ).collect();

    let mut orbits = Orbits { map: HashMap::<String, String>::new() };
    for relationship in relationships_res?.into_iter() {
        if let Some(existing_parent) = orbits.map.get(&relationship.child) {
            return Err(format!(
                "{} already orbits {} but wants to orbit {}!",
                relationship.child, existing_parent, relationship.parent
            ));
        } else {
            orbits.map.insert(relationship.child, relationship.parent);
        }
    }
    Ok(0)
}

struct Relationship { parent: String, child: String };
impl Relationship {
    fn parse(s: &str) -> Result<Relationship, String> {
        let mut split = s.splitn(2, ")");
        let parent = split.nth(0).ok_or(format!("Can't find part 0 in '{}'", s))?;
        let child = split.nth(0).ok_or(format!("Can't find part 1 in '{}'", s))?;
        Ok(Relationship { parent: String::from(parent), child: String::from(child) })
    }
}

struct Orbits {
    map: HashMap<String, String>
}
impl Orbits {
    fn count(&self) -> u32 {
        self.map.iter().fold(0u32, |sum, entry| {

        })
    }
}