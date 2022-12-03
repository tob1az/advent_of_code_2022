mod data;

fn calculate_solution(calories_list: &str) -> usize {
    calories_list
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|food| food.parse::<usize>().unwrap_or_default())
                .sum::<usize>()
        })
        .max()
        .unwrap_or_default()
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::ELF_CALORIES_LIST));
}
