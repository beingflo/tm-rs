# Turing machine Simulator

### Building
Install Rust from [the Rust website](https://www.rust-lang.org/)

To build, clone the repository and execute 

```
    cargo build
```

### Running
Execute 
```
    cargo run [filename]
```

For instance
```
    cargo run examples/bin2dec.tm
```


### .tm Syntax

### Config
```
[c]:10000
```
\[c]: followed by a number specifying how many simulation steps should be performed as most, before the TM stops.

#### States
```
[s]:a,b
```
\[s]: followed by a comma separated list of the names of the states, these may be arbitrary sequences of characters except for ','. 


#### Alphabet
```
[a]:0,1
```
\[a]: followed by a comma separated list of the letters, these may be any single character except ',', '[' and ']'.


#### Start state
```
[e]:a
```
\[e]: followed by a single state name. This state has to be specified in the [s] clause. 
The turing machine will start in this state.  


#### End state
```
[x]:b
```
\[x]: followed by a single state name. This state has to be specified in the [s] clause. 
The turing machine will terminate in this state.



#### Transitions
```
[t|a]:1->(a,0,>)|0->(b,1,>)
```
\[t|a]: followed by a list of transitions separated by '|'. 
'a' refers to the state the machine has to be in to act on any of these transitions. 
Every transition is specified by 'x->(s,y,z)' where 'x' is the character that is read, 's' the state that the TM switches to, 'y' the character that is written and 'z' the movement that the TM does in this transition. 'z' may be '<' or '>'.


#### Initial configuration of band
```
[b|0]:00000[1]111100000
```

\[b|x]: followed by a sequence of character denoting the initial state of the tape. 
'x' refers to the 'default' character on the tape. This character is used to extend the tape in case the TM runs off either end of the tape.
The characters in the sequence may be any character specified in the [a] clause.
One character is surrounded by '[' and ']' to denote the starting position of the turing machine.
