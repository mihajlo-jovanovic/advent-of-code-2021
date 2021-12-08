# day 6

### Part 1
> _Find a way to simulate lanternfish. How many lanternfish would there be after 80 days?_

This one was pretty straight-forward; I was able to get it running in Rust fairly quickly.

### Part 2
> _How many lanternfish would there be after 256 days?_

Ok, so this one gave me quite a bit of trouble...more than I would like to admit. So after realizing that the approach 
from part 1 was not going to work, I then went down the proverbial `rabbit hole` trying to come up with a mathematical 
formula that would allow me to calculate the number of children based on number of days. I had this idea of using a 
weighted tree where left edges would be 7 and right ones 9...upper bound is 2^l where l is depth of the tree...long 
story short, I could not figure out a way to eliminate un-balanced nodes. I was also (unreasonably) concerned that the 
problem is harder than it really is, as the number may be larger than 2^64 and therefore could not be stored in a 
primitive.
After all that, I finally did come up with a simpler way of using linear-size list; however then I ran into issues 
writing a mutation-style solution in Rust. So I finally resorted to Clojure recursive function which worked nicely.

Some day I may return to part 2 in Rust, as it would be good to understand how to do it. One thing is clear: Rust is
not a very good choice for doing these puzzles fast; it is a neat language but it does require some thought about how to
structure the code beforehand. As well as dealing with all the low-level details: choice of primitive data type (u8, 
u16, isize etc.), going between refs and values. But I did want to learn it, so will have to stick it out.

That's all for now...

## Usage

```
lein run
```
or simply build uber-jar and run it:
```
java -jar day7-0.1.0-SNAPSHOT-standalone.jar
```

## License

Copyright © 2021 FIXME

This program and the accompanying materials are made available under the
terms of the Eclipse Public License 2.0 which is available at
http://www.eclipse.org/legal/epl-2.0.

This Source Code may also be made available under the following Secondary
Licenses when the conditions for such availability set forth in the Eclipse
Public License, v. 2.0 are satisfied: GNU General Public License as published by
the Free Software Foundation, either version 2 of the License, or (at your
option) any later version, with the GNU Classpath Exception which is available
at https://www.gnu.org/software/classpath/license.html.
