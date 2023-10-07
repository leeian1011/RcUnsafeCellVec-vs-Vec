# WHAT ON EARTH IS THIS? 
I am interested in the performance difference when cloning a `Vec<T>` over cloning a `Rc<UnsafeCell<Vec<T>>>`.

Rc returns a pointer to the memory location of the data it wraps vs Cloning an entire vector... you see my point?

So... Rc should be much cheaper compared to cloning an entire vector right..?

## THE SCENARIO

I am literally just smashing stuff together to see if this works, if you think this is horrid please do let me know.<br>

I have a specific scenario that I would like to benchmark.<br>
A HashMap that holds a Value of either `Rc<UnsafeCell<Vec<Struct>>>` or 'Vec<Struct>'.
We will iterate every hashmap entry and clone the Value (cursed Rc-Uc-Vec vs Vec).

## THE CURSED EXPERIMENT

So I have set these variables.<br>

Iterations = 1000

a) HashMap-Entry : 1    |  Vector-Size : 1
b) HashMap-Entry : 10   |  Vector-Size : 1 
c) HashMap-Entry : 10   |  Vector-Size : 10
d) HashMap-Entry : 10   |  Vector-Size : 100 
e) HashMap-Entry : 100  |  Vector-Size : 10
f) HashMap-Entry : 100  |  Vector-Size : 100
g) HashMap-Entry : 1    |  Vector-Size : 100
h) HashMap-Entry : 100  |  Vector-Size : 1 
i) HashMap-Entry : 1000 |  Vector-Size : 1
j) HashMap-Entry : 1    |  Vector-Size : 1000

*All times are measured in nanoseconds. The log files are also provided. All tests were ran in --release* 

average nano-seconds/iterations : 

nevermind.
!(cursed)[./Screenshot from 2023-10-07 21-58-21.png]
