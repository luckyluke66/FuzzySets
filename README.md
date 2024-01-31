# Fuzzy set abstraction

## Fuzzy set 
<p>is Pair (A, μ<sub>A</sub>) where A is a subset of X 
and μ<sub>A</sub>:A -> <0,1> is membership function (that maps every element of A to a number from interval <0,1>).
This function defines how much elements belong to the set.
</p>
<p>It is essentially a generalization of the normal set (often called crisp set, not to be confused with fuzzy set)
crisp sets have membership function that maps all objects from set A to set {1,0}:
</p>
1 meaning the elemenet belongs to A</br>
0 meaning it doesn't belong.</br>
<p>Fuzzy sets expands on this concepts and allow for elemets of a set to belong in a certain degree.</p>


</br>
Two fuzzy sets are equal (a = B), if and only if μ<sub>A</sub>(x) = μ<sub>B</sub>(x)
for each x ∈ X.

## Fuzzy operations

### Union (OR)
The union of two fuzzy sets A and B is a fuzzy set C = A ∪ B,
whose membership function is determined by
</br>
    μ<sub>C</sub> = MAX(μ<sub>A</sub>(x), μ<sub>B</sub>(x)), x ∈ X.

### Intersection (AND)
The intersection of two fuzzy sets A and B is a fuzzy set C = A ∩ B,
whose membership function is determined by
</br>
    μ<sub>C</sub> = MIN(μ<sub>A</sub>(x), μ<sub>B</sub>(x)), x ∈ X.


all of the operations use definitions given by Lotfi A. Zadeh in his paper Fuzzy Sets
