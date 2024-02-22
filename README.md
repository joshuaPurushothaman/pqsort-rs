# pqsort-rs: Parallel Quick Sort in Rust

This is just a place to mess around with:
- threadpools
- channels
- quick sort in parallel! 


## Parallel quick sort concept:

0. if the given list has <= 3 elements, sort it (using something like insertion sort) and return it
   - *technically, i think this is advisable for some sizes larger than 3 as well due to overhead of anything more complex than insertion*

1. pick pivot point (let's use the last element in the vec)
2. make halves:
    1. make lists LEFT_LIST and RIGHT_LIST
    2. for each element
        1. append to LEFT_LIST or RIGHT_LIST, depending on whether it's less than or equal to the pivot element
3. submit LEFT_LIST and RIGHT_LIST to threadpool for recursion :)


