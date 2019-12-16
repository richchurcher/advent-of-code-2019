use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn get_input(input: &str) -> HashMap<String, Orbit> {
    let orbits: Vec<(String, String)> = input
        .lines()
        .map(|l| l.split(")").map(|s| s.to_string()))
        .map(|mut bodies| bodies.next_tuple().unwrap())
        .collect();

    let mut orbit_map = HashMap::new();
    orbit_map.insert(
        orbits[0].0.to_string(),
        Orbit {
            object: orbits[0].0.to_string(),
            in_orbit: None,
        },
    );

    for orbit in orbits {
        let (object1, object2) = orbit;
        if !orbit_map.contains_key(&object1) {
            orbit_map.insert(
                object1.to_string(),
                Orbit {
                    object: object1.to_string(),
                    in_orbit: None,
                },
            );
        }

        orbit_map.insert(
            object2.to_string(),
            Orbit {
                object: object2.to_string(),
                in_orbit: Some(object1.to_string()),
            },
        );
    }

    orbit_map
}

#[derive(Debug)]
pub struct Orbit {
    object: String,
    in_orbit: Option<String>,
}

fn count_orbits(orbit: &Orbit, orbit_map: &HashMap<String, Orbit>) -> usize {
    match &orbit.in_orbit {
        Some(object) => 1 + count_orbits(orbit_map.get(object).unwrap(), orbit_map),
        None => 0,
    }
}

#[aoc(day6, part1)]
pub fn orbit_checksum(orbit_map: &HashMap<String, Orbit>) -> usize {
    let mut total = 0;
    for orbit in orbit_map {
        total += count_orbits(orbit.1, orbit_map);
    }

    total
}

fn trace_path(
    orbit: &str,
    orbit_map: &HashMap<String, Orbit>,
    path: &mut Vec<String>,
) -> Vec<String> {
    let o = orbit_map.get(orbit).unwrap();
    match &o.in_orbit {
        Some(object) => {
            path.push(object.to_string());
            trace_path(&object, orbit_map, path)
        }
        None => path.to_vec(),
    }
}

#[aoc(day6, part2)]
pub fn transfers_to_santa(orbit_map: &HashMap<String, Orbit>) -> usize {
    let own_path = trace_path("YOU", orbit_map, &mut vec![]);
    let santa_path = trace_path("SAN", orbit_map, &mut vec![]);
    for i in 1..santa_path.len() {
        match own_path.iter().position(|o| *o == santa_path[i]) {
            Some(hops) => return i + hops,
            None => continue,
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn orbit_checksum_example() {
        assert_eq!(
            orbit_checksum(&get_input(
                "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"
            )),
            42
        )
    }

    #[test]
    fn orbital_transfers_example() {
        assert_eq!(
            transfers_to_santa(&get_input(
                "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"
            )),
            4
        )
    }
}
