//https://github.com/N3cro0o/Dynamic-TSP
pub mod tsp;
pub mod io;
pub mod matrix;

const ACO_ANTS: f64 = 1.0;
const ACO_ITERAT: isize = 350;
const ACO_PHERO_VANISH: f64 = 0.65;
const ACO_APLHA: f64 = 0.8;
const ACO_BETA: f64 = 1.2;
const ACO_Q: f64 = 5.0;

const TABU_ITERAT: isize = 1_000;
const TABU_LIFETIME: usize = 50;
const TABU_MAXSTRIKES: u32 = 5;

const SA_TEMPERATURE: f64 = 250.0;
const SA_TEMPERATURE_DROP: f64 = 0.999;
const SA_SAME_LENGTH: u128 = 1_500_000;
const SA_SAME_SOLLUTION:u128 = 175_000;