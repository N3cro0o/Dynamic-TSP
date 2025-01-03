use crate::matrix::{Matrix, MatrixFloat};
use rand::Rng;

use::std::{thread, sync::{mpsc, Arc}};

pub fn tsp_aco(m: &Matrix) -> Option<(Vec<usize>, usize)> {
    let mut pheromone = MatrixFloat::new_with_constant(m.vertices, 0.5);
    let mut path = vec![];
    let mut len = usize::MAX;
    'outer: for i in 0..crate::ACO_ITERAT{
        if i % 75 == 0 {println!("ACO Iteration: {i}");}
        // Find for every ant path
        // Update pheromones
        // Multiply pheromones by the vanish constant
        // Get shortest path
        let mut ant_paths: Vec<Vec<usize>> = vec![];
        aco_ants(m, &pheromone, &mut ant_paths);
        pheromone.multiply_const(crate::ACO_PHERO_VANISH);
        aco_phero_path(m, &mut pheromone, &ant_paths);

        let mut cur_len = 0;
        let mut ant_id = 0;
        for i in 0..ant_paths.len() {
            let l = ant_paths[i].len();
            if ant_paths[i][l - 1] == 0 { // Find right cycle
                ant_id = i; // Save path id
                
                if let Some(x) = m.get_cycle_length(&ant_paths[i]) {
                    cur_len = x;
                }
                else { // If path is wrong ==> points to impossible path
                    continue 'outer;
                }
            }
        }
        if cur_len < len {
            len = cur_len;
            path = ant_paths[ant_id].clone();
        }

    }
    Some((path, len))
}

fn aco_ants(m: &Matrix, phero: &MatrixFloat, paths: &mut Vec<Vec<usize>>) {
    let mut glob_target_vertex = 0;
    let ants = (m.vertices as f64 * crate::ACO_ANTS).trunc() as usize;

    'ant: for _ in 0..ants {
        // 1. Generate desired values 
        // 2. Find probability
        // 3. Randomize path
        // 4. Remove target vertex from vector and save it
        // 5. Repeat 2. until all vertices used
        // 6. Store path and advance target vertex

        let mut vertices = m.get_vertex_number_vec();
        let mut target_vertex = glob_target_vertex;
        vertices.remove(target_vertex); // remove target vertex
        let mut d_values = vec![];
        let mut max_d_value: f64;
        let mut prob = vec![];
        let mut path: Vec<usize> = vec![];

        while vertices.len() != 0 {
            let mut ids = vec![];
            // Values
            d_values.clear();
            max_d_value = 0.0;
            // Find path
            let mut last_i = vertices[0];
            for i in vertices.iter() {
                if m.matrix[target_vertex][*i] > 0 {
                    last_i = *i;
                }
            }
            for i in vertices.iter(){
                let mut target = *i;
                let pheromone = phero.matrix[target_vertex][target].powf(crate::ACO_APLHA);

                let matrix_val;
                if m.matrix[target_vertex][target] > 0 {
                    matrix_val = m.matrix[target_vertex][target];
                    last_i = target;
                }
                else {    
                    matrix_val = m.matrix[target_vertex][last_i];
                    target = last_i;
                }

                ids.push(target);
                let path_len = (crate::ACO_PATH_LEN_DIV / matrix_val as f64).powf(crate::ACO_BETA); // DIVION IS IMPORTANTE!!!!!! Now it gets the smallest path possible
                let x = pheromone * path_len; 
                d_values.push(x);
                max_d_value += x;
            }
            // Probability
            prob.clear();
            for i in 0..d_values.len() {
                prob.push(d_values[i] / max_d_value);
            }
            // Randomize next vertex
            if prob.len() == 0 {continue 'ant;}
            let t = rand::thread_rng().gen();
            let mut id = 0;
            let mut sum = prob[0];
            while sum < t {
                id += 1;
                sum += prob[id];
            }
            // Path
            let mut pos = 0;
            for i in 0..vertices.len() {
                if vertices[i] == ids[id] {pos = i; break;}
            }
            id = vertices.remove(pos);
            path.push(id);
            target_vertex = id;
        }
        path.push(glob_target_vertex);
        // Store path
        if path.len() == m.vertices {
            paths.push(path);
        }
        glob_target_vertex += 1;
        if glob_target_vertex >= m.vertices {
            glob_target_vertex = 0;
        }
    }
}

