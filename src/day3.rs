use array_tool::vec::Intersect;

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Vec<PathSegment>> {
    input
        .lines()
        .map(|wire| {
            wire.split(",")
                .map(|segment| PathSegment {
                    direction: segment.chars().next().unwrap(),
                    length: segment[1..].parse::<i32>().unwrap(),
                })
                .collect()
        })
        .collect()
}

#[derive(Debug, PartialEq)]
pub struct PathSegment {
    direction: char,
    length: i32,
}

pub fn get_path(segment: &PathSegment, origin_x: i32, origin_y: i32) -> Vec<(i32, i32)> {
    match segment.direction {
        'R' => ((origin_x + 1)..=(origin_x + segment.length))
            .map(|n| (n, origin_y))
            .collect(),
        'L' => ((origin_x - segment.length)..origin_x)
            .rev()
            .map(|n| (n, origin_y))
            .collect(),
        'U' => ((origin_y + 1)..=(origin_y + segment.length))
            .map(|n| (origin_x, n))
            .collect(),
        'D' => ((origin_y - segment.length)..origin_y)
            .rev()
            .map(|n| (origin_x, n))
            .collect(),
        _ => unreachable!(),
    }
}

pub fn get_entire_path(segments: &[PathSegment]) -> Vec<(i32, i32)> {
    let mut last: (i32, i32) = (0, 0);
    segments
        .iter()
        .map(|seg| {
            let path = get_path(seg, last.0, last.1);
            last = *path.last().unwrap();
            path
        })
        .flatten()
        .collect()
}

#[aoc(day3, part1)]
pub fn find_closest_intersection(wires: &[Vec<PathSegment>]) -> i32 {
    let first_wire = get_entire_path(&wires[0]);
    let second_wire = get_entire_path(&wires[1]);
    let intersections = first_wire.intersect(second_wire.clone());
    intersections
        .iter()
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
pub fn find_closest_intersection_by_distance_travelled(wires: &[Vec<PathSegment>]) -> usize {
    let first_wire = get_entire_path(&wires[0]);
    let second_wire = get_entire_path(&wires[1]);
    let intersections = first_wire.intersect(second_wire.clone());
    let mut steps: Vec<usize> = intersections
        .iter()
        .map(|&intersection| {
            let first_steps = first_wire.iter().position(|&x| x == intersection).unwrap() + 1;
            let second_steps = second_wire.iter().position(|&x| x == intersection).unwrap() + 1;
            first_steps + second_steps
        })
        .collect();
    steps.sort();
    steps[0]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generator_is_correct() {
        assert_eq!(
            generator("R1,L2\nD4,U5"),
            vec![
                vec![
                    PathSegment {
                        direction: 'R',
                        length: 1,
                    },
                    PathSegment {
                        direction: 'L',
                        length: 2,
                    },
                ],
                vec![
                    PathSegment {
                        direction: 'D',
                        length: 4,
                    },
                    PathSegment {
                        direction: 'U',
                        length: 5,
                    }
                ],
            ]
        )
    }

    #[test]
    fn get_path_correct_for_right() {
        assert_eq!(
            get_path(
                &PathSegment {
                    direction: 'R',
                    length: 2
                },
                0,
                0
            ),
            vec![(1, 0), (2, 0)]
        )
    }

    #[test]
    fn get_path_correct_for_left() {
        assert_eq!(
            get_path(
                &PathSegment {
                    direction: 'L',
                    length: 2
                },
                2,
                0
            ),
            vec![(1, 0), (0, 0)]
        )
    }

    #[test]
    fn get_path_correct_for_up() {
        assert_eq!(
            get_path(
                &PathSegment {
                    direction: 'U',
                    length: 2
                },
                0,
                0
            ),
            vec![(0, 1), (0, 2)]
        )
    }

    #[test]
    fn get_path_correct_for_down() {
        assert_eq!(
            get_path(
                &PathSegment {
                    direction: 'D',
                    length: 2
                },
                0,
                2
            ),
            vec![(0, 1), (0, 0)]
        )
    }

    #[test]
    fn get_path_correct_for_down_below_zero() {
        assert_eq!(
            get_path(
                &PathSegment {
                    direction: 'D',
                    length: 2
                },
                0,
                0
            ),
            vec![(0, -1), (0, -2)]
        )
    }

    #[test]
    fn get_path_correct_for_left_below_zero() {
        assert_eq!(
            get_path(
                &PathSegment {
                    direction: 'L',
                    length: 2
                },
                0,
                0
            ),
            vec![(-1, 0), (-2, 0)]
        )
    }

    #[test]
    fn get_entire_path_returns_correct_coords() {
        assert_eq!(
            get_entire_path(&vec![
                PathSegment {
                    direction: 'U',
                    length: 5
                },
                PathSegment {
                    direction: 'R',
                    length: 5
                }
            ]),
            vec![
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (0, 5),
                (1, 5),
                (2, 5),
                (3, 5),
                (4, 5),
                (5, 5)
            ]
        )
    }

    #[test]
    fn simple_complete_example_is_2() {
        assert_eq!(
            find_closest_intersection(&generator("R1,U5,L1,D2\nU1,R2,U3,L2")),
            2
        )
    }

    #[test]
    fn first_test_example_is_159() {
        assert_eq!(
            find_closest_intersection(&generator(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            )),
            159
        )
    }

    #[test]
    fn second_test_example_is_135() {
        assert_eq!(
            find_closest_intersection(&generator(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            135
        )
    }

    #[test]
    fn part2_simple_example() {
        assert_eq!(
            find_closest_intersection_by_distance_travelled(&generator("R8,U5,L5,D3\nU7,R6,D4,L4")),
            30
        )
    }

    #[test]
    fn part2_example1() {
        assert_eq!(
            find_closest_intersection_by_distance_travelled(&generator(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            )),
            610
        )
    }

    #[test]
    fn part2_example2() {
        assert_eq!(
            find_closest_intersection_by_distance_travelled(&generator(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            410
        )
    }
}
