# Cracking the Coding Interview - Chapter 1 Question 8 - Zero Rows and Columns in an Array

Write a function that takes a 2D array as input and zeros the entire row and column of every element in the array that has the value zero.

Idea:

Brute force:

We first iterate over the entire array and write out the positions with value 0. We then iterate over all found locations and zero the corresponding row and column.

We can reduce the effort by avoiding double work: If a row or column has already been deleted in a previous attempt we can leave it out. For this purpose,
we write each zero'd row and column into a hash set.
Before zeroing a row we check in the hash set if it has already been zerod. We do the same before zeroing a column.

Avoiding the hash set: we can avoid the hash set by assuming that we delete rows and columns in ascending order. This means: when zeroing a row we check the element to the left of the current one if it is already zero. If it is we don't have to do anything. In the same way, before deleting a column we check the element before the current one. If it is already zero we can skip the column.
