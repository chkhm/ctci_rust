# Cracking the Coding Interview - Chapter 2 Question 1 - Remove Duplicates from a Linked List

Write a function that removes all duplicates from a linked list.

Follow-up Question: How can you solve the problem if you cannot use temporary buffer memory?

## Discussion

Let's first look at the - buffer memory allowed - situation: Intuition is to iterate over the linked list and putting the value of every visited node into a hashset if it is not yet there. If it is already there we remove the currently visited node.
Fairly expensive from a memory perspective but we get the job done in `O(n)`.

If no buffer memory is allowed the obvious way is to go brute force: We iterate in two nested loops: The outer loop is the search value, the inner loop searches for duplicates. The starting point of the inner loop is one after the current node of the outer loop. So we have a time complexity of `O(1/2 n^2) = O(n^2)`.

