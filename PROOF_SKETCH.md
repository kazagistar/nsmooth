For the ease of explaining this proof, every n-smooth number will be represented as a unique n-long vector of exponents of the prime factors.

Here are some 3-smooth examples:

- 1  = 2^0 * 3^0 * 5^0 = [0,0,0]

- 2  = 2^1 * 3^0 * 5^0 = [1,0,0]

- 3  = 2^0 * 3^1 * 5^0 = [0,1,0]

- 20 = 2^2 * 3^0 * 5^1 = [2,0,1]

# Part 1: Partial Order Tree

We will to construct a tree structure over the n-smooth numbers for any particular n. These are the important invariants of this tree that we will ensure:

1. Every child number is larger then its parent number.

2. Every number occurs exactly once in the tree.

3. Given any number, we are able to compute its children.


In order to satisfy these constraints, a simplified set of rules is as follows:

For a given number p, there are two types of children.

The "successor" of p is a child where the largest factor with a nonzero exponent is increased by 1. Example: [3,2,0,0] has the successor [3,3,0,0], which has the successor [3,4,0,0] and so on.

The "spawn" of p are the children where one of (the exponents of (the prime factors strictly larger then (the largest prime factor with a nonzero exponent))) is incremented. Example: [0,4,0,0] has the spawn [0,4,1,0] and [0,4,0,1]. Note, [1,4,0,0] is NOT a spawn, since the first prime factor is smaller then the second, which has a nonzero exponent.

By construction, this satisfies invariant 3.

Each child is bigger then its parent by a multiplicative factor, satisfying invariant 1.

To show 2, we must demonstrate that any number [a, b, .. c, d] has only one possible ancestry back to the root node [0, 0, .. 0, 0].

The parent of a node is computed by decrementing the largest factor with a nonzero exponent. Examples: The parent of [4,2,1] is [4,2,0]. The parent of [1,2,0] is [1,1,0]. [0,0,0] has no parent (it is the root node). I haven't yet figured out how to explain that this is truly the exact inverse of the child generating rules, but I urge you to experiment with it until you are convinced.

Because this rule can be used to decide a unique trail of ancestry back to [0,0,0] from any n-smooth number, every n-smooth number is in this tree, satisfying invariant 2.

# Part 2: Sequencing

The goal of the algorithm is to iterate over the n-smooth numbers. Showing that this is done correctly will be a proof by induction. Prior to each step of iteration, all the n-smooth numbers will be in one of 3 sets:

- "Seen": The numbers we have already emitted from our iteration.

- "Lookahead": Direct children of those numbers we have iterated over which have not yet been emitted.

- "Unseen": All the rest, which means all the descendants of of numbers in the lookahead.

We will assume that after n steps, all the sets are valid, and show that after n+1 steps they will still be valid. The step will involve taking the smallest number from the "Lookahead" emitting it into the "Seen", and moving its children from "Unseen" to "Lookahead".

- Each child in the unseen is a descendant of something in the "lookahead", and thus larger then something in the "lookahead". Each item in the "lookahead" is larger then the smallest one. Therefore, this number is the smallest number not yet seen, and the order of the seen is preserved.

- The only item added to the seen has all its children added to the lookahead after each step.

- Since all the children of items in seen are in the lookahead, the most direct ancestors of numbers in the unseen must be in the lookahead.

# Part 3: Optimization and Performance

If you store the "lookahead" as a heap-based priority queue, the cost for each iteration is O(log(S)), where S is the size of the heap. In
 the tree described above, after each step, the lookahead grows by 1 to n, where n is the number of prime factors we are using.

However, the tree can be modified and still maintain the same invariants. Instead of generating many "spawn", it can generate a single "sibling". This is done by reducing the largest prime factor with a non-zero exponent by 1, and and then incrementing what was the smallest prime with a zero exponent by 1. Example: [4,2,0,0,0] creates sibling [4,1,1,0,0], which creates sibling [4,1,0,1,0], which creates sibling [4,1,0,0,1], which creates no further siblings.

These siblings are directly analogous to the spawn of the parent of the first node in this chain.

They maintain invariant 1, because while we divide by one prime factor, we multiply by a larger one, which guarantees the resulting child is still larger.

Using this optimization, the size of the heap only increases by 2 after each step, making the time complexity independent of n.

(In practice, the increased complexity of this method might lead to a slight slowdown for small n, but should be a massive win for larger n, or over a long enough iteration.)

 



