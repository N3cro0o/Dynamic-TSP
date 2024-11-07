use super::Matrix;
use std::{fs, io::{self, Write}};

/*
    TO DO
    4. create_matrix_form_file no path argument logic
*/

pub fn console_create_matrix_from_density() -> Matrix {
    println!("Vertices:");
    let ver = match console_read_usize(){
        Ok(x) => x,
        Err(_) => {
            return Matrix::default();
        }
    };
    println!("Density:");
    let dens = match console_read_f32(){
        Ok(x) => x,
        Err(_) => {
            return Matrix::default();
        }
    };   
    // Check if correct values
    if ver <= 0 {println!("Wrong vertices value. Must be larger than 0."); return Matrix::default();}
    if dens < 0.0 || dens > 1.0 {println!("Wrong density value. Must be between 0 and 1."); return Matrix::default();}
    Matrix::new_with_density(ver, dens)
}

pub fn create_matrix_from_file(path: Option<&str>) -> Result<Matrix, &'static str> {
    // Open file
    let file;
    if let None = path {
        return Err("File directory is empty");
    }
    let path = path.unwrap();
    dbg!(&path);
    file = open_file_to_string(path).unwrap();

    // Create matrix
    /*
        First line --> graph size, edges count
        Latter lines --> Start vertex, end vertex, value
     */
    let mut file_iter = file.lines();
    let vertices: usize;
    let edges: usize;
    // First line
    let first_line = file_iter.next().unwrap();
    (vertices, edges) = read_first_line(first_line);
    let mut matrix = Matrix::new_with_edges(vertices, 0);
    // Rest file
    let mut buff_string = String::new();
    for line in file_iter {
        // Setup
        buff_string.clear();
        let char_iter = line.chars();
        let mut act = false;
        let mut start_v = 0;
        let mut end_v = 0;

        // Get data from line
        for ch in char_iter {
            if ch == ' ' && !act {
                start_v = buff_string.trim().parse::<usize>().unwrap();
                buff_string.clear();
                act = true;
                continue;
            }
            else if ch == ' ' && act {
                end_v = buff_string.trim().parse::<usize>().unwrap();
                buff_string.clear();
                continue;
            }
            buff_string.push(ch);
        }
        let edge = buff_string.trim().parse::<isize>().unwrap();

        // Push edge
        matrix.push_edge(start_v, end_v, edge);
    }
    if edges == matrix.edges { 
        Ok(matrix)
    }
    else {
        Err("The number of edges is not the same")
    }
}

pub fn create_matrix_form_file_matrix(path: Option<&str>) -> Result<Matrix, &'static str> {
    // Open file
    let file;
    if let None = path {
        return Err("File directory is empty");
    }
    let path = path.unwrap();
    file = open_file_to_string(path).unwrap();

    // Create matrix
    /*
        First line --> graph size
        Latter lines --> Matrix
    */
    let mut file_iter = file.lines();
    let vertices: usize;

    // First line
    vertices = file_iter.next().unwrap().trim().parse::<usize>().unwrap();
    let mut m_out = Matrix::new_with_edges(vertices, 0);
    let mut x = 0;

    // Read matrix
    let mut buff_string = String::new();
    for line in file_iter {
        let mut y = 0;
        for ch in line.chars() {
            if ch != ' '{
                buff_string.push(ch);
                continue;
            }
            if !buff_string.is_empty() {
                if x != y {
                    let val = buff_string.trim().parse().unwrap();
                    m_out.push_edge(x, y, val);
                }
                y += 1;
                buff_string.clear();
            }
        }
        // The last edge cannot be added in inner loop, so we do this here. But its [x, y] x == y, so just push 0
        m_out.push_edge(x, y, 0);
        buff_string.clear();
        x += 1;
    }
    Ok(m_out)
}

pub fn save_matrix_to_file(matrix: &Matrix) -> Result<(), io::Error> {
    let mut file = fs::File::create("matrix_output.txt")?;
    write!(&mut file, "{} {}\n", matrix.vertices, matrix.edges)?;
    for i in 0..matrix.vertices {
        for j in 0..matrix.vertices {
            let dist = matrix.matrix[i][j];
            if dist > 0 {
                write!(&mut file, "{i} {j} {dist}\n")?;
            }
        }
    }

    Ok(())
}

fn open_file_to_string(path: &str) -> Result<String, &'static str> {
    let file = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(err) => {
            println!("{}", err.to_string());
            return Err("Missing files");
        }
    };
    Ok(file)
}

fn read_first_line(first_line: &str) -> (usize, usize) {
    let mut vertices: usize = 0;
    let edges; // We don't have to say the type, compilator knows that output has to be 'usize' so it fills the gap
    let mut buff_string =String::new();
    for ch in first_line.to_string().chars(){
        if ch == ' ' {
            vertices = buff_string.trim().parse().unwrap();
            buff_string.clear();
        }
        buff_string.push(ch);
    }
    edges = buff_string.trim().parse().unwrap();
    (vertices, edges)
}

fn console_read_usize() -> Result<usize, &'static str> {
    let mut str = String::new();
    
    if let Err(err) = std::io::stdin().read_line(&mut str){
        println!("{err}");
        return Err("Wrong input");
    }
    let x: usize = match str.trim().parse() {
        Ok(x) => x,
        Err(err) => {
            println!("{err}");
            return Err("Wrong input");
        }
    };
    Ok(x)
}

fn console_read_f32() -> Result<f32, &'static str> {
    let mut str = String::new();
    
    if let Err(err) = std::io::stdin().read_line(&mut str){
        println!("{err}");
        return Err("Wrong input");
    }
    let x: f32 = match str.trim().parse() {
        Ok(x) => x,
        Err(err) => {
            println!("{err}");
            return Err("Wrong input");
        }
    };
    Ok(x)
}

pub fn clear_output_file() -> Result<(), io::Error> {
    fs::remove_file("output.txt")?;
    Ok(())
}

pub fn store_test_data_in_file(num: usize, dist: usize, vec: Vec<usize>, time: Option<std::time::Duration>, method: &str) -> Result<(), io::Error> {
    let mut file = fs::OpenOptions::new().create(true).write(true).append(true).open("output.txt").unwrap();
    let real_time = match time{
        Some(x) => x.as_nanos(),
        None => 0
    };
    file.write(format!("{};{};{};{:?};{}\n", num, method, real_time, vec, dist).as_bytes())?;
    //write!(&mut file, "{};{};{};{:?};{}\n", num, method, real_time, vec, dist)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_matrix() {
        let x = create_matrix_from_file(Some("matrix.txt"));
        let y = x.unwrap();
        y.print_matrix();
    }

    #[test]
    fn test_read_first_line() {
        let file = open_file_to_string("matrix.txt").unwrap();
        let first_line = file.lines().next().unwrap();
        assert_eq!((21, 3), read_first_line(first_line))
    }

    /*#[test]
    fn test_write_file(){
        let m = Matrix::default().randomize();
        save_matrix_to_file(&m).unwrap();
    }*/
}