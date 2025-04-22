# Coding Interview Question

You are given a 2D matrix of characters. Find within the matrix all occurrences of a given word. For spelling a word each position in the matrix can only be used once. The side of the matrix do not allow to continue on the other side. They are "walls".


## Example

Here is an example:

```
a b X d e w
f g J I J X
k l m n I y
p p m a v z
```

Find all occurrences of the word "xji". The matrix above has three occurences highlighted in upper case letters. 

Other example with character repetition in the word:

```
A N c d e f 
g A P j A l
m n P x N r
s t P P A y
z a b c d e
```

Find all occurrences of the word: "anapp".


## Solution idea 1: backtracking over the matric

Use a recursive backtracking approach:

- Find all potential starting points in the matrix
- For each found starting point:
-   Recursive_search(word, pos, idx, path)

Recursive_search(word, pos, idx, path)
- if idx equals length of word: 
  - write path into result, 
  - return
- if pos is not equal to word[idx] then return false
- if pos in path then return false
- add the pos to the path
- idx++
- for each neighbor n of pos:
  - recursive_search( word, pos, idx, path.clone() )

```
n = size of matrix
m = size of word
```

Worst Case Complexity: 

- all characters are the same, the word is as long as the matrix and all chars are the same
- every neighbor can be visited (for simplicity every field has 4 neighbors)

Complexity: `O(4^n)` i.e., exponential


## Solution idea 2: backtracking over the characters in the word

Can we reduce the effort by back tracking over the word rather than the matrix positions? Chances are this is the smaller set:

- Iterate over all fields in the matrix and add the positions to the hashmaps of all chars in the word

Recursive_search(idx, path)
- if idx equals to length of word: 
  - write path to result
  - return
- for each pos in hashset[idx]:
  -  if (pos is neighbor of path[-1])  and  (pos is not in the path):
     -  recursive_search(idx+1, path + pos)

Complexity:
- same worst case, but better in average because the word cannot be longer than the matrix has chars
- Should be quicker, because valid neighbors are limited

Complexity: `O(4^n)` in the crazy case that word and matrix all consist of the same char and have the same length

