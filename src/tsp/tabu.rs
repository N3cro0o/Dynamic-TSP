use crate::matrix::Matrix;
use rand::{seq::SliceRandom, thread_rng};

pub fn tabu_tsp(m: &Matrix) -> Option<(Vec<usize>, usize)> {
    let mut len;
    let mut min_len;
    let mut rng = thread_rng();

    let mut path = m.get_vertex_number_vec();
    path.remove(0);
    path.shuffle(&mut rng);
    path.push(0);
    let mut min_path;

    let mut tabu_vec: Vec<Vec<usize>> = vec![vec![0; m.vertices]; m.vertices];
    let mut strike = 0;
    len = match m.get_cycle_length(&path) {
        Some(x) => x,
        None=> usize::MAX
    };
    min_path = path.clone();
    min_len = len;

    for i in 0..crate::TABU_ITERAT {
        if i % 75 == 0 {println!("Tabu search Iteration {i}");}

        let start_len = len;
        tabu_paths(m, &mut tabu_vec, &mut path);
        len = match m.get_cycle_length(&path) {
            Some(x) => x,
            None=> usize::MAX
        };

        if len < min_len {
            min_len = len;
            min_path = path.clone();
        }

        // Decrease short-term tabu list
        for i in 0..m.vertices {
            for j in 0..m.vertices {
                if tabu_vec[i][j] > 0 {
                    tabu_vec[i][j] -= 1;
                }
            }
        }

        // If cannot find better path
        if start_len >= len {
            strike += 1;
            if strike == crate::TABU_MAXSTRIKES {
                path.clear();
                path = m.get_vertex_number_vec();
                path.remove(0);

                path.shuffle(&mut rng);
                path.push(0);
                
                tabu_vec.clear();
                tabu_vec = vec![vec![0; m.vertices]; m.vertices];
            }
        }
        else {strike = 0;}
    } 
    if min_len == usize::MAX {None}
    else {Some((min_path, min_len as usize))}
}

fn tabu_paths(m: &Matrix, tabu: &mut Vec<Vec<usize>>, path: &mut Vec<usize>) {
    let mut cur_path;
    let mut min_path = path.clone();
    let mut cur_len;
    let mut min_len = match m.get_cycle_length(&min_path) {
        Some(x) =>x,
        None => usize::MAX
    };
    let mut vertex_x = 0;
    let mut vertex_y = 0;

    // Checking 0 is unnecessary, the last edge is always checked and last vertex must be 0
    for x in 1 .. m.vertices {
        for y in 1 .. m.vertices {
            if x == y {continue;}

            if tabu[x][y] == 0 {
                cur_path = path.clone();
                
                let mut vertex_x1 = 0;
                let mut vertex_y1 = 0;

                for vert in 0..path.len() - 1{
                    if path[vert] == x {vertex_x1 = vert;}
                    if path[vert] == y {vertex_y1 = vert;}
                }
                cur_path[vertex_x1] = y;
                cur_path[vertex_y1] = x;
                // Add path checking
                cur_len = match m.get_cycle_length(&cur_path) {
                    Some(x) =>x,
                    None => usize::MAX
                };

                if cur_len < min_len {
                    min_len = cur_len;
                    min_path = cur_path;
                    vertex_x = x;
                    vertex_y = y;
                }
            }
        }
    }
    tabu[vertex_x][vertex_y] = crate::TABU_LIFETIME;
    path.copy_from_slice(&min_path);
}