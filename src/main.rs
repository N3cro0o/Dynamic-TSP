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

    'main: loop {
        // Setup
        let mut input_buff = String::new();
        let input_num;

        // Menu text
        {
            println!("--------------------------------------------------------------------------");
            println!("Witaj urzytkowniku! Proszę wybrać opcję komiwojażer!");
            println!("0. Druk macierzy\n1. Generacja macierzy wykorzystując gęstość.\n2. Wczytanie macierzy z pliku.\n3. Przeprowadzenie podstawowego testu.");
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
                println!("Proszę podać liczbę naturalną");
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

            11 => {
                println!("Proszę podać ścieżkę do pliku z macierzą. Ścieszka może myć lokalna albo globalna.");
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
                println!("Proszę podać ścieżkę do pliku z macierzą. Ścieszka może myć lokalna albo globalna.");
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
