extern crate rand;

use rand::random;
use rand::{thread_rng, Rng};

// Using boolean in place of a bit.
// true = 1 ; false = 0
type Individual = Vec<bool>;
type Population = Vec<Individual>;
type FitnessFunction = fn(&Individual) -> usize;
type PopulationFitness = Vec<usize>;

fn random_individual(size: usize) -> Individual {
    let mut individual = Vec::with_capacity(size);
    for _ in 0..size {
        individual.push(random());
    }
    individual
}

fn random_population(pop_size: usize, chromosome_length: usize) -> Population {
    let mut pop = Vec::with_capacity(pop_size);
    for _ in 0..pop_size {
        pop.push(random_individual(chromosome_length));
    }
    pop
}

fn evaluate(population: &Population, f: FitnessFunction) -> PopulationFitness {
    population.into_iter().map(|i| f(i)).collect()
}

fn show_population(run_num: usize, gen_num: usize, pop: &Population, fitness: &Vec<usize>) {
    let pop_size = pop.len();
    let avg_fitness: f32 = fitness.iter().sum::<usize>() as f32 / pop_size as f32;
    for i in 0..pop_size {
        println!(
            "{},{},{},{},{},{}",
            run_num,
            gen_num,
            i,
            avg_fitness,
            fitness[i],
            pop[i]
                .iter()
                .map(|b| if *b { "1" } else { "0" })
                .collect::<String>()
        );
    }
}

fn evolve(
    pop: &Population,
    fitness: &PopulationFitness,
    crossover_rate: f32,
    mutation_rate: f32,
) -> Population {
    let pop_size = pop.len();
    let mut next_pop = Vec::with_capacity(pop_size);

    let num_pairs = pop_size / 2; // TODO: handle odd pop_size
    for _ in 0..num_pairs {
        let (mum, dad) = select(&pop, &fitness);
        let (mut child1, mut child2) = if random::<f32>() <= crossover_rate {
            crossover(mum, dad)
        } else {
            (mum.clone(), dad.clone())
        };

        mutate(&mut child1, mutation_rate);
        mutate(&mut child2, mutation_rate);

        next_pop.push(child1);
        next_pop.push(child2);
    }
    next_pop
}

fn mutate(ind: &mut Individual, rate: f32) {
    let mut rng = thread_rng();
    for i in 0..ind.len() {
        if rng.gen::<f32>() <= rate {
            ind[i] = !ind[i];
        }
    }
}

fn crossover(mum: &Individual, dad: &Individual) -> (Individual, Individual) {
    let mut rng = thread_rng();
    let len = mum.len();
    let crossover_from = rng.gen_range(0, len);

    let mut child1: Individual = Vec::with_capacity(len);
    let mut child2: Individual = Vec::with_capacity(len);

    for i in 0..len {
        if i < crossover_from {
            child1.push(mum[i]);
            child2.push(dad[i]);
        } else {
            child1.push(dad[i]);
            child2.push(mum[i]);
        }
    }

    (child1, child2)
}

fn select<'p>(
    population: &'p Population,
    fitness: &PopulationFitness,
) -> (&'p Individual, &'p Individual) {
    // roulette-wheel sampling:
    let total_fitness: usize = fitness.iter().sum();
    let mut rng = thread_rng();
    // TODO: is it ok for i1 to be the same as i2? No, because cross over woud be a no-op
    let i1 = spin_index(fitness, rng.gen_range(0, total_fitness));
    let i2 = spin_index(fitness, rng.gen_range(0, total_fitness));
    (&population[i1], &population[i2])
}

fn spin_index(fitness: &PopulationFitness, max: usize) -> usize {
    let mut total = 0;
    for (i, v) in fitness.iter().enumerate() {
        total += v;
        if total >= max {
            return i;
        }
    }
    // can't reach this?
    fitness.len()
}

fn main() {
    let pop_size = 100;
    let crossover_rate = 0.7;
    let mutation_rate = 0.001;
    let chromosome_length = 20;

    fn fitness(i: &Individual) -> usize {
        i.iter().filter(|&b| b == &true).count()
    }

    let solved = |fitness: &PopulationFitness| fitness.iter().max() == Some(&chromosome_length);

    let verbose = false;

    if verbose {
        println!("run,gen,individual,average fitness,individual fitness, indivdial");
    } else {
        println!("run,generation");
    };

    for run in 1..21 {
        let mut gen = 0;
        let mut population = random_population(pop_size, chromosome_length);
        loop {
            let scores = evaluate(&population, fitness);
            // show_population(run, gen, &population, &scores);
            if solved(&scores) {
                if verbose {
                    show_population(run, gen, &population, &scores);
                } else {
                    println!("{},{}", run, gen);
                }
                break;
            } else {
                gen += 1;
                population = evolve(&population, &scores, crossover_rate, mutation_rate);
            }
        }
    }
}
