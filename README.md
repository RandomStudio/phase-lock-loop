## Notes
Based on https://en.wikipedia.org/wiki/Phase-locked_loop and ported directly from the C code as per https://liquidsdr.org/blog/pll-howto/


## Installation and running

If needed, install gnuplot first, e.g. 
```
brew install gnuplot
```

Run the demo and output to dat file:
```
cargo run > pll_example.dat  
```

Output the plot using gnuplot:
```
gnuplot -e 'set terminal png size 800,700' pll_example.gnuplot > pll_example.png
```

## TODO
Simulate varying frequency.