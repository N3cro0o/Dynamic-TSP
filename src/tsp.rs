pub mod naive;
pub mod dynamic;
pub mod aco;
pub mod tabu;
pub mod sa;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct TSP_Data{
    length: isize,
    target: usize,
    vertices: Vec<usize>,
    real_vertices: Vec<usize>
}

impl TSP_Data {
    pub fn is_the_same(&self, target: usize, vertices: Vec<usize>) -> bool {
        if target != self.target {
            return false;
        }
        if vertices.len() != self.vertices.len() {
            return false;
        }
        for v in 0..vertices.len(){
            if vertices[v] != self.vertices[v] {
                return false;
            }
        }
        true
    }
}

pub fn print_all_permutations(vec_size: usize) -> usize {
    let mut vec: Vec<usize> = vec![];
    let mut repeats = 0;
    for i in 1..vec_size {
        vec.push(i);
    }
    vec.push(0);
    while vec[0] < vec_size {
        println!("{vec:?}");
        vec = std_update_vector(vec, vec_size);
        repeats += 1;
    }
    repeats
}

fn std_update_vector(vec: Vec<usize>, vertices: usize) -> Vec<usize> {
    let mut vec = vec;
    let mut pos = 0;
    let mut add = 1;

    // Find change position
    'outer: for i in 3 .. vertices {
        //println!("{}",vertices - i);
        let val = vec[vertices - i];
        //println!("{}", val + add);
        while val + add < vertices {
            if !crate::matrix::check_if_exist_in_vec(&vec[0 .. vertices - i], val + add) {
                pos = vertices - i;
                break 'outer;
            }
            add += 1;
        }
        add = 1;
    }

    // Change pos index
    vec[pos] += add;

    // Small QOL addition, print start vertices
    if pos == 0 {println!("Update --> now check ver nr. {}", vec[pos]);}

    // Update vec
    for i in pos + 1 .. vertices - 1 {
        let mut min = 1;
        loop {
            let mut b = false;
            for j in 0 .. i {
                if vec[j] == min {b = true; break;}
            }
            if !b {break;}
            min += 1;
        }
        vec[i] = min;
    }
    vec
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_vec(){
        let mut vec = vec![1, 2, 3, 4, 0];
        vec = std_update_vector(vec, 5);
        assert_eq!([1, 2, 4, 3, 0], *vec);
    }

    #[test]
    fn test_vec_overflow(){
        let mut vec = vec![4, 3, 2, 1, 0];
        vec = std_update_vector(vec, 5);
        assert_eq!([5, 1, 2, 3, 0], *vec);
    }

    #[test]
    fn test_vec_all_4(){
        let mut vec = vec![1, 2, 3, 0];
        vec = std_update_vector(vec, 4);
        assert_eq!(vec, [1, 3, 2, 0]);
        vec = std_update_vector(vec, 4);
        assert_eq!(vec, [2, 1, 3, 0]);
        vec = std_update_vector(vec, 4); // ERROR!
        assert_eq!(vec, [2, 3, 1, 0]);
        vec = std_update_vector(vec, 4);
        assert_eq!(vec, [3, 1, 2, 0]);
        vec = std_update_vector(vec, 4);
        assert_eq!(vec, [3, 2, 1, 0]);
        vec = std_update_vector(vec, 4);
        assert_eq!(vec, [4, 1, 2, 0]);
    }

    #[test]
    fn test_vec_slice(){
        let vec = vec![2, 1, 2, 3, 4];
        assert_eq!(true, crate::matrix::check_if_exist_in_vec(&vec[0.. 2], 2))
    }
}