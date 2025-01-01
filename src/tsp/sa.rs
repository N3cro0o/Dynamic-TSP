//SIMULATED ANNEALING BABYYYYYY
use rand::Rng;
use crate::matrix::Matrix;
use std::cmp::Ordering;

pub fn tso_sa(m: &Matrix) -> Option<(Vec<usize>, usize)> {
    let mut len;

    let mut path = m.get_vertex_number_vec();
    path.remove(0);
    path.push(0);

    let mut temp = crate::SA_TEMPERATURE * m.vertices as f64;
    let mut same_sollution = 0;
    let mut same_len = 0;
    len = m.get_cycle_length(&path).unwrap();

    while same_len < crate::SA_SAME_LENGTH && same_sollution < crate::SA_SAME_SOLLUTION {
        // progress
        if same_len != 0 && same_len % (crate::SA_SAME_LENGTH / 4) == 0 {
            println!("Same length check: {same_len}");
        }

        if same_sollution != 0 && same_sollution % (crate::SA_SAME_SOLLUTION / 4) == 0 {
            println!("Same sollution check: {same_sollution}");
        }
        let cur_len;
        let cur_path;
        cur_path = swap_random_cities(&path);
        cur_len = m.get_cycle_length(&cur_path).unwrap();
        
        match cur_len.cmp(&len) {
            Ordering::Less => {
                len = cur_len;
                path = cur_path;
                same_len = 0;
                same_sollution = 0;
            }
            Ordering::Equal => {
                len = cur_len;
                path = cur_path;
                same_len += 1;
                same_sollution = 0;
            }
            Ordering::Greater => {
                let p: f64 = rand::thread_rng().gen();
                let cost:f64 = ((cur_len as isize - len as isize) as f64 / temp).exp();
                if p >= cost {
                    len = cur_len;
                    path = cur_path;
                    same_len = 0;
                    same_sollution = 0;
                }
                else {
                    same_len += 1;
                    same_sollution += 1;
                }
            }
        }

        temp *= crate::SA_TEMPERATURE_DROP;
    }

    Some((path, len))
}

fn swap_random_cities(path: &Vec<usize>) -> Vec<usize>{
    let a = rand::thread_rng().gen_range(0 .. (path.len() - 1));
    let b = rand::thread_rng().gen_range(0 .. (path.len() - 1));
    let mut new_path = path.clone();

    new_path[a] = path[b];
    new_path[b] = path[a];

    return new_path;
}