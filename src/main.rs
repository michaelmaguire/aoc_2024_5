use std::cmp::Ordering;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::HashSet;

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
    fn get_destinations_for(&self, source: usize) -> Vec<usize>;
    fn get_sources_for(&self, destination: usize) -> Vec<usize>;
    fn has_sources(&self, destination: usize) -> bool ;
    fn get_destinations_without_sources(&self) -> Vec<usize>;
    fn get_all_nodes(&self) -> Vec<usize>;
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
    fn get_destinations_for(&self, source: usize) -> Vec<usize> {
        let mut destinations = Vec::new();
        for destination in 0 .. self.matrix.len() {
            if self.get_directed(source, destination) {
                destinations.push(destination);
            }
        }
        return destinations; 
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
    fn has_sources(&self, destination: usize) -> bool {
        for source in 0 .. self.matrix.len() {
            if self.get_directed(source, destination) {
                return true;
            }
        }
        return false;
    }
    fn get_destinations_without_sources(&self) -> Vec<usize> {
        let mut destinations_without_sources = Vec::new();
        for destination in 0.. self.matrix.len() {
            if ! self.has_sources(destination) {
                destinations_without_sources.push(destination);
            }
        }
        return destinations_without_sources;
    }
    fn get_all_nodes(&self) -> Vec<usize> {
        let mut set = HashSet::new();
        for i in 0..self.matrix.len() {
            for j in 0 .. self.matrix.len() {
                if self.matrix[i][j] {
                    set.insert(i);
                    set.insert(j);
                }
            }
        }
        return Vec::from_iter(set);
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

impl Clone for AdjencyMatrix {
    fn clone(&self) -> Self {
        let mut matrix = Vec::new();
        for r in 1 .. self.matrix.len() {
            matrix.push( self.matrix[r].clone() );
        }
        Self { matrix }
    }    
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

        let compare = | x: &usize, y:&usize | {
            let (x,y) = (*x, *y);
            if adjacency_matrix.get_directed(x, y) {
                Ordering::Less
            } else if adjacency_matrix.get_directed(y, x) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        };
                

        for update in updates {

            //println!("\nSTART CHECKING update {:?}", update);

            let mut good = true;
            for outer_index in 0 .. update.len() {
                let val_at_outer = update[outer_index];
                let prohibitions_for_output_page = adjacency_matrix.get_sources_for(val_at_outer);
                if ! prohibitions_for_output_page.is_empty() {
                    //println!("update {:?} outer_index {outer_index} val_at_outer {val_at_outer} PROHIBITIONS MUST BE AFTER {:?}", update, prohibitions_for_output_page);
                    let mut inner_index = outer_index +1;
                    while inner_index < update.len() {
                        let val_at_inner = update[inner_index];
                        //println!("update {:?} outer_index {outer_index} val_at_outer {val_at_outer} inner_index {inner_index} val_at_inner {val_at_inner}", update);
                        if prohibitions_for_output_page.contains(&val_at_inner) {
                            good = false;
                            break;
                        } else {
                            inner_index += 1;
                        }
                    }
                } else {
                    //println!("update {:?} outer_index {outer_index} val_at_outer {val_at_outer} NO PROHIBITIONS", update);
                }
            }

            if good {
                count_good += 1;
                let middle_val = update[update.len()/2];
                sum_good_middles += middle_val;
                //println!("\n===>GOOD update {:?} middle_val {middle_val} sum_good_middles {sum_good_middles}", update);
            } else {
                count_adjusted += 1;
                println!("ADJUSTED update before sort_by {:?}", update);
                let mut sorted_update = update.clone();
                sorted_update.sort_by(compare);
                let middle_val = sorted_update[sorted_update.len()/2];
                sum_adjusted_middles += middle_val;
                println!("ADJUSTED sorted_update         {:?} middle_val {middle_val} sum_adjusted_middles {sum_adjusted_middles}", sorted_update);
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