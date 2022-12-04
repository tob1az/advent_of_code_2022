mod data;

fn calculate_solution(calories_list: &str) -> (usize, usize) {
    let mut calories_per_elf = calories_list
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|food| food.parse::<usize>().unwrap_or_default())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();
    calories_per_elf.sort_unstable();
    let most_nutritious = *calories_per_elf.iter().last().unwrap();
    let top_3 = calories_per_elf.iter().rev().take(3).sum();
    (most_nutritious, top_3)
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::ELF_CALORIES_LIST));
}
