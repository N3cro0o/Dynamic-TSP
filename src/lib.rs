//https://github.com/N3cro0o/Dynamic-TSP
pub mod tsp;
pub mod io;
pub mod matrix;

const ACO_ANTS: f64 = 1.0;
const ACO_PATH_LEN_DIV: f64 = 10.0;
const ACO_ITERAT: isize = 200;
const ACO_PHERO_VANISH: f64 = 0.7;
const ACO_APLHA: f64 = 0.8;
const ACO_BETA: f64 = 1.2;
const ACO_Q: f64 = 5.0;