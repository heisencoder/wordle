# wordle
This repository contains tools for the [NY Times' Wordle Game](https://www.nytimes.com/games/wordle/index.html).

## Word List
The file [words.txt](words.txt) contains all the possible Words in the game, stored as two Javascript arrays.  
The first array contains all the words that can appear as a solution, and the second list contains all the additional words that can be used as guesses.
These words were taken directly from the JavaScript used by the game itself.

The file [solutions.txt](solutions.txt) contains just the possible solutions, formatted as one word per line. 

Similarly, the file [guesses.txt](guesses.txt) contains all the other words that wordle allows as a valid guess.

## Statistics

The Python3 file [stats.py](stats.py) contains code to do quick statistical analysis across the words, with results outlined below:

### Letter usage in solutions.txt

This table shows the percent chance that a particular letter appears in a random word from solutions.txt:

```
e: 45.60%
a: 39.24%
r: 36.16%
o: 29.10%
t: 28.89%

i: 27.98%
l: 27.93%
s: 26.72%
n: 23.73%
u: 19.75%

c: 19.32%
y: 18.02%
h: 16.33%
d: 16.02%
p: 14.94%

g: 12.95%
m: 12.91%
b: 11.52%
f:  8.92%
k:  8.75%

w:  8.36%
v:  6.41%
x:  1.60%
z:  1.52%
q:  1.26%
j:  1.17%
```

This distribution is in a slightly different order than standard English words of any length (which roughly follow the order "etaoin shrdlu" for the first 12 characters).

### Letter usage in guesses.txt

The same analysis on guesses shows a skew towards words containing 's':

```
s: 49.87%
e: 43.62%
a: 41.50%
o: 30.38%
r: 28.84%

i: 27.59%
l: 23.15%
t: 22.18%
n: 21.01%
u: 18.57%

d: 18.08%
y: 15.14%
m: 14.72%
p: 14.44%
c: 13.82%

h: 12.48%
b: 11.75%
g: 11.66%
k: 11.65%
w:  7.83%

f:  7.35%
v:  4.93%
z:  3.34%
j:  2.46%
x:  2.34%
q:  0.78%
```

This is likely because the guesses.txt file contains a lot of pluralized versions of 4-letter words, which are apparently not common in the possible solutions.
