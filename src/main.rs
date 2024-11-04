use dyn_prog::{io, tsp, Matrix};
use std::time::SystemTime;

fn main() {
    /*
        let matrix = io::console_create_matrix_from_density().randomize();
        let vec;
        let dist;
        matrix.print_matrix();
        let start_timestamp = SystemTime::now();
        (vec, dist) = match tsp::tsp_standard(&matrix) {
            Ok((x, y)) => (x,y),
            Err(err) => {
                println!("Error! {err}");
                return;
            }
        };
        let end_timestamp = SystemTime::now();
        let dur = SystemTime::duration_since(&end_timestamp, start_timestamp).unwrap();
        println!("Dist = {dist}, vec = {vec:?}");
        println!("Time {}", dur.as_secs_f64());
    */
    let mut main_matrix = Matrix::empty();
    println!("{:b}", 15_i32 ^ 1 << 2);
    'main: loop {
        // Setup
        let mut input_buff = String::new();
        let input_num;

        // Menu text
        {
            println!("--------------------------------------------------------------------------");
            println!("Wilkommen to über TSP program! Please select desired komiwojażer option!");
            println!("0. Print matrix\n1. Density-based generation.\n2. Read from file.\n3. Brute-force TSP.");
            println!();
        }

        // Read input line and parse to number
        match std::io::stdin().read_line(&mut input_buff){
            Ok(_) => (), // Return empty tuple, so called unit
            Err(err) => {
                println!("Error: {err}");
                continue 'main;
            }
        }
        input_num = match input_buff.trim().parse::<usize>() {
            Ok(u) => u,
            Err(_) => {
                println!("Input only natural numbers");
                continue 'main;
            }
        };

        // Menu logic
        match input_num {
            0 => {
                main_matrix.print_matrix();
            }

            1 => {
                main_matrix = io::console_create_matrix_from_density().randomize();
            }

            11 => { // Old read function from file
                println!("Input directory path to the file. Can be local or global.");
                input_buff.clear();
                let path: Option<&str> = match std::io::stdin().read_line(&mut input_buff) {
                    Ok(_) => Some(&input_buff.trim()),
                    Err(err) => {
                        println!("{err}");
                        None
                    }
                };
                main_matrix = io::create_matrix_from_file(path).unwrap();
            }

            2 => {
                println!("Input directory path to the file. Can be local or global.");
                input_buff.clear();
                let path: Option<&str> = match std::io::stdin().read_line(&mut input_buff) {
                    Ok(_) => Some(&input_buff.trim()),
                    Err(err) => {
                        println!("{err}");
                        None
                    }
                };
                println!("{}", input_buff);
                main_matrix = io::create_matrix_form_file_matrix(path).unwrap();
            }

            3 => {
                if main_matrix.is_empty() == true {println!("Matrix is empty!"); continue 'main;}
                let vec;
                let dist;
                let start_timestamp = SystemTime::now();
                (vec, dist) = match tsp::tsp_standard(&main_matrix) {
                    Ok((x, y)) => (x,y),
                    Err(err) => {
                        println!("Error! {err}");
                        continue 'main;
                    }
                };
                let end_timestamp = SystemTime::now();
                let dur = SystemTime::duration_since(&end_timestamp, start_timestamp).unwrap();
                println!("Dist = {dist}, vec = {vec:?}");
                println!("Time {}", dur.as_secs_f64());
            }
            4 => {
                if main_matrix.is_empty() == true {println!("Matrix is empty!"); continue 'main;}
                let dist;
                let start_timestamp = SystemTime::now();
                dist=tsp::tsp_dyn(&main_matrix).unwrap();
                let end_timestamp = SystemTime::now();
                let dur = SystemTime::duration_since(&end_timestamp, start_timestamp).unwrap();
                println!("Dist = {dist}");
                println!("Time {}", dur.as_secs_f64());
            }

            5 => {
                if main_matrix.is_empty() == true {println!("Matrix is empty!"); continue 'main;}

                let dist;
                let start_timestamp = SystemTime::now();
                dist = dyn_prog::tsp::tsp_dyn_new(&main_matrix);
                let end_timestamp = SystemTime::now();
                let dur = SystemTime::duration_since(&end_timestamp, start_timestamp).unwrap();
                println!("Dist = {dist}");
                println!("Time {}", dur.as_secs_f64());
            }

            20 => {
                println!("Number larger than 0");
                input_buff.clear();
                std::io::stdin().read_line(&mut input_buff).expect("Wrong value");
                let num = input_buff.trim().parse::<usize>().unwrap();
                println!("{}", tsp::print_all_permutations(num));
            }


            _ => {
                break 'main;
            }
        };
    }
}
