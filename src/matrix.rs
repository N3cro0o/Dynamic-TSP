use rand::Rng;
use std::env;

use super::io;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub edges: usize,
    pub vertices: usize,
    pub matrix: Vec<Vec<isize>>
}

pub struct MatrixFloat {
    pub edges: usize,
    pub vertices: usize,
    pub matrix: Vec<Vec<f64>>
}

impl Default for Matrix {
    fn default() -> Self {
        let mut m = Matrix {
            edges: 20,
            vertices: 5,
            matrix: vec![vec![-1; 5]; 5]
        };
        m.main_diagonal_setup();
        m
    }
}

impl Default for MatrixFloat {
    fn default() -> Self {
        let mut m = MatrixFloat {
            edges: 20,
            vertices: 5,
            matrix: vec![vec![-1.0; 5]; 5]
        };
        m.main_diagonal_setup();
        m
    }
}

impl Matrix {
    fn main_diagonal_setup(&mut self) {
        for i in 0..self.vertices{
            self.matrix[i][i] = 0;
        }
    }

    // new functions
    pub fn new_with_density(vertices: usize, density: f32) -> Self {
        let edges: usize = f32::round((vertices * (vertices - 1)) as f32 * density) as usize;
        let mut m = Matrix {
            edges,
            vertices,
            matrix: vec![vec![-1; vertices]; vertices]
        };
        m.main_diagonal_setup();
        m
    }

    pub fn new_with_edges(vertices: usize, edges: usize) -> Self {
        let mut m = Matrix {
            vertices,
            edges,
            matrix: vec![vec![-1; vertices]; vertices]
        };
        m.main_diagonal_setup();
        m
    }

    pub fn new_with_constant(vertices: usize, constant: isize) -> Self {
        let mut m = Matrix {
            vertices,
            edges: vertices * (vertices - 1),
            matrix: vec![vec![constant; vertices]; vertices]
        };
        m.main_diagonal_setup();
        m
    }

    pub fn empty() -> Self{
        Matrix {
            vertices: 0,
            edges: 0,
            matrix: vec![]
        }
    }

    pub fn print_matrix(&self) {
        for i in self.matrix.iter() {
            for j in i.iter(){
                if *j > 9 {print!("{} | ", j);}
                else if *j >= 0 {print!("{}  | ", j);}
                else {print!("   | ");}
            }
            println!();
        }
    }

    pub fn push_edge(&mut self, start_vertex: usize, end_vertex: usize, edge: isize) {
        // Checks
        if start_vertex >= self.vertices || end_vertex >= self.vertices {
            panic!("One of the vertex indexes is too large. ---> {}", Matrix::push_edge_panic(start_vertex, end_vertex, edge))
        }
        if edge < 0 {
            panic!("The edge value is invalid. Must be equal or larger than 0. ---> {}", Matrix::push_edge_panic(start_vertex, end_vertex, edge))
        }
        self.edges += 1;
        self.matrix[start_vertex][end_vertex] = edge;
    }

    fn push_edge_panic(start_vertex: usize, end_vertex: usize, edge: isize) -> String {
        format!("|{start_vertex} -> {end_vertex}, {edge}|")
    }

    pub fn randomize(&mut self) -> Self {
        let env_var = env::var("PRINT_MATRIX").unwrap_or(String::new());
        let mut left_edges = self.edges;
        // Generate cycle
        {
            let mut perm_vec:Vec<usize> = vec![];
            let mut y = 0;
            for _i in 0..self.vertices - 1 {
                if left_edges == 0 { // check edges left
                    return self.clone();
                }
                let mut x = 0;
                while check_if_exist_in_vec(&perm_vec, x) || x == 0 {
                    x = rand::thread_rng().gen_range(0..self.vertices);
                }
                perm_vec.push(x);
                let d = rand::thread_rng().gen_range(1..100);
                self.matrix[y][x] = d;
                left_edges -= 1;
                y = x;
            }
            // Generate last cycle edge
            if left_edges == 0 { // check edges left
                return self.clone();
            }
            let d = rand::thread_rng().gen_range(1..100);
            self.matrix[y][0] = d;
            left_edges -= 1;
        }
        while left_edges != 0 {
            let ver1 = rand::thread_rng().gen_range(0..self.vertices);
            let ver2 = rand::thread_rng().gen_range(0..self.vertices);
            
            if ver1 == ver2 {continue;}

            let target = self.matrix.get_mut(ver1).unwrap();
            if target[ver2] < 0 {
                let d = rand::thread_rng().gen_range(1..100);
                target[ver2] = d;
                left_edges -= 1;
            }
        }
        if env_var.to_lowercase() == "true"{
            io::save_matrix_to_file(&self).unwrap();
        }
        self.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.vertices == 0
    }

    pub fn check_cycle(&self, vec: &Vec<usize>, dist: usize) -> bool {
        let mut d = 0;
        let mut last = 0;
        for i in vec.iter(){
            if self.matrix[last][*i] < 0 {return false}
            d += self.matrix[last][*i] as usize;
            last = *i;
        }
        dist == d
    }

    pub fn get_vertex_number_vec(&self) -> Vec<usize> {
        let mut v = vec![];
        for i in 0..self.vertices{
            v.push(i);
        }
        v
    }
}

impl MatrixFloat {
    fn main_diagonal_setup(&mut self) {
        for i in 0..self.vertices{
            self.matrix[i][i] = 0.0;
        }
    }

    pub fn new_with_constant(vertices: usize, constant: f64) -> Self {
        let mut m = MatrixFloat {
            vertices,
            edges: vertices * (vertices - 1),
            matrix: vec![vec![constant; vertices]; vertices]
        };
        m.main_diagonal_setup();
        m
    }

    pub fn print_matrix(&self) {
        for i in self.matrix.iter() {
            for j in i.iter(){
                if *j > 9.0 {print!("{} | ", j);}
                else if *j >= 0.0 {print!("{:.2}  | ", j);}
                else {print!("   | ");}
            }
            println!();
        }
    }

    pub fn multiply_const(&mut self, constant: f64) {
        for row in 0..self.vertices {
            for col in 0..self.vertices {
                self.matrix[row][col] *= constant;
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.vertices == 0
    }
}

pub fn check_if_exist_in_vec(vec: &[usize], val: usize) -> bool {
    for i in vec.iter(){
        if *i == val{
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod test {
    use super::*;

    #[should_panic]
    #[test]
    fn test_push_edge_vertex_panic() {
        let mut m = Matrix::default();
        m.push_edge(12, 2, 1);
    }

    #[test]
    fn test_push_edge_edge_panic() {
        let mut m = Matrix::default();
        m.push_edge(4, 2, 0);
    }

    #[test]
    fn test_matrixfloat() {
        let m = crate::matrix::Matrix::new_with_constant(5, 10);
        let p = crate::matrix::MatrixFloat::new_with_constant(5, 0.5);

        for i in 0..5 {
            for j in 0..5 {
                if i == j {continue;}
                let x = m.matrix[i][j] as f64 * p.matrix[i][j];
                println!("{x}");
                assert_eq!(x, 5.0);
            }
        }
    }
}