# Piko
Esolang created for the Twist YSWS program. 

# Features
The grammar is limited to one character:

- "f": Function
- "c": Call Function
- "l": Loop
- "b": Break
- "r": Return
- "o": Output
- "i": Input
- "a": Assign

# Operators
Comparison and arithmetic operators are available. However, there are no numbers allowed in literals. Or capital letters. Or any other symbols. Just a-z. If you try to perform comparison or arithmetic, the two strings you pass are interpreted as bijective base-26 numbers. The operations you perform are done on the base-26 representation of the string. 

# Chaining
Some operations can be chained.
- Assign Output (ao) assigns a value to a variable then also output's that value. 
- Input Assign (ia) takes in a value as input and stores it in the provided variable.
- Input Output (io) takes in a value and immediately calls output on it.

# Building
There are two projects in this repository, a REPL tool to test Piko and the library itself. 
To build and run the REPL directly, use the following command (you need to have Rust installed):
```
cargo run --release
```

# Minimum Supported Rust Version (MSRV)
This project requires **Rust `1.85.1` or later**.