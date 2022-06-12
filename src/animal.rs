extern crate rand;

use rand::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub enum Sex {
	Male,
	Female
}

impl Default for Sex {
	fn default() -> Self { Sex::Male }
}

#[derive(Default, Clone)]
pub struct Animal {
	pub sex: Sex,
	pub genes: Vec<char>,
	pub fitness: f32
}

impl Animal {
	pub fn create(&mut self, father: &Animal, mother: &Animal, mutation_rate: f32) {
		if father.sex == Sex::Male && mother.sex == Sex::Female {
			let mut rng = rand::thread_rng();
			let gene_length: u32 = father.genes.len() as u32;
			let mut i:u32 = 0;
			let mutate = rng.gen::<f32>();
			let s: u8 = rng.gen_range(0..2);
			match s {
				0 => self.sex = Sex::Male,
				1 => self.sex = Sex::Female,
				_ => self.sex = Sex::Male
			}
			while i < gene_length {
				let gene_origin: bool = rng.gen::<bool>();
				if gene_origin {
					self.genes.push(father.genes[i as usize].clone());
				} else {
					self.genes.push(mother.genes[i as usize].clone());
				}
				if mutate < mutation_rate {
					let c = rng.gen_range('\u{0000}'..='\u{FEFF}') as char;
					self.genes[i as usize] = c;
				}
				i += 1;
			}
		}
	}
}

/* impl PartialEq for Animal {
	fn eq(&self, other: &Self) -> bool {
		 self.isbn == other.isbn
	}
} */


pub fn new_animal(gene_length: u64, target: &String) -> Animal {
	let mut genes: Vec<char> = Vec::new();
	let mut rng = rand::thread_rng();
	let sex: Sex;
	let s: u8 = rng.gen_range(0..2);
	match s {
		0 => sex = Sex::Male,
		1 => sex = Sex::Female,
		_ => sex = Sex::Male
	}
	let mut fit = 0.0;
	let mut i = 0;
	while i < gene_length {
		//let c = rng.gen::<char>();
		let c = rng.gen_range('\u{0000}'..='\u{FEFF}') as char;
		if c == target.chars().nth(i as usize).unwrap_or_default() {
			fit += 1.0;
		}
		genes.push(c);
		i += 1;
	}
	let fitn: f32 = fit / (gene_length as f32);
	Animal {sex, genes, fitness:fitn}
}

/* pub fn print_animal(animal: &Animal) {
	let sex: &str = if animal.sex == Sex::Male {"Male"} else {"Female"};
	let s: String = animal.genes.iter().collect();
	println!("Animal with genes '{}' is {} and has a fitness of {}", s, sex, animal.fitness);
} */

pub fn print_fitest_animal(animals: &Vec<Animal>) {
	let mut fitest: Animal = Animal {sex:Sex::Male, genes:Vec::new(), fitness: 0.0};
	for animal in animals {
		if animal.fitness > fitest.fitness {
			fitest.fitness = animal.fitness;
			fitest.genes = animal.genes.clone();
			fitest.sex = animal.sex;
		}
	}
	let s: String = fitest.genes.iter().collect();
	let sex: &str = if fitest.sex == Sex::Male {"Male"} else {"Female"};
	println!("Animal with fittest genes is '{}', is {}, and has a fitness of {}", s, sex, fitest.fitness);
}

pub fn get_fitest_animal(animals: &Vec<Animal>) -> Animal {
	let mut fitest: Animal = Animal {sex:Sex::Male, genes:Vec::new(), fitness: 0.0};
	for animal in animals {
		if animal.fitness > fitest.fitness {
			fitest.fitness = animal.fitness;
			fitest.genes = animal.genes.clone();
			fitest.sex = animal.sex;
		}
	}
	let s: String = fitest.genes.iter().collect();
	let sex: &str = if fitest.sex == Sex::Male {"Male"} else {"Female"};
	println!("Animal with fittest genes is '{}', is {}, and has a fitness of {}", s, sex, fitest.fitness);
	return fitest
}

pub fn generate_mate_pool(animals: &Vec<Animal>) -> Vec<Animal> {
	let mut population:Vec<Animal> = Vec::new();
	for animal in animals {
		let n:u32 = (animal.fitness * 100.0) as u32;
		let mut i:u32 = 0;
		while {
			population.push(animal.clone());
			i += 1;
			i < n
		} {}
	}
	return population;
}

pub fn generate_next_generation(pool: &Vec<Animal>, population_size: u32, mutation_rate: f32) -> Vec<Animal> {
	let mut population:Vec<Animal> = Vec::new();
	let mut rng = rand::thread_rng();
	let mut i: u32 = 0;
	let pool_size: u32 = pool.len() as u32;
	while i < population_size {
		let mut father: u32 = rng.gen_range(0..pool_size);
		while pool[father as usize].sex != Sex::Male {
			father = rng.gen_range(0..pool_size);
		}
		let mut mother: u32 = rng.gen_range(0..pool_size);
		while pool[mother as usize].sex != Sex::Female {
			mother = rng.gen_range(0..pool_size);
		}
		let mut animal:Animal = Default::default();
		animal.create(&pool[father as usize], &pool[mother as usize], mutation_rate);
		population.push(animal);
		i += 1;
	}
	return population;
}

pub fn set_fitness(animals: &mut Vec<Animal>, target: &String) {
	for animal in animals {
		let mut fit = 0.0;
		let mut i = 0;
		let gene_length: u32 = target.len() as u32;
		while i < gene_length {
			if animal.genes[i as usize] == target.chars().nth(i as usize).unwrap_or_default() {
				fit += 1.0;
			}
			i += 1;
		}
		animal.fitness = fit / (gene_length as f32);
	}
}