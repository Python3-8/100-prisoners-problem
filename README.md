# 100 prisoners problem

This is a simulation, written in Rust, of the
[100 prisoners problem](https://en.wikipedia.org/wiki/100_prisoners_problem) and
its solution, inspired by [Veritasium](https://www.youtube.com/@veritasium)'s
_[The Riddle That Seems Impossible Even If You Know The Answer](https://www.youtube.com/watch?v=iSNsgj1OCLA)_.

The first version (which I made at like 2 AM today 12/23/23) had no
optimizations made and was very slow. Therefore today I've udpated the code to
use a thread pool, and thus the use of parallel processing makes the program
much faster (14s -> 4s for 1M iterations).

Compiling and executing this simulation, it can be seen that the probability of
winning is indeed ~31.18% for 100 prisoners, and the relative frequency of
winning of course gets closer and closer to the actual probability as the number
of iterations increases (currently I have it at 10,000,000 which runs in about
45 seconds).

# Sample output

Here is the output from running the simulation 10,000,000 times with 100
prisoners each time, using a thread pool with 15 workers:
![image](https://github.com/Python3-8/100-prisoners-problem/assets/66139317/bf6474f2-cba9-4714-8b5e-84676badc31a)
