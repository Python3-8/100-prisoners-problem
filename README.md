# 100 prisoners problem

This is a simulation, written in Rust, of the
[100 prisoners problem](https://en.wikipedia.org/wiki/100_prisoners_problem) and
its solution, inspired by [Veritasium](https://www.youtube.com/@veritasium)'s
_[The Riddle That Seems Impossible Even If You Know The Answer](https://www.youtube.com/watch?v=iSNsgj1OCLA)_.

Compiling and executing this simulation, it can be seen that the probability of
winning is indeed ~31.18% for 100 prisoners, and the relative frequency of
winning of course gets closer and closer to the actual probability as the number
of iterations increases (currently I have it at 1,000,000 which runs in about 12
seconds).

# Sample output

Here is the output from running this program 10 times:

```sh
❯ ./target/release/hundred-prisoners-problem
312104 wins out of 1000000 iterations with 100 prisoners each time
winrate: 31.2104%
❯ ./target/release/hundred-prisoners-problem
312025 wins out of 1000000 iterations with 100 prisoners each time
winrate: 31.2025%
❯ ./target/release/hundred-prisoners-problem
311641 wins out of 1000000 iterations with 100 prisoners each time
winrate: 31.1641%
❯ ./target/release/hundred-prisoners-problem
312049 wins out of 1000000 iterations with 100 prisoners each time
winrate: 31.2049%
❯ ./target/release/hundred-prisoners-problem
311594 wins out of 1000000 iterations with 100 prisoners each time
winrate: 31.1594%
❯ ./target/release/hundred-prisoners-problem
311321 wins out of 1000000 iterations with 100 prisoners each time
winrate: 31.1321%
❯ ./target/release/hundred-prisoners-problem
311535 wins out of 1000000 iterations with 100 prisoners each time
winrate: 31.1535%
❯ ./target/release/hundred-prisoners-problem
311055 wins out of 1000000 iterations with 100 prisoners each time
winrate: 31.1055%
❯ ./target/release/hundred-prisoners-problem
312222 wins out of 1000000 iterations with 100 prisoners each time
winrate: 31.2222%
❯ ./target/release/hundred-prisoners-problem
311834 wins out of 1000000 iterations with 100 prisoners each time
winrate: 31.1834%
```
