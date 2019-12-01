#[aoc_generator(day1)]
pub fn converter(input: &str) -> Vec<f32> {
    input.lines().map(|l| l.parse::<f32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn fuel_counter(masses: &[f32]) -> f32 {
    masses.iter().map(&fuel_by_mass).sum()
}

pub fn fuel_by_mass(mass: &f32) -> f32 {
    (mass / 3.).floor() - 2.
}

pub fn calculate_module_fuel(mass: &f32) -> f32 {
    let fuel = fuel_by_mass(mass);
    let fuel_for_fuel = fuel_by_mass(&fuel);
    if fuel_for_fuel < 0. {
        return fuel;
    }
    return fuel + calculate_module_fuel(&fuel);
}

#[aoc(day1, part2)]
pub fn smart_fuel_counter(masses: &[f32]) -> f32 {
    masses.iter().map(&calculate_module_fuel).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fuel_counter_is_correct_for_small_sample() {
        assert_eq!(
            fuel_counter(&vec![12., 14., 1969., 100756.]),
            2. + 2. + 654. + 33583.
        )
    }

    #[test]
    fn fuel_by_mass_is_correct_for_12() {
        assert_eq!(fuel_by_mass(&12.), 2.)
    }

    #[test]
    fn fuel_by_mass_is_correct_for_100756() {
        assert_eq!(fuel_by_mass(&100756.), 33583.)
    }

    #[test]
    fn calculate_module_fuel_is_correct_for_12() {
        assert_eq!(calculate_module_fuel(&12.), 2.)
    }

    #[test]
    fn calculate_module_fuel_is_correct_for_1969() {
        assert_eq!(calculate_module_fuel(&1969.), 966.)
    }
}
