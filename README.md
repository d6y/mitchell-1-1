# Basic GA

Implementation of a basic GA from the first
exercise in _An Introduction to Genetic Algorithms_ by Melanie Mitchell (p. 31):

> "Implement a simple GA with fitness-proportionate selection,
roulette-wheel sampling, population size 100, single-point crossover
rate p_c = 0.7, and bitwise mutation rate p_m = 0.001.
Try it on the following fitness function: f(x) = number of ones in x,
where x is a chromosome of length 20.
Perform 20 runs, and measure the average generation at which the string of all ones is discovered."

## Example run:

By default 20 runs are made, and the output shows which generation the solution was found in:

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/mitchell-1-1`
run,generation
1,61
2,17
3,16
4,68
5,47
6,25
7,15
8,29
9,19
10,28
11,17
12,28
13,24
14,17
15,18
16,23
17,31
18,31
19,52
```

Verbose output (flag in the source) shows the details of every generation in every run.

## Optimised run

```
$ cargo build --release

$ time ./target/release/mitchell-1-1
run,generation
1,25
2,20
3,26
4,15
5,15
6,19
7,28
8,26
9,21
10,20
11,19
12,49
13,21
14,23
15,25
16,32
17,44
18,19
19,34
20,40

real    0m0.044s
user    0m0.032s
sys     0m0.008s
```