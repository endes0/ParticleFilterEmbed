# Particle Filter Embbeded System Optimization Project

## Run and Check results
``` bash
$ cargo build
$ target/debug/particle_filter_embed 1000 100 p > results.txt
$ python3 plot_results.py
```

. $HOME/export-esp.sh

## Measure execution time
``` bash
$ cargo build --release
$ hyperfine --parameter-scan num_particles 100 1000 --parameter-step-size 100  './target/release/particle_filter_embed {num_particles} 100' --export-csv results/1000_iter_100_particles.csv
$ hyperfine --parameter-scan num_particles 1000 10000 --parameter-step-size 1000  './target/release/particle_filter_embed {num_particles} 100' --export-csv results/1000_iter_1000_particles.csv

$ hyperfine --parameter-scan num_iter 100 1000 --parameter-step-size 100  './target/release/particle_filter_embed 1000 {num_iter}' --export-csv results/var100_iter_1000_particles.csv
$ hyperfine --parameter-scan num_iter 1000 10000 --parameter-step-size 1000  './target/release/particle_filter_embed 1000 {num_iter}' --export-csv results/var1000_iter_1000_particles.csv
```