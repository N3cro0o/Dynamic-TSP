use dyn_prog::{io, tsp, Matrix};
use std::time::{Duration, SystemTime};

const TEST_SIZE: [usize; 5] = [8, 10, 12, 14, 15];

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
    'main: loop {
        // Setup
        let mut input_buff = String::new();
        let input_num;

        // Menu text
        {
            println!("--------------------------------------------------------------------------");
            println!("Wilkommen to über TSP program! Please select desired komiwojażer option!");
            println!("0.  Print matrix\n1.  Density-based generation.\n2.  Read from file.\n3.  Brute-force TSP.\n4.  Worse dynamic TSP\n5.  Dynamic TSP");
            println!("10. Stress test\n11. Delete output file\n\nInputting anything else will close the application");
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

            // 101 => { // Old read function from file
            //     println!("Input directory path to the file. Can be local or global.");
            //     input_buff.clear();
            //     let path: Option<&str> = match std::io::stdin().read_line(&mut input_buff) {
            //         Ok(_) => Some(&input_buff.trim()),
            //         Err(err) => {
            //             println!("{err}");
            //             None
            //         }
            //     };
            //     main_matrix = io::create_matrix_from_file(path).unwrap();
            // }

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
                    Some((x, y)) => (x,y),
                    None => {
                        println!("Error! Hamiltonian cycle doesn't exist");
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
                let vec;
                let start_timestamp = SystemTime::now();
                (vec, dist) = tsp::tsp_dyn(&main_matrix).unwrap();
                let end_timestamp = SystemTime::now();
                let dur = SystemTime::duration_since(&end_timestamp, start_timestamp).unwrap();
                println!("Dist = {dist}, vec = {vec:?}");
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
            10 => {
                let mut vec: Vec<usize>;
                let mut dist: usize;
                let mut naive: Option<Duration> = None;
                let mut worse: Option<Duration> = None;
                let mut dynamic: Option<Duration> = None;
                let mut start_time: SystemTime;
                let mut end_time: SystemTime;

                println!("Insert number of iterations:");
                input_buff.clear();
                std::io::stdin().read_line(&mut input_buff).expect("Something wrong");
                let x = input_buff.trim().parse::<usize>().expect("Should be a number");

                for num in TEST_SIZE {
                    println!("{num}");
                    for _ in 0..x {
                        naive = None;
                        worse = None;
                        dynamic = None;
                        main_matrix = Matrix::new_with_density(num, 1.0).randomize();
                        // naive only to max 13
                        if num < 13 {
                            start_time = SystemTime::now();
                            (vec, dist) = match tsp::tsp_standard(&main_matrix) {
                                Some(tup) => tup,
                                None => (vec![usize::MAX], usize::MAX)
                            };
                            end_time = SystemTime::now();
                            naive = Some(SystemTime::duration_since(&end_time, start_time).unwrap());
                            io::store_test_data_in_file(num, dist, vec.clone(), naive, "naive").unwrap();
                        }
                        // my take
                        start_time = SystemTime::now();
                        (vec, dist) = match tsp::tsp_dyn(&main_matrix) {
                            Some(tup) => tup,
                            None => (vec![usize::MAX], usize::MAX)
                        };
                        end_time = SystemTime::now();
                        worse = Some(SystemTime::duration_since(&end_time, start_time).unwrap());
                        io::store_test_data_in_file(num, dist, vec.clone(), worse, "personal take").unwrap();
                        // dynamic
                        start_time = SystemTime::now();
                        (dist) = tsp::tsp_dyn_new(&main_matrix) as usize;
                        end_time = SystemTime::now();
                        dynamic = Some(SystemTime::duration_since(&end_time, start_time).unwrap());
                        io::store_test_data_in_file(num, dist, vec.clone(), dynamic, "held-karp").unwrap();
                    }
                }
            }

            11 => {
                if let Err(err) = io::clear_output_file() {
                    println!("{err}");
                }
            }

            // 20 => {
            //     println!("Number larger than 0");
            //     input_buff.clear();
            //     std::io::stdin().read_line(&mut input_buff).expect("Wrong value");
            //     let num = input_buff.trim().parse::<usize>().unwrap();
            //     println!("{}", tsp::print_all_permutations(num));
            // }

            _ => {
                break 'main;
            }
        };
    }
}
