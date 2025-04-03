# Cracking the Coding Interview - Chapter 1: Arrays - Question 1.1: Unique Characters

## Question

Implement an algorithm that determines if a string has only unique characters.
Write algorithms with and without using additional data structures.

## Discussion of my answers

### Brute force 

A brute force approach would be to iterate over the entire string and check if the current
character shows up later in the string. You can ignore characters before the current one 
because they have already been tested.

Complexity: `O(n + n-1 + n-2 + ... + 1) = O( 1/2 n^2) = O(n^2)`

Not implemented.

### Using data structures

We can achieve `O(n)` by iterating over the string and put the current character in a hash 
set. If the character is already in the headset it is obviously not unique.

### No data structures allowed - sort the array

We can achieve `O(n log n)` by sorting the string first alphabetically and then iterate over
the string searching for repeating neighboring characters.

Complexity: `O(n log n + n) = O(2n log n) = O(n log n)`

### No data structures allowed - no changing of array allowed - recursively build a search tree

Let's try another approach with a search tree...