# 2024 Advent of Code

## Day 1:
Day 1 consisted of taking two lists, sorting them, finding the distance between corresponding list 
elements, and returning the sum. The second part required getting the number of occurrences of each
number in list 2, and multiplying each number in list 1 by the number of times it occurs in list 2,
returning the sum these multiplications.

### Rust: 68.000 µs
In Rust I utilized a somewhat object oriented approach, first generating a structure containing both
the lists, and then defining methods to return the total distance and similarity. These methods made
heavy use of iterators which resulted in a 13% time save compared to utilizing for loops. Sorting the
lists dominate the time complexity resulting in O(n log n).

### Python: 421.063 µs
I'm rather rusty when it comes to Python, so I took a pretty naive approach. Ultimately 
though the code iterates over the input data 4 times and maintains an O(n log n) time complexity 
which is the same as my Rust code.

## Day 2:
Day 2 gives a list of of reports containing an arbitrary amount of numbers, and asks to check each 
set to see if it is both all increasing or all decreasing, by 1..=3, and return the number of "safe" 
reports. Part 2 asks you to correct the number of safe reports if you allow reports that can be fixed
by removing a single number. The simplest method of doing this would be to check each unsafe report
for safety after ignoring a single number, and then do it again ignoring the next single number. However,
this introduces many more iterations, it would be possible to cut down on these by finding where each
error occurs on the first iteration and then checking only those with a limited number of errors.

### Rust: 270.390 µs
To implement the simple method, I first created a structure holding the reports, then created a method
that confirmed the safety of the report by initializing mutable booleans for whether it is increasing
or decreasing, in order, and a tuple containing the min and the max change between each value. If the
report is in order, and has changes within the proper range, the method returns true. Then there is a
second method for iterating through all reports and checking for safety, and another for iterating through
the reports marked as unsafe and checking if removing any single number results in it becoming safe. 
Although not especially optimized it clocked in at 299.24 µs. After a little bit of optimization I got
it down to 270.390 µs.

