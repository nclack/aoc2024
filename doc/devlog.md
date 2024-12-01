# 2024-12-01

Starting AOC again even though I really shouldn't :)

## Day 1 

### part 1

Approach is to sort both lists, compute pairwise distances, and sum.

I'll use nom to parse to a burn tensor.

I remember, I wanted to create an egui frontend for running these things.

Trying to use claude to understand burn documentation was frustrating at best,
and it also did a poor job on the nom parser.

### part 2

- need frequency from right column, $n(x)$
- compute $$\sum ({left}_i * n({left}_i))$$

The trouble is we need a histogram and it's not in burn.

Just made one myself using HashMap.

I did find the `Tensor::iter_dims()` function which was pretty useful.
It allows me to avoid the god awful slicing routines.
