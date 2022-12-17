mod data;
mod filesystem;

fn deduce_directory_tree(terminal_output: &str) -> filesystem::TreeWalker {
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
    tree_walker
}

fn calculate_solution(terminal_output: &str) -> (usize, usize) {
    let tree_walker = deduce_directory_tree(terminal_output);
    let mut directory_sizes = vec![];
    tree_walker.traverse(&mut |n| {
        if n.is_file() {
            return;
        }
        directory_sizes.push(n.size());
    });

    let small_directories_size = directory_sizes
        .iter()
        .filter(|s| **s <= 100_000)
        .cloned()
        .sum();
    directory_sizes.sort();
    let used_space = directory_sizes
        .iter()
        .cloned()
        .max()
        .expect("There are 1+ directories");
    let disk_space = 70_000_000;
    let free_space = disk_space - used_space;
    let target_free_space = 30_000_000;
    debug_assert!(free_space < target_free_space);
    let bytes_to_free = target_free_space - free_space;
    let most_fitting_directory_size = directory_sizes
        .into_iter()
        .filter(|s| *s > bytes_to_free)
        .next()
        .expect("There is a big enough directory");
    (small_directories_size, most_fitting_directory_size)
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::TERMINAL_OUTPUT));
}
