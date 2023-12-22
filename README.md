# 100 prisoners problem

This is a simulation, written in Rust, of the
[100 prisoners problem](https://en.wikipedia.org/wiki/100_prisoners_problem) and
its solution, inspired by [Veritasium](https://www.youtube.com/@veritasium)'s
_[The Riddle That Seems Impossible Even If You Know The Answer](https://www.youtube.com/watch?v=iSNsgj1OCLA)_.

Compiling and executing this simulation, it can be seen that the probability of
winning is indeed ~31.18%, and the relative frequency of winning of course gets
closer and closer to the actual probability as the number of iterations
increases (currently I have it at 1,000,000 which runs in 12 seconds).
