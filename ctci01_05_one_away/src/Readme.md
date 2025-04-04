# Cracking the Coding Interview - Question 01-05 - One Away

This is about the Levenstein metric: Given two strings check if they are no more than one edit
away from each other. Edit operations are: insert, delete, and replace.

So this is about calculating the Levenshtein distance between two strings but the algorithm can terminate early if the editing distance is bigger than 1.

## Discussion and algo design

First we can add some up-front tests to shorten the algorithm: If the length of the two strings is more than 1 we can obviously not achieve the required goal and we can stop early.

Next we do a sweep over both strings. As long as the characters in both is the same we keep sweeping. If the characters differ, we can do one of the three editing operations. Here we have to try out each of the three editing ops to see which one works best. However, we have the advantage that we only need to go to editing distance one.

If the two strings have the same length only replacement is allowed. Everything else will lead to at least two edits:
- If the characters differ and we already did an edit we stop and return false
- If the characters differ replace the current character in string 1 with the current of string 2
- if we reach the end of the two strings we return true

If the length difference is one character, we iterate over the two strings trying to make the longer of the two strings looking the same as the shorter one.

We simply check the character after the current one: 
- If the characters differ we simply remove the current character in the longer of the two strings
- If we reach the end of the shorter string without any editing we simply remove the last char of the longer string
- If we reach the end 

Some example string pairs:

- abcdefg axxdefgh --> false
- abcdefg axcdefg --> true
- "" "" --> true
- abc abc --> true


```pseudocode
fn one_away(s1, s2) --> bool:
    if |len(s1) - len(s2)| > 1:
        return false;
    if len(s1) == len(s2):
        replacement_iter(s1, s2);
    if len(s1) > len(s2):
        return remove_iter(s1, s2)
    else
        return remove_iter(s2, s1)

// we can safely asume that s1 is 1 char longer than s2
fn remove_iter(s1, s2) --> bool:
    bool edit_done = false
    for i in [0..len(s2)]:
        if s1[i] != s2[i] and edit_done:
            return false
        if s1[i] != s2[i]:
            s1 = s1[0..i] + s1[i+1..]
            edit_done = true
    
    if not edit_done:
        // no edits done, i.e. s1 is one char longer than s2 and we simplt cut it
        s1 = s1[0..len(s1)-1]
    else:
        // nothing to be done. we had one edit, so the strings now have the same length

    return true

// we can safely assume that s1 and s2 have the same length
fn replacement_iter(s1, s2) --> bool:
    bool edit_done = false
    for i in [0..len(s2)]:
        if s1[i] != s2[i] and edit_done:
            return false
        if s1[i] != s2[i]:
            s1[i] = s2[i]
            edit_done = true;
    return true

```