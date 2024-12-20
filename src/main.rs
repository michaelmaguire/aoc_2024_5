use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use multimap::MultiMap;


fn main() {
    println!("Hello, aoc_2024_5!");

    if let Ok(lines) = read_lines("./src/input.txt") {

        let mut updates = Vec::new();

        let mut ordering_rules : MultiMap<i64,i64> = MultiMap::new();

        println!("Raw:");

        // Consumes the iterator, returns an ( Optional) String
        let mut reading_ordering_rules = true;
        for line in lines.flatten() {
            println!("{}", line);

            if line.is_empty() {
                reading_ordering_rules = false;
            } else {
                if reading_ordering_rules {
                    let ordering_rule_vec: Vec<i64> = line.split("|")
                    .map(|x| x.parse().expect("Not an integer!"))
                    .collect();

                    let lower = ordering_rule_vec[0];
                    let upper = ordering_rule_vec[1];

                    // We want to be able to look up violations of rules later.
                    ordering_rules.insert(upper, lower);

                } else {
                    let update_vec: Vec<i64> = line.split(",")
                    .map(|x| x.parse().expect("Not an integer!"))
                    .collect();

                    updates.push(update_vec);
                }
            }
        }

        println!("Processed:");

        for (upper, lowers) in &ordering_rules {
            print!("upper {upper}: ");
            for lower in lowers {
                print!("{lower} ");
            }
            println!();
        }

        let mut countGood = 0;
        let mut sumGoodMiddles = 0;

        for update in updates {

            let mut good = true;
            for outer_index in 0 .. update.len() {

                let page_at_outer = update[outer_index];

                let optional_prohibitions_for_output_page = ordering_rules.get_vec(&page_at_outer);
                if optional_prohibitions_for_output_page.is_some() {
                    let prohibitions_for_output_page = optional_prohibitions_for_output_page.unwrap();
                    print!("outer_index {outer_index} page_at_outer {page_at_outer} inner_index[");
                    for inner_index in outer_index+1 .. update.len() {
                        let page_at_inner = update[inner_index];
                        print!("inner_index {inner_index} page_at_inner{page_at_inner}");
                        if prohibitions_for_output_page.contains(&page_at_inner) {
                            good = false;
                        }
                    }
                    print!("] ");
                }
            }
            if good {
                countGood += 1;
                sumGoodMiddles += update[update.len()/2];
            }
            println!()
        }

        println!("done countGood {countGood} sumGoodMiddles {sumGoodMiddles}");

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