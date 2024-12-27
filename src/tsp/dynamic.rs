use crate::matrix::Matrix;
use super::TSP_Data;

pub fn tsp_dyn(m: &Matrix) -> Option<(Vec<usize>, usize)> { //--------------------------------------------------------------------------------------------
    let mut mem_vec: Vec<Vec<TSP_Data>> = vec![vec![]; m.vertices];
    let mut perm_vec: Vec<usize> = vec![];
    let mut answ_vec: Vec<usize> = vec![];
    let mut distance: isize = -1;

    // Setup vertices vec
    for i in 1..m.vertices {
        perm_vec.push(i);
    }

    for i in 1..m.vertices {
        println!("Now checing vert {i}");
        let id = tsp_dyn_main(m, i, perm_vec.clone(), &mut mem_vec);
        let id_dist = mem_vec[perm_vec.len() - 1][id].length;
        let next_dist = m.matrix[i][0];

        // Check if prev path and cycle exist
        if id_dist <= 0 { continue; }
        if next_dist <= 0 { continue; }

        let d = id_dist + next_dist;
        if d < distance || distance == -1 {
            distance = d;
            answ_vec = mem_vec[perm_vec.len() - 1][id].real_vertices.clone();
            answ_vec.push(i);
        }
    }

    answ_vec.push(0);
    if distance != -1 {
        Some((answ_vec, distance as usize))
    }
    else {
        None
    }
}

fn tsp_dyn_main(m: &Matrix, target: usize, perm_vec: Vec<usize>, mem_vec: &mut Vec<Vec<TSP_Data>>) -> usize {
    // Setup
    #[allow(unused_assignments)]
    let mut len = 0;
    let vec_size = perm_vec.len() - 1;
    let mut tsp_data = TSP_Data {
        target: target,
        vertices: perm_vec.clone(),
        real_vertices: vec![],
        length: -1
    };

    // Memorization
    for data in 0..mem_vec[vec_size].len() {
        if mem_vec[vec_size][data].is_the_same(target, perm_vec.clone()) {
            return data as usize;
        }
    }

    // Check if last
    if perm_vec.len() == 1{
        tsp_data.length = m.matrix[0][target];
        mem_vec[0].push(tsp_data);
        len = mem_vec[0].len() as usize - 1;
    }
    else {
        let mut distance = -1;

        // Create new vec
        let mut buff_vec = vec![];
        for j in perm_vec.iter() {
            // Skip chosen edge
            if *j == target {continue;}
            buff_vec.push(*j);
        }

        // Test
        for new_target in buff_vec.iter() {
            let id = tsp_dyn_main(m, *new_target, buff_vec.clone(), mem_vec);
            let id_dist = mem_vec[vec_size - 1][id].length;
            let next_dist = m.matrix[*new_target][target];

            if id_dist <= 0 { continue; } // If prev path doesn't exit, continue to the next iteration
            if next_dist <= 0 { continue; } // If prev path doesn't exit, continue to the next iteration

            let d = id_dist + next_dist;
            if d < distance || distance == -1 { // Check if found distance is shorter or stored distance is infinite
                distance = d;
                tsp_data.real_vertices = mem_vec[vec_size - 1][id].real_vertices.clone();
                tsp_data.real_vertices.push(*new_target);
            }
        }
        
        tsp_data.length = distance;
        mem_vec[vec_size].push(tsp_data);
        len = mem_vec[vec_size].len() - 1;
    }
    len
}


// https://github.com/williamfiset
pub fn tsp_dyn_new(m: &Matrix) -> Option<(Vec<usize>, usize)> { //---------------------------------------------------------------------------------------------------
    let size= m.vertices;
    let exp = 2_usize.checked_pow(size as u32).expect("Problem too big");
    let mut aux_mat: Vec<Vec<Option<isize>>> = vec![vec![None; exp]; size];
    
    // Store first distances; from ver 0 to each one
    for i in 1..size {
        let x = 1 << 0 | 1 << i;
        if m.matrix[0][i] <= 0 {
            aux_mat[i][x] = None;
        }
        else {
            aux_mat[i][x] = Some(m.matrix[0][i]);
        }
    }

    // Find the path
    for i in 3 ..= size {
        println!("----- {size}, {i}");
        for sets in dyn_perms(size, i) {
            // Checks if value x is part of the set (sets & (1 << x)) == 1, 
            // so (sets & (1 << x)) == 0 checks if x is not part of the set
            if (sets & (1 << 0)) == 0 {
                continue;
            }
            // Variable 'next' is used to remove one vertex already present in a set
            for next in 0..size {
                if next == 0 || (sets & (1 << next)) == 0 {
                    continue;
                }
                // Remove from the bit mask
                let mask = sets ^ (1 << next);
                let mut dist = -1;
                for end in 0..size {
                    if end == next || end == 0 || (sets & (1 << end)) == 0 {
                        continue;
                    }
                    // check if path exists
                    let path_prev = match aux_mat[end][mask] {
                        Some(x) => x,
                        None => {continue;}
                    };
                    let path_next = m.matrix[end][next];
                    if path_next <= 0 {continue;}

                    let d = path_prev + path_next;
                    if d < dist || dist == -1 {
                        dist = d;
                    }
                }
                if dist > 0 {
                    aux_mat[next][sets] = Some(dist);
                }
            }
        }
    }
    let finish = (1 << size) - 1;
    let mut min_dist = -1;

    // Cost, basically the same as in the struct method
    for i in 1..size {
        let aux_dist = match aux_mat[i][finish] {
            Some(x) => x,
            None => {continue;}
        };
        let m_dist = m.matrix[i][0];
        if m_dist <= 0 {continue;}

        let cost = aux_dist + m_dist;
        if cost < min_dist || min_dist == -1 {
            min_dist = cost;
        }
    }

    // Path vector
    let mut prev = 0;
    let mut state: usize = (1 << size) - 1;
    let mut path_vec = vec![0; size];

    for i in (1..size).rev() {
        let mut index = 0;

        for j in 1..size {
            if state & (1 << j) == 0 { continue; }
            if index == 0 {
                index = j;
            }
            // Check if path REALLY exist
            // Next distance
            let aux_dist = match aux_mat[j][state] {
                Some(x) => x,
                None => {continue;}
            };
            let m_dist = m.matrix[j][prev];
            if m_dist <= 0 {continue;}
            let new_d = aux_dist + m_dist;

            // Prev distance
            let aux_dist = aux_mat[index][state].unwrap_or(-1);
            let m_dist = m.matrix[index][prev];
            if aux_dist <= 0 || m_dist <= 0 { // If prev path doesn't exit, then save the "next distance"
                index = j;
                continue;
            }
            let prev_d = aux_dist + m_dist;

            if new_d < prev_d {
                index = j;
            }
        }

        // Save vertex and remove it form the mask
        path_vec[i - 1] = index;
        state ^= 1 << index;
        prev = index;
    }

    if min_dist < 0 {None}
    else {Some((path_vec, min_dist as usize))}
}

fn dyn_perms(size: usize, ones: usize) -> Vec<usize> {
    /* 
       TL;DR
       | - or
       & - and
       ^ - xor
       << - bit shift to the left (pow)
       >> - bit shift to the right (root)
     */
   let mut vec = vec![];
   // Smallest one:
   let mut target = (1 << ones) - 1;
   vec.push(target);

   'outer: loop {
       let mut p = 0;
       while p < size - 1 && ((target & (1 << (p + 1))) >> (p + 1)) >= ((target & (1 << (p))) >> (p)) {
           p += 1;
       }

       // Check if last
       if p == size - 1{
           break 'outer;
       }
       target |= 1 << (p + 1);

       for swap in 0.. (p + 1) {
           if (target & (1 << swap)) >> swap == 1 {
               target ^= 1 << swap;
               break;
           }
       }

       let mut end: usize = 0;
       while p > end {
           
           if ((target & (1 << end)) >> end) ^ ((target & (1 << p)) >> p) == 1 {
               target ^= 1 << end;
               target ^= 1 << p;
           }
           end += 1;
           p -= 1;
       }
       vec.push(target);
   }
   vec
}