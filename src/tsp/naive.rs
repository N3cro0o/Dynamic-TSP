use crate::matrix::Matrix;
use super::std_update_vector;
use factorial::Factorial;

pub fn tsp_standard(m: &Matrix) -> Option<(Vec<usize>, usize)> { //-----------------------------------------------------------
    // Variables
    let max_factorial:u128 = (m.vertices as u128 - 1 ).factorial();
    let mut permutation_nr: u128 = 0;
    let m_vert = m.vertices;
    let mut test_vec: Vec<usize> = vec![0; m_vert]; // Store current path
    let mut test_answ_check = false;
    
    // Output
    let mut answ_vec: Vec<usize> = vec![0; m_vert];
    let mut answ_dist = usize::MAX;
    let mut answ = None;
    // Setup test_vec
    for i in 0 .. m_vert - 1 {
        test_vec[i] = i + 1;
    }

    // LST standard test
    'lst: loop {
        // Print percent
        if permutation_nr % 100_000_000 == 0  {
            let percent = permutation_nr as f64 / max_factorial as f64 * 100.0;
            println!("Current -> {percent}% done");
        }

        // Test to end the 'lst loop
        if *test_vec.get(0).unwrap() == m_vert {break 'lst;}

        // Prep
        let mut test_dist: usize = 0;
        let mut test_last: usize = 0;

        // Read permutation
        permutation_nr += 1;
        for i in test_vec.iter() {
            // Check if path exsts. If not, get new permutation
            if m.matrix[test_last][*i] <= 0 {
                test_vec = std_update_vector(test_vec, m_vert);
                continue 'lst;
            }
            test_dist += m.matrix[test_last][*i] as usize;
            test_last = *i;
        }

        // Check answ
        if answ_dist > test_dist {
            test_answ_check = true;
            answ_dist = test_dist;
            answ_vec = test_vec.clone();
        }

        // Update permutation
        test_vec = std_update_vector(test_vec, m_vert);
    }
    if test_answ_check {
        answ = Some((answ_vec, answ_dist));
    }
    answ
}