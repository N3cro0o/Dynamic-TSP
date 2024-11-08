use dyn_prog::{io, tsp, Matrix};
use std::time::{Duration, SystemTime};

const TEST_SIZE: [usize; 6] = [5, 10, 12, 13, 14, 15];

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
                let vec;
                let start_timestamp = SystemTime::now();
                (vec, dist) = dyn_prog::tsp::tsp_dyn_new(&main_matrix).unwrap();
                let end_timestamp = SystemTime::now();
                let dur = SystemTime::duration_since(&end_timestamp, start_timestamp).unwrap();
                println!("Dist = {dist}, vec = {vec:?}");
                println!("Time {}", dur.as_secs_f64());
            }
            10 => {
                let mut vec_n: Vec<usize>;
                let mut vec_p: Vec<usize>;
                let mut vec_d: Vec<usize>;
                let mut dist_n: usize;
                let mut dist_p: usize;
                let mut dist_d: usize;

                let mut naive: Option<Duration> = None;
                let mut worse: Option<Duration> = None;
                let mut dynamic: Option<Duration> = None;
                let mut start_time: SystemTime;
                let mut end_time: SystemTime;

                println!("Insert number of iterations:");
                input_buff.clear();
                std::io::stdin().read_line(&mut input_buff).expect("Something wrong");
                let x = input_buff.trim().parse::<usize>().expect("Should be a number");

                'test: for num in TEST_SIZE {
                    println!("----------------------| {num} |-------------------------");
                    for _ in 0..x {
                        naive = None;
                        worse = None;
                        dynamic = None;
                        main_matrix = Matrix::new_with_density(num, 1.0).randomize();
                        
                        // my take
                        start_time = SystemTime::now();
                        (vec_p, dist_p) = match tsp::tsp_dyn(&main_matrix) {
                            Some(tup) => tup,
                            None => (vec![usize::MAX], usize::MAX)
                        };
                        end_time = SystemTime::now();
                        worse = Some(SystemTime::duration_since(&end_time, start_time).unwrap());
                        io::store_test_data_in_file(num, dist_p, vec_p.clone(), worse, "dyn_struct").unwrap();

                        // dynamic
                        start_time = SystemTime::now();
                        (vec_d, dist_d) = match tsp::tsp_dyn_new(&main_matrix){
                            Some(tup) => tup,
                            None => (vec![usize::MAX], usize::MAX)
                        };
                        end_time = SystemTime::now();
                        dynamic = Some(SystemTime::duration_since(&end_time, start_time).unwrap());
                        io::store_test_data_in_file(num, dist_d, vec_d.clone(), dynamic, "dyn_held-karp").unwrap();

                        // naive only to max 13
                        if num <= 13 {
                            start_time = SystemTime::now();
                            (vec_n, dist_n) = match tsp::tsp_standard(&main_matrix) {
                                Some(tup) => tup,
                                None => (vec![usize::MAX], usize::MAX)
                            };
                            end_time = SystemTime::now();
                            naive = Some(SystemTime::duration_since(&end_time, start_time).unwrap());
                            io::store_test_data_in_file(num, dist_n, vec_n.clone(), naive, "naive").unwrap();
                        }
                        else {
                            vec_n = vec_p.clone();
                            dist_n = dist_p;
                        }
                        
                        // Test
                        if dist_p != dist_d || dist_p != dist_n || dist_d != dist_n {
                            println!("Distance error");
                            break 'test;
                        }

                        // Don't check path vectors, because multiple cycles can exist with different paths
                        if !vec_check(&main_matrix, &vec_n, dist_n) || !vec_check(&main_matrix, &vec_p, dist_p) ||
                            !vec_check(&main_matrix, &vec_d, dist_d) {
                            println!("Path error");
                            break 'test;
                        }

                        // But we can check if the path is correct

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

fn vec_check(m: &Matrix, vec: &Vec<usize>, dist: usize) -> bool {
    let mut d = 0;
    let mut last = 0;
    for i in vec.iter(){
        d += m.matrix[last][*i] as usize;
        last = *i;
    }
    dist == d
}