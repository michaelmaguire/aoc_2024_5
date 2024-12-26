use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

#[derive(Default)]
struct AdjencyMatrix {
    matrix: Vec<Vec<bool>>,
}

impl fmt::Debug for AdjencyMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let height = self.matrix.len();
        let width = self.matrix[0].len();
        for i in 0..height {
            for j in 0..width {
                let val = self.matrix[i][j];
                if val {
                    write!(f, "1 ");
                } else {
                    write!(f, "0 ");
                }
            }
            writeln!(f);
        }
        Ok(())
    }
}

trait Directed {
    fn set_directed(&mut self, source: usize, destination: usize);
    fn clear_directed(&mut self, source: usize, destination: usize);
    fn get_directed(&self, source: usize, destination: usize) -> bool;
    fn get_sources_for(&self, destination: usize) -> Vec<usize>;
}

impl Directed for AdjencyMatrix {
    fn set_directed(&mut self, source: usize, destination: usize) {
        self.matrix[source][destination] = true;
    }
    fn clear_directed(&mut self, source: usize, destination: usize) {
        self.matrix[source][destination] = false;
    }
    fn get_directed(&self, source: usize, destination: usize) -> bool {
        return self.matrix[source][destination];
    }
    fn get_sources_for(&self, destination: usize) -> Vec<usize> {
        let mut sources = Vec::new();
        for source in 0 .. self.matrix.len() {
            if self.get_directed(source, destination) {
                sources.push(source);
            }
        }
        return sources; 
    }
}

impl AdjencyMatrix {
    fn new(size: usize) -> Self {
        let mut matrix = Vec::new();
        for _r in 1 .. size {
            matrix.push( vec![false; size] );
        }
        Self { matrix }
    }
}

// https://en.wikipedia.org/wiki/Topological_sorting
fn kahn_sort( adjacency_matrix : &AdjencyMatrix, nodes: &Vec<usize>) -> Vec<usize> {
    /*
        let mut l: Vec<i64> = Vec::new();
        let mut s: Vec<i64> = Vec::new();
        let mut directed_edges = original_directed_edges.clone();
        let mut directed_edges_reversed = original_directed_edges_reversed.clone();

        directed_edges.remove(&17);

        // Fill s with all nodes that have no incoming edge.
        for node in nodes {
            if ! directed_edges_reversed.contains_key(node) {
                s.push(*node);
            }
        }
        println!("s after populating: {:?}", s);

        while ! s.is_empty() {
            let item = s.remove(s.len()-1);
            l.push(item);
            
            let optional_directed_edges_reversed = directed_edges_reversed.get_vec(&item);
            if optional_directed_edges_reversed.is_some() {


            }
        }
    */
        return nodes.clone();
}



fn main() {
    println!("Hello, aoc_2024_5!");

    let mut adjacency_matrix = AdjencyMatrix::new(100);


    if let Ok(lines) = read_lines("./src/input.txt") {

        let mut updates = Vec::new();

        //println!("Raw:");

        // Consumes the iterator, returns an ( Optional) String
        let mut reading_ordering_rules = true;
        for line in lines.flatten() {
            //println!("{}", line);

            if line.is_empty() {
                reading_ordering_rules = false;
            } else {
                if reading_ordering_rules {
                    let ordering_rule_vec: Vec<usize> = line.split("|")
                    .map(|x| x.parse().expect("Not an integer!"))
                    .collect();

                    let lower = ordering_rule_vec[0];
                    let upper = ordering_rule_vec[1];

                    adjacency_matrix.set_directed(lower, upper);

                } else {
                    let update_vec: Vec<usize> = line.split(",")
                    .map(|x| x.parse().expect("Not an integer!"))
                    .collect();

                    updates.push(update_vec);
                }
            }
        }


        println!("\nadjacency_matrix: \n{:?}", adjacency_matrix);

        println!("\nupdates:");

        for update in &updates {
            println!("{:?}", update);
        }

        let mut count_good = 0;
        let mut sum_good_middles = 0;

        let mut count_adjusted = 0;
        let mut sum_adjusted_middles = 0;

        for update in updates {

            println!("\nSTART CHECKING update {:?}", update);

            let mut good = true;
            for outer_index in 0 .. update.len() {
                let val_at_outer = update[outer_index];
                let prohibitions_for_output_page = adjacency_matrix.get_sources_for(val_at_outer);
                if ! prohibitions_for_output_page.is_empty() {
                    println!("update {:?} outer_index {outer_index} val_at_outer {val_at_outer} PROHIBITIONS MUST BE AFTER {:?}", update, prohibitions_for_output_page);
                    let mut inner_index = outer_index +1;
                    while inner_index < update.len() {
                        let val_at_inner = update[inner_index];
                        println!("update {:?} outer_index {outer_index} val_at_outer {val_at_outer} inner_index {inner_index} val_at_inner {val_at_inner}", update);
                        if prohibitions_for_output_page.contains(&val_at_inner) {
                            good = false;
                            break;
                        } else {
                            inner_index += 1;
                        }
                    }
                } else {
                    println!("update {:?} outer_index {outer_index} val_at_outer {val_at_outer} NO PROHIBITIONS", update);
                }
            }

            if good {
                count_good += 1;
                let middle_val = update[update.len()/2];
                sum_good_middles += middle_val;
                println!("\n===>GOOD update {:?} middle_val {middle_val} sum_good_middles {sum_good_middles}", update);
            } else {
                let sorted = kahn_sort(&adjacency_matrix, &update);
                count_adjusted += 1;
                let middle_val = sorted[sorted.len()/2];
                sum_adjusted_middles += middle_val;
                println!("\n===>ADJUSTED update {:?} middle_val {middle_val} sum_adjusted_middles {sum_adjusted_middles}", sorted);
            }
            println!()
        }

        println!("done count_good {count_good} sum_good_middles {sum_good_middles} count_adjusted {count_adjusted} sum_adjusted_middles {sum_adjusted_middles}");

    } else {
        if let Ok(path) = env::current_dir() {
            println!("Error reading lines, the current directory is {}", path.display());
        } else {
            println!("Error reading lines, and can't print the current directory");

        }
    }
}

// Thanks to https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}