use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use multimap::MultiMap;
use std::cmp::Ordering;

fn main() {
    println!("Hello, aoc_2024_5!");

    if let Ok(lines) = read_lines("./src/input.txt") {

        let mut updates = Vec::new();

        let mut ordering_rules : MultiMap<i64,i64> = MultiMap::new();

        //println!("Raw:");

        // Consumes the iterator, returns an ( Optional) String
        let mut reading_ordering_rules = true;
        for line in lines.flatten() {
            //println!("{}", line);

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

        println!("\nordering_rules multimap:");

        for (upper, lowers) in &ordering_rules {
            println!("upper {upper}: {:?}", lowers);
        }

        println!("\nupdates:");

        for update in &updates {
            println!("{:?}", update);
        }

        let mut count_good = 0;
        let mut sum_good_middles = 0;

        let mut count_adjusted = 0;
        let mut sum_adjusted_middles = 0;

        for mut update in updates {

            println!("\nSTART CHECKING update {:?}", update);

            let mut good = true;
            for outer_index in 0 .. update.len() {
                let val_at_outer = update[outer_index];
                let optional_prohibitions_for_output_page = ordering_rules.get_vec(&val_at_outer);
                if optional_prohibitions_for_output_page.is_some() {
                    let prohibitions_for_output_page = optional_prohibitions_for_output_page.unwrap();
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
                update.sort_by( | a, b| {
                    match ordering_rules.get_vec(&a) {
                        Some(rules) => {
                            if rules.contains(&b) {
                                return Ordering::Greater;
                            } else {
                                return Ordering::Equal;
                            }
                        },
                        None => return Ordering::Equal,
                    }
                    });

                count_adjusted += 1;
                let middle_val = update[update.len()/2];
                sum_adjusted_middles += middle_val;
                println!("\n===>ADJUSTED update {:?} middle_val {middle_val} sum_adjusted_middles {sum_adjusted_middles}", update);
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