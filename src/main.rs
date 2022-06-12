/*
Rewritten from Processing example:
    http://natureofcode.com/book/chapter-9-the-evolution-of-code/	
*/

mod animal;

fn main() {
    //Setup
    let mut is_goal_reached = false;
    let mut current_generation:u64 = 0;
    let mut target = String::new();
    let mut pop_input = String::new();
    let mut mutation_input = String::new();
    let mut population:Vec<animal::Animal> = Vec::new(); 

    println!("Input target string:");
    std::io::stdin().read_line(&mut target).expect("Error: Failed to read line...");
    
    println!("Input pop start size (u32):");
    std::io::stdin().read_line(&mut pop_input).expect("Error: Failed to read line...");

    println!("Input mutation rate (f32: 0.01):");
    std::io::stdin().read_line(&mut mutation_input).expect("Error: Failed to read line...");
    target = target.trim().to_string();
    let gene_length = target.len() as u64;
    let pop_size: u32 = pop_input.trim().parse().unwrap();
    let mutation_rate: f32 = mutation_input.trim().parse().unwrap();
    println!(
        "{} pops with {} genomes will target '{}' with a mutation rate of {}", 
        pop_size, 
        gene_length, 
        target,
        mutation_rate
    );

    //Initiate population
    for _ in 1..=pop_size {
        population.push(animal::new_animal(gene_length, &target));
    }

    //TEST
    /* for an in &population {
        animal::print_animal(an);
    } */
    
    animal::print_fitest_animal(&population);

    while !is_goal_reached {
        current_generation += 1;
        println!("Gneration {}:", current_generation);
        let mate_pool: Vec<animal::Animal> = animal::generate_mate_pool(&population);

        population.clear();

        population = animal::generate_next_generation(&mate_pool, pop_size, mutation_rate);
        
        animal::set_fitness(&mut population, &target);

        let fitest_animal = animal::get_fitest_animal(&population);
        
        if fitest_animal.fitness >= 1.0 {
            is_goal_reached = true;
        }
    }
}
