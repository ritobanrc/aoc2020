use crate::Part;
use euclid::{vec2, Rotation2D, UnknownUnit, Vector2D};

pub fn solutions(input: String, part: Part) -> i64 {
    let mut current: Vector2D<_, UnknownUnit> = vec2(0f32, 0f32);
    let mut direction = match part {
        Part::Part1 => vec2(1f32, 0f32),
        Part::Part2 => vec2(10f32, -1.),
    };

    for line in input.lines() {
        let (ins, arg) = line.split_at(1);
        let arg: f32 = arg.parse().unwrap();

        let thing_to_move = match part {
            Part::Part1 => &mut current,
            Part::Part2 => &mut direction,
        };

        match ins {
            "N" => *thing_to_move += vec2(0., -arg),
            "S" => *thing_to_move += vec2(0., arg),
            "E" => *thing_to_move += vec2(arg, 0.),
            "W" => *thing_to_move += vec2(-arg, 0.),
            "L" => {
                let rotation = Rotation2D::radians(arg.to_radians()).inverse();
                direction = rotation.transform_vector(direction)
            }
            "R" => {
                let rotation = Rotation2D::radians(arg.to_radians());
                direction = rotation.transform_vector(direction)
            }
            "F" => current += direction * arg,
            _ => panic!("Unrecognized input: {:?}", line),
        }
    }

    current.x.round().abs() as i64 + current.y.round().abs() as i64
}

#[test]
fn day12_test() {
    let input = "
F10
N3
F7
R90
F11"
    .trim();

    assert_eq!(25, solutions(input.to_owned(), Part::Part1));
}
