# TSP Algorithms
Repository used for university project about testing different TSP algorithms. The used parameter to check efficiency of each algorithm is the time of finding the shortest path.

## Already implemented algoritms:
* Brute-force algorithm
* Held-Karp dynamic algorithm
* Personal take on Held-Karp algorithm
* Ant Colony Optimization
* Simulated annealing
* Tabu search

## Dynamic algorithms
Repository has two takes on dynamic solution. One of them is the classic Held-Karp algorithm based on bitmasks to store optimal path. The second one uses an array of structs to store the permutations, target and the shortest path. The latter algorithm was made to check different method of implementation so the given solution is far from desired.

## ACO
Repository implements Parallel ACO and Standard ACO. Threads are used to calculate each ant's path to optimize findings. To move generated paths out of threads sync::mpsc::channel is being used and to share Path Matrix and Pheromone Matrix Arc<> structure is being used.