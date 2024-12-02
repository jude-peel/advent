# 2024 Advent of Code

## Day 1:
Day 1 consisted of taking two lists, sorting them, finding the distance between corresponding list 
elements, and returning the sum. The second part required getting the number of occurrences of each
number in list 2, and multiplying each number in list 1 by the number of times it occurs in list 2,
returning the sum these multiplications.

### Rust: 68.970 µs
In Rust I utilized a somewhat object oriented approach, first generating a structure containing both
the lists, and then defining methods to return the total distance and similarity. These methods made
heavy use of iterators which resulted in a 13% time save compared to utilizing for loops. Sorting the
lists dominate the time complexity resulting in O(n log n).

### Python: 421.063 µs
I'm rather rusty when it comes to Python, so I took a pretty naive approach. Ultimately 
though the code iterates over the input data 4 times and maintains an O(n log n) time complexity 
which is the same as my Rust code.

## Day 2:
Day 2 gives a list of of reports containing an  arbitrary amount of numbers, and asks to check each 
set to see if it is both all increasing or all decreasing, by 1..=3, and return the number of "safe" 
reports.
