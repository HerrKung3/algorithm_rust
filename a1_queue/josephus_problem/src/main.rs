use josephus_problem::josephus_problem1;

fn main() {
    let names = vec!["Shieber", "David", "Susan", "Jane", "Kew", "Brad"];
    let left = josephus_problem1(names, 12);

    println!("the lucky guy is {}", left)
}