pub fn tsp_aco_thread(m:&Matrix) -> Option<(Vec<usize>, usize)> {
    let mut pheromone = MatrixFloat::new_with_constant(m.vertices, 0.5);
    let mut path = vec![];
    let mut len = usize::MAX;
    'outer: for i in 0..crate::ACO_ITERAT{
        if i % 75 == 0 {println!("ACO-P Iteration: {i}");}
        // Find for every ant path
        // Update pheromones
        // Multiply pheromones by the vanish constant
        // Get shortest path
        let mut ant_paths: Vec<Vec<usize>> = vec![];
        aco_ants_thread(m, &pheromone, &mut ant_paths);
        pheromone.multiply_const(crate::ACO_PHERO_VANISH);
        aco_phero_path(m, &mut pheromone, &ant_paths);

        let mut cur_len = 0;
        let mut ant_id = 0;
        for i in 0..ant_paths.len() {
            let l = ant_paths[i].len();
            if ant_paths[i][l - 1] == 0 { // Find right cycle
                ant_id = i; // Save path id
                
                if let Some(x) = m.get_cycle_length(&ant_paths[i]) {
                    cur_len = x;
                }
                else { // If path is wrong ==> points to impossible path
                    continue 'outer;
                }
            }
        }
        if cur_len < len {
            len = cur_len;
            path = ant_paths[ant_id].clone();
        }

    }
    Some((path, len))
}

fn aco_ants_thread(m: &Matrix, phero: &MatrixFloat, paths: &mut Vec<Vec<usize>>) {
    let mut glob_target_vertex = 0;
    let ants = (m.vertices as f64 * crate::ACO_ANTS).trunc() as usize;
    let (tx, rx) = mpsc::channel();
    let mut ant_threads = vec![];
    let arc_m = Arc::new(m.clone());
    let arc_phero = Arc::new(phero.clone());

    for _ in 0..ants {
        // 1. Generate desired values 
        // 2. Find probability
        // 3. Randomize path
        // 4. Remove target vertex from vector and save it
        // 5. Repeat 2. until all vertices used
        // 6. Store path and advance target vertex
        let m_thread = Arc::clone(&arc_m);
        let phero_thread = Arc::clone(&arc_phero);
        let tx_thread = tx.clone();
        let mut vertices = m.get_vertex_number_vec();
        let mut target_vertex = glob_target_vertex;
        vertices.remove(target_vertex); // remove target vertex

        let thread_closure = move || {
            let mut d_values = vec![];
            let mut max_d_value: f64;
            let mut prob = vec![];
            let mut path: Vec<usize> = vec![];
            let end = target_vertex;

            while vertices.len() != 0 {
                let mut ids = vec![];
                // Values
                d_values.clear();
                max_d_value = 0.0;
                // Find path
                let mut last_i = vertices[0];
                for i in vertices.iter() {
                    if m_thread.matrix[target_vertex][*i] > 0 {
                        last_i = *i;
                    }
                }
                for i in vertices.iter(){
                    let mut target = *i;
                    let pheromone = phero_thread.matrix[target_vertex][target].powf(crate::ACO_APLHA);
    
                    let matrix_val;
                    if m_thread.matrix[target_vertex][target] > 0 {
                        matrix_val = m_thread.matrix[target_vertex][target];
                        last_i = target;
                    }
                    else {    
                        matrix_val = m_thread.matrix[target_vertex][last_i];
                        target = last_i;
                    }
    
                    ids.push(target);
                    let path_len = (crate::ACO_PATH_LEN_DIV / matrix_val as f64).powf(crate::ACO_BETA); // DIVION IS IMPORTANTE!!!!!! Now it gets the smallest path possible
                    let x = pheromone * path_len; 
                    d_values.push(x);
                    max_d_value += x;
                }
                // Probability
                prob.clear();
                for i in 0..d_values.len() {
                    prob.push(d_values[i] / max_d_value);
                }
                // Randomize next vertex
                if prob.len() == 0 {return}
                let t = rand::thread_rng().gen();
                let mut id = 0;
                let mut sum = prob[0];
                while sum < t {
                    id += 1;
                    sum += prob[id];
                }
                // Path
                let mut pos = 0;
                for i in 0..vertices.len() {
                    if vertices[i] == ids[id] {pos = i; break;}
                }
                id = vertices.remove(pos);
                path.push(id);
                target_vertex = id;
            }
            path.push(end);
            tx_thread.send(path).unwrap();
        };
        
        ant_threads.push(thread::spawn(thread_closure));

        glob_target_vertex += 1;
        if glob_target_vertex >= m.vertices {
            glob_target_vertex = 0;
        }
    }
    std::mem::drop(tx); // Drop original transmiter, we don't want an endless loop, amrite?
    for r in rx {
        // Store path
        if r.len() == m.vertices {
            paths.push(r);
        }
    }
}

fn aco_phero_path(m: &Matrix, phero: &mut MatrixFloat, paths: &Vec<Vec<usize>>) {
    for p in paths{
        // Len and scent
        let scent: f64;
        let mut len = 0;
        let mut first = p[p.len() - 1];
        for x in p.iter() {
            if m.matrix[first][*x] <= 0 {
                phero.matrix[first][*x] = 0.0;
                continue;
            }
            len += m.matrix[first][*x];
            first = *x;
        }
        scent = crate::ACO_Q / len as f64;
        // Add scent to paths
        for path in p.iter() {
            phero.matrix[first][*path] += scent;
            first = *path;
        }
    }
}