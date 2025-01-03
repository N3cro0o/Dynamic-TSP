use dyn_prog::{io, matrix::{self, Matrix}, tsp};
use std::time::SystemTime;

const TEST_SIZE: [usize; 6] = [5, 10, 12, 13, 14, 15];
const TEST_SIZE_2: [usize; 6] = [50, 100, 125, 150, 175, 200];
const TEST_DENS: [f32; 5] = [1.0, 0.9, 0.8, 0.70, 0.5];

fn main() {
    let mut main_matrix = matrix::Matrix::empty();
    let mut vec = vec![0];
    let mut dist = 0;
    'main: loop {
        // Setup
        let mut input_buff = String::new();
        let input_num;

        // Menu text
        {
            println!("--------------------------------------------------------------------------");
            println!("Welcome to TSP program! Please select one of the options below");
            println!("0.  Print matrix\n1.  Density-based generation.\n2.  Read from file.\n3.  Permutations check\n\n11. Brute-force TSP.\n12. Worse dynamic TSP\n13. Dynamic TSP");
            println!("14. ACO TSP\n15. ACO parallel TSP\n16. Tabu search TSP\n17. Simulated annealing TSP\n");
            println!("21. Check generated path.\n22. Delete output file\n24. Test naive/dynamic\n25. Test ACO/SA/TS\n31. ACO parallel speed test\n\nAnything else will close the application");
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
                println!("Number larger than 0");
                input_buff.clear();
                std::io::stdin().read_line(&mut input_buff).expect("Wrong value");
                let num = input_buff.trim().parse::<usize>().unwrap();
                println!("{}", tsp::print_all_permutations(num));
            }

            11 => {
                if main_matrix.is_empty() == true {println!("Matrix is empty!"); continue 'main;}
                let start_timestamp = SystemTime::now();
                (vec, dist) = match tsp::naive::tsp_standard(&main_matrix) {
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

            12 => {
                if main_matrix.is_empty() == true {println!("Matrix is empty!"); continue 'main;}
                let start_timestamp = SystemTime::now();
                (vec, dist) = tsp::dynamic::tsp_dyn(&main_matrix).unwrap();
                let end_timestamp = SystemTime::now();
                let dur = SystemTime::duration_since(&end_timestamp, start_timestamp).unwrap();
                println!("Dist = {dist}, vec = {vec:?}");
                println!("Time {}", dur.as_secs_f64());
            }

            13 => {
                if main_matrix.is_empty() == true {println!("Matrix is empty!"); continue 'main;}
                let start_timestamp = SystemTime::now();
                (vec, dist) = dyn_prog::tsp::dynamic::tsp_dyn_new(&main_matrix).unwrap();
                let end_timestamp = SystemTime::now();
                let dur = SystemTime::duration_since(&end_timestamp, start_timestamp).unwrap();
                println!("Dist = {dist}, vec = {vec:?}");
                println!("Time {}", dur.as_secs_f64());
            }

            14 => {
                if main_matrix.is_empty() == true {println!("Matrix is empty!"); continue 'main;}
                let start_timestamp = SystemTime::now();
                (vec, dist) = dyn_prog::tsp::aco::tsp_aco(&main_matrix).unwrap();
                let end_timestamp = SystemTime::now();
                let dur = SystemTime::duration_since(&end_timestamp, start_timestamp).unwrap();
                println!("Dist = {dist}, vec = {vec:?}");
                println!("Time {}", dur.as_secs_f64());
            }

            15 => {
                if main_matrix.is_empty() == true {println!("Matrix is empty!"); continue 'main;}
                let start_timestamp = SystemTime::now();
                (vec, dist) = dyn_prog::tsp::aco::tsp_aco_thread(&main_matrix).unwrap();
                let end_timestamp = SystemTime::now();
                let dur = SystemTime::duration_since(&end_timestamp, start_timestamp).unwrap();
                println!("Dist = {dist}, vec = {vec:?}");
                println!("Time {}", dur.as_secs_f64());
            }

            16 => {
                if main_matrix.is_empty() == true {println!("Matrix is empty!"); continue 'main;}
                let start_timestamp = SystemTime::now();
                (vec, dist) = match dyn_prog::tsp::tabu::tabu_tsp(&main_matrix) {
                    Some (t) => t,
                    None => {
                        println!("Couldn't find a path");
                        (vec![], 0)
                    }
                };
                let end_timestamp = SystemTime::now();
                let dur = SystemTime::duration_since(&end_timestamp, start_timestamp).unwrap();
                println!("Dist = {dist}, vec = {vec:?}");
                println!("Time {}", dur.as_secs_f64());
            }

            17 => {
                if main_matrix.is_empty() == true {println!("Matrix is empty!"); continue 'main;}
                let start_timestamp = SystemTime::now();
                (vec, dist) = match dyn_prog::tsp::sa::tso_sa(&main_matrix) {
                    Some (t) => t,
                    None => {
                        println!("Couldn't find a path");
                        (vec![], 0)
                    }
                };
                let end_timestamp = SystemTime::now();
                let dur = SystemTime::duration_since(&end_timestamp, start_timestamp).unwrap();
                println!("Dist = {dist}, vec = {vec:?}");
                println!("Time {}", dur.as_secs_f64());
            }
            
            21 => {
                if main_matrix.check_length(&vec, dist) {
                    println!("Length is fine.")
                }
                else {
                    let mut d = 0;
                    let mut last = 0;
                    for i in vec.iter(){
                        d += main_matrix.matrix[last][*i] as usize;
                        last = *i;
                    }
                    println!("{d} in not the same as {dist}");
                }

                if main_matrix.check_cycle(&vec) {
                    println!("Cycle path is fine.")
                }
                else {
                    println!("Cycle repeats vertex");
                }
            }

            24 => {
                let mut vec_n: Vec<usize>;
                let mut vec_p: Vec<usize>;
                let mut vec_d: Vec<usize>;
                let mut dist_n: usize;
                let mut dist_p: usize;
                let mut dist_d: usize;

                let mut start_time: SystemTime;
                let mut end_time: SystemTime;

                println!("Insert number of iterations:");
                input_buff.clear();
                std::io::stdin().read_line(&mut input_buff).expect("Something wrong");
                let x = input_buff.trim().parse::<usize>().expect("Should be a number");

                'test: for num in TEST_SIZE {
                    for it in 0..x {
                        println!("Length: {num}, iteration: {:1}", it + 1);
                        main_matrix = matrix::Matrix::new_with_density(num, 1.0).randomize();
                        
                        // my take
                        start_time = SystemTime::now();
                        (vec_p, dist_p) = match tsp::dynamic::tsp_dyn(&main_matrix) {
                            Some(tup) => tup,
                            None => (vec![usize::MAX], usize::MAX)
                        };
                        end_time = SystemTime::now();
                        let worse = Some(SystemTime::duration_since(&end_time, start_time).unwrap());
                        io::store_test_data_in_file("output_test1.txt".to_string(), num, dist_p, 1.0, vec_p.clone(), worse, "dyn_struct").unwrap();

                        // dynamic
                        start_time = SystemTime::now();
                        (vec_d, dist_d) = match tsp::dynamic::tsp_dyn_new(&main_matrix){
                            Some(tup) => tup,
                            None => (vec![usize::MAX], usize::MAX)
                        };
                        end_time = SystemTime::now();
                        let dynamic = Some(SystemTime::duration_since(&end_time, start_time).unwrap());
                        io::store_test_data_in_file("output_test1.txt".to_string(), num, dist_d, 1.0, vec_d.clone(), dynamic, "dyn_held-karp").unwrap();

                        // naive only to max 13
                        if num <= 13 {
                            start_time = SystemTime::now();
                            (vec_n, dist_n) = match tsp::naive::tsp_standard(&main_matrix) {
                                Some(tup) => tup,
                                None => (vec![usize::MAX], usize::MAX)
                            };
                            end_time = SystemTime::now();
                            let naive = Some(SystemTime::duration_since(&end_time, start_time).unwrap());
                            io::store_test_data_in_file("output_test1.txt".to_string(), num, dist_n, 1.0, vec_n.clone(), naive, "naive").unwrap();
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
                        if !main_matrix.check_length(&vec_n, dist_n) || !main_matrix.check_length(&vec_p, dist_p) ||
                            !main_matrix.check_length(&vec_d, dist_d) {
                            println!("Path error");
                            break 'test;
                        }
                    }
                }
            }

            25 => {
                let mut vec_aco: Vec<usize>;
                let mut vec_sa: Vec<usize>;
                let mut vec_ts: Vec<usize>;
                let mut dist_aco: usize;
                let mut dist_sa: usize;
                let mut dist_ts: usize;
                let mut dist_hk: usize;
                let mut vec_hk: Vec<usize>;

                let mut aco_err = 0;
                let mut sa_err = 0;
                let mut ts_err = 0;

                let mut start_time: SystemTime;
                let mut end_time: SystemTime;

                println!("Insert number of comparasion iterations:");
                input_buff.clear();
                std::io::stdin().read_line(&mut input_buff).expect("Something wrong");
                let x = input_buff.trim().parse::<usize>().expect("Should be a number");

                println!("Insert number of main iterations:");
                input_buff.clear();
                std::io::stdin().read_line(&mut input_buff).expect("Something wrong");
                let y = input_buff.trim().parse::<usize>().expect("Should be a number");

                // Held karp - metaheuristic comparasion
                for d in TEST_DENS {
                    for it in 0..x {
                        println!("Density: {d:.3}, iteration: {:1}", it + 1);
                        main_matrix = matrix::Matrix::new_with_density(23, d).randomize();
                        // Held-Karp
                        start_time = SystemTime::now();
                        (vec_hk, dist_hk) = match tsp::dynamic::tsp_dyn_new(&main_matrix) {
                            Some(tup) => tup,
                            None => (vec![0], 0)
                        };
                        end_time = SystemTime::now();
                        let duart = Some(SystemTime::duration_since(&end_time, start_time).unwrap());
                        io::store_test_data_in_file("output-comp-test.txt".to_string(), 23, dist_hk, d, vec_hk.clone(), duart, "Held-Karp").unwrap();

                        // ACO
                        start_time = SystemTime::now();
                        (vec_aco, dist_aco) = match tsp::aco::tsp_aco_thread(&main_matrix) {
                            Some(tup) => tup,
                            None => (vec![0], 0)
                        };
                        end_time = SystemTime::now();
                        let duart = Some(SystemTime::duration_since(&end_time, start_time).unwrap());
                        io::store_test_data_in_file("output-comp-test.txt".to_string(), 23, dist_aco, d, vec_aco.clone(), duart, "ACO").unwrap();

                        // SA
                        start_time = SystemTime::now();
                        (vec_sa, dist_sa) = match tsp::sa::tso_sa(&main_matrix) {
                            Some(tup) => tup,
                            None => (vec![0], 0)
                        };
                        end_time = SystemTime::now();
                        let duart = Some(SystemTime::duration_since(&end_time, start_time).unwrap());
                        io::store_test_data_in_file("output-comp-test.txt".to_string(), 23, dist_sa, d, vec_sa.clone(), duart, "SA").unwrap();

                        // TS
                        start_time = SystemTime::now();
                        (vec_ts, dist_ts) = match tsp::tabu::tabu_tsp(&main_matrix) {
                            Some(tup) => tup,
                            None => (vec![0], 0)
                        };
                        end_time = SystemTime::now();
                        let duart = Some(SystemTime::duration_since(&end_time, start_time).unwrap());
                        io::store_test_data_in_file("output-comp-test.txt".to_string(), 25, dist_ts, d, vec_ts.clone(), duart, "TS").unwrap();

                        // Tests
                        // ACO
                        if !main_matrix.check_length(&vec_aco, dist_aco) || !main_matrix.check_cycle(&vec_aco) {
                            aco_err += 1;
                            println!("Aco vec: {:?}\nAco dist: {dist_aco}", vec_aco);
                        }
                        
                        // SA
                        if !main_matrix.check_length(&vec_sa, dist_sa) || !main_matrix.check_cycle(&vec_sa) {
                            sa_err += 1;
                            println!("Sa vec: {:?}\nSa dist: {dist_sa}", vec_sa);
                        }

                        // TS
                        if !main_matrix.check_length(&vec_ts, dist_ts) || !main_matrix.check_cycle(&vec_ts) {
                            ts_err += 1;
                            println!("Ts vec: {:?}\nTs dist: {dist_ts}", vec_ts);
                        }
                    }
                }
                println!("Errors:\nACO: {aco_err}\nSA: {sa_err}\nTS: {ts_err}");
                for l in TEST_SIZE_2 {
                    for it in 0..y {
                        println!("Length: {l}, iteration: {:1}", it + 1);
                        main_matrix = matrix::Matrix::new_with_density(l, 1.0).randomize();
                        // ACO
                        start_time = SystemTime::now();
                        (vec_aco, dist_aco) = match tsp::aco::tsp_aco_thread(&main_matrix) {
                            Some(tup) => tup,
                            None => (vec![0], 0)
                        };
                        end_time = SystemTime::now();
                        let duart = Some(SystemTime::duration_since(&end_time, start_time).unwrap());
                        io::store_test_data_in_file("output-meta-test.txt".to_string(), l, dist_aco, 1.0, vec_aco.clone(), duart, "ACO").unwrap();

                        // SA
                        start_time = SystemTime::now();
                        (vec_sa, dist_sa) = match tsp::sa::tso_sa(&main_matrix) {
                            Some(tup) => tup,
                            None => (vec![0], 0)
                        };
                        end_time = SystemTime::now();
                        let duart = Some(SystemTime::duration_since(&end_time, start_time).unwrap());
                        io::store_test_data_in_file("output-meta-test.txt".to_string(), l, dist_sa, 1.0, vec_sa.clone(), duart, "SA").unwrap();

                        // TS
                        start_time = SystemTime::now();
                        (vec_ts, dist_ts) = match tsp::tabu::tabu_tsp(&main_matrix) {
                            Some(tup) => tup,
                            None => (vec![0], 0)
                        };
                        end_time = SystemTime::now();
                        let duart = Some(SystemTime::duration_since(&end_time, start_time).unwrap());
                        io::store_test_data_in_file("output-meta-test.txt".to_string(), l, dist_ts, 1.0, vec_ts.clone(), duart, "TS").unwrap();

                        // Tests
                        // ACO
                        if !main_matrix.check_length(&vec_aco, dist_aco) || !main_matrix.check_cycle(&vec_aco) {
                            aco_err += 1;
                            println!("Aco vec: {:?}\nAco dist: {dist_aco}", vec_aco);
                        }
                        
                        // SA
                        if !main_matrix.check_length(&vec_sa, dist_sa) || !main_matrix.check_cycle(&vec_sa) {
                            sa_err += 1;
                            println!("Sa vec: {:?}\nSa dist: {dist_sa}", vec_sa);
                        }

                        // TS
                        if !main_matrix.check_length(&vec_ts, dist_ts) || !main_matrix.check_cycle(&vec_ts) {
                            ts_err += 1;
                            println!("Ts vec: {:?}\nTs dist: {dist_ts}", vec_ts);
                        }
                    }
                }
                println!("Errors:\nACO: {aco_err}\nSA: {sa_err}\nTS: {ts_err}");
            }

            22 => {
                if let Err(err) = io::clear_output_file("output-meta-test.txt".to_string()) {
                    println!("{err}");
                }
                if let Err(err) = io::clear_output_file("output-comp-test.txt".to_string()) {
                    println!("{err}");
                }
                if let Err(err) = io::clear_output_file("output_test1.txt".to_string()) {
                    println!("{err}");
                }
            }

            31 => {
                let mut thread_time = vec![0; TEST_DENS.len()];
                let mut normal_time = vec![0; TEST_DENS.len()];

                let mut start_time: SystemTime;
                let mut end_time: SystemTime;

                for i in 0..TEST_DENS.len() {
                    let d = TEST_DENS[i];
                    for it in 0..50 {
                        println!("Density: {d:.3}, iteration: {:1}", it + 1);
                        let m = Matrix::new_with_density(75, d).randomize();

                        start_time = SystemTime::now();
                        tsp::aco::tsp_aco(&m);
                        end_time = SystemTime::now();
                        let duart = SystemTime::duration_since(&end_time, start_time).unwrap();
                        normal_time[i] += duart.as_nanos();

                        start_time = SystemTime::now();
                        tsp::aco::tsp_aco_thread(&m);
                        end_time = SystemTime::now();
                        let duart = SystemTime::duration_since(&end_time, start_time).unwrap();
                        thread_time[i] += duart.as_nanos();
                    }
                }
                for i in 0.. TEST_DENS.len() {
                    println!("\nDensity: {:.3}", TEST_DENS[i]);
                    let normal_mean = normal_time[i] as f64 / 50 as f64;
                    let thread_mean = thread_time[i] as f64 / 50 as f64;
                    println!("Mean normal time: {:.4}\nMean parallel time: {:.4}\nRatio normal\\parallel: {:.4}", (normal_mean / 1_000_000_000.0),
                        (thread_mean / 1_000_000_000.0), normal_mean / thread_mean);
                }
            }

            _ => {
                break 'main;
            }
        };
    }
}
