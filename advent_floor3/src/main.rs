fn main() {
    manhanttanDistance(361527);
    fillSquares(361527);
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

fn fillSquares(input: usize) {
    let mut step = 1;
    let mut current_number: i32 = 1;

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut current_values = vec![[x, y, current_number]];

    while current_number < input as i32 {
        // Move right by step
        for i in 0..step {
            x += 1;
            current_number = sumNeighbours(&[x, y, 0], &current_values);
            current_values.push([x, y, current_number]);
        }

        // Move up by step
        for i in 0..step {
            y += 1;
            current_number = sumNeighbours(&[x, y, 0], &current_values);
            current_values.push([x, y, current_number]);
        }           

        // Move left by step + 1
        for i in 0..(step + 1) {
            x -= 1;
            current_number = sumNeighbours(&[x, y, 0], &current_values);
            current_values.push([x, y, current_number]);
        }

        // Move down by step + 1 
        for i in 0..(step + 1) {
            y -= 1;
            current_number = sumNeighbours(&[x, y, 0], &current_values);
            current_values.push([x, y, current_number]);
        }

        step += 2;
    }

    println!("{:?}", current_values);
}

fn sumNeighbours(input: &[i32; 3], current_values: &Vec<[i32; 3]>) -> i32 {

    let input_x = input[0];
    let input_y = input[1];
    let mut result = 0;

    for value in current_values {
        let x = value[0];
        let y = value[1];
        let num = value[2];

        if (input_x - x).abs() <= 1 && (input_y - y).abs() <= 1 {
            result += num;
        }
    }

    return result;
}