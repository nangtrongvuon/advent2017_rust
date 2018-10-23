fn main() {
    manhanttanDistance(361527);
}

fn manhanttanDistance(input: usize) {
    let mut total = 1;
    let mut level = 1;

    while total < input {
        level += 2;
        total = level * level;
    }

    let offset = total - input;
    let distance_from_input_to_corner = offset % (level - 1);

    let distance_from_corner_to_center = (level - 1) / 2;

    let distance_from_center_to_input = if level / 2 > distance_from_input_to_corner {
        level / 2 - distance_from_input_to_corner
    } else {
        distance_from_input_to_corner - level / 2
    };

    println!("{:?}", distance_from_corner_to_center + distance_from_center_to_input);
}