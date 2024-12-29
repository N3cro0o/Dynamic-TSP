use crate::matrix::{Matrix, MatrixFloat};
use rand::Rng;

pub fn tsp_aco(m: &Matrix) -> Option<(Vec<usize>, usize)> {
    let mut pheromone = MatrixFloat::new_with_constant(m.vertices, 0.5);
    let mut path = vec![];
    let mut len = 0;
    for i in 0..crate::ACO_ITERAT{
        if i % 20 == 0 {println!("ACO Iteration {i}");}
        // Find for every ant path
        // Update pheromones
        // Multiply pheromones by the vanish constant
        let mut ant_paths: Vec<Vec<usize>> = vec![];
        aco_ants(m, &pheromone, &mut ant_paths);
        pheromone.multiply_const(crate::ACO_PHERO_VANISH);
        aco_phero_path(m, &mut pheromone, &ant_paths);
    }

    // select path
    let mut first = 0;
    let mut vert = m.get_vertex_number_vec();
    vert.remove(0);

    while vert.len() != 0 {
        let mut min = 0;
        for i in 0..vert.len() {
            if pheromone.matrix[first][vert[i]] > pheromone.matrix[first][vert[min]] {
                min = i;
            }
        }
        min = vert.remove(min);
        len += m.matrix[first][min] as usize;
        path.push(min);
        first = min;
    }
    len += m.matrix[first][0] as usize;
    path.push(0);
    Some((path, len))
}

fn aco_ants(m: &Matrix, phero: &MatrixFloat, paths: &mut Vec<Vec<usize>>) {
    let mut glob_target_vertex = 0;
    let ants = (m.vertices as f64 * crate::ACO_ANTS).trunc() as usize;

    for _ in 0..ants {
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
            // Values
            d_values.clear();
            max_d_value = 0.0;
            for i in vertices.iter(){
                let pheromone = phero.matrix[target_vertex][*i].powf(crate::ACO_APLHA);
                let path_len = (crate::ACO_PATH_LEN_DIV / m.matrix[target_vertex][*i] as f64).powf(crate::ACO_BETA); // DIVION IS IMPORTANTE!!!!!! Now it gets the smallest path possible
                let x = pheromone * path_len; 
                d_values.push(x);
                max_d_value += x;
            }
            // Probability
            prob.clear();
            for i in 0..vertices.len() {
                prob.push(d_values[i] / max_d_value);
            }
            // Randomize next vertex
            let t = rand::thread_rng().gen();
            let mut id = 0;
            let mut sum = prob[0];
            while sum < t {
                id += 1;
                sum += prob[id];
            }
            // Path
            id = vertices.remove(id);
            path.push(id);
            target_vertex = id;
        }
        path.push(glob_target_vertex);
        // Store path
        paths.push(path);
        glob_target_vertex += 1;
        if glob_target_vertex >= m.vertices {
            glob_target_vertex = 0;
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