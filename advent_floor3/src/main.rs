fn main() {
    manhanttanDistance(361527);
    fillSquares(2);
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
            current_number = sumNeighbours(&current_values.last().unwrap(), &current_values);
            x += 1;
            current_values.push([x, y, current_number]);
        }

        // Move up by step
        for i in 0..step {
            current_number = sumNeighbours(&current_values.last().unwrap(), &current_values);
            y += 1;

            current_values.push([x, y, current_number]);
        }           

        // Move left by step + 1
        for i in 0..(step + 1) {
            current_number = sumNeighbours(&current_values.last().unwrap(), &current_values);
            x -= 1;

            current_values.push([x, y, current_number]);
        }

        // Move down by step + 1 
        for i in 0..(step + 1) {
            current_number = sumNeighbours(&current_values.last().unwrap(), &current_values);
            y -= 1;

            current_values.push([x, y, current_number]);
        }
    }

    println!("{:?}", current_values);
}

fn sumNeighbours(input: &[i32; 3], current_values: &Vec<[i32; 3]>) -> i32 {

    let input_x = input[0];
    let input_y = input[1];

    let mut result = 1;

    for value in current_values {
        let x = value[0];
        let y = value[1];
        let num = value[2];

        println!("input: {:?}", input);
        println!("{:?} {:?}", x - input_x, y - input_y);

        if (x - input_x).abs() == 1 || (y - input_y).abs() == 1 {
            println!("num to add: {:?}", num);
            result += num;
        }
    }
    println!("====");
    return result;
}