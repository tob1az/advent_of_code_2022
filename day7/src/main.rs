mod data;
mod filesystem;

fn calculate_solution(terminal_output: &str) -> usize {
    let mut tree_walker = filesystem::TreeWalker::default();
    let mut file_sizes = vec![];
    for line in terminal_output.lines() {
        println!("{line}");
        let (first, second) = line
            .split_once(' ')
            .expect("Each line has at least 2 words");
        match first {
            "$" => {
                if !file_sizes.is_empty() {
                    tree_walker.populate_current_directory(&file_sizes);
                    file_sizes.clear();
                }
                if second.starts_with("cd ") {
                    let (_, directory) =
                        second.split_once(' ').expect("cd command has an argument");
                    tree_walker.cd(directory);
                }
            }
            "dir" => file_sizes.push((second.to_owned(), 0)),
            _ => file_sizes.push((
                second.to_owned(),
                first.parse::<usize>().expect("File size is a number"),
            )),
        }
    }
    if !file_sizes.is_empty() {
        tree_walker.populate_current_directory(&file_sizes);
    }

    let mut sum = 0;
    tree_walker.traverse(&mut |n| {
        if n.is_file() {
            return;
        }
        let size = n.size();
        if size <= 100000 {
            sum += size;
        }
    });
    sum
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::TERMINAL_OUTPUT));
}
