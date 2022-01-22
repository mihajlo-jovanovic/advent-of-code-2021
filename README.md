# advent-of-code-2021

My solutions to the [Advent of Code](https://adventofcode.com/) 2021 puzzles, mostly in Rust.

`clojure` contains solutions in Clojure (dah!) for days where I felt that language is a more appropriate tool
(and I also got tired of wrestling with the Rust borrow checker ;-)

### day 11

This is exactly the kind of problem Rust is made for. I def enjoyed this one, it felt I finally picked the right tool 
for the job. Sub 1ms times for both parts...

### day 19

Problems like this one it helps to know a little bit of linear algebra. Unfortunately I had forgot all of it since a
very long time dating back to my first year of college. You would still need to do 24 transformations using matrices
cross products...apparently some tricks from statistical learning can provide a shortcut, specifically linear regression
or standard deviation calculation on a distances matrics should provide a way to match orientation. 

### day 23

Dijkstra `Shortest Path` again...took a while to find the right state representation and code it in Rust, but it was 
worth it.

## Things I learned about Rust while doing AoC
* Derived traits: Copy vs Clone, Eq vs PartialEq
* When to use usize vs u8, u32 etc.
* Testing: use --nocapture to see std out, #[ignore] etc.