# Operations

(o value) - output value
(i variable) - input to variable
(a variable value) - assign value to variable
(r value) - return value

# Functions

(f name param1 param2 ... body) - define function
(c name arg1 arg2 ...) - call function

# Loops

(l body) - infinite loop
(l condition body) - conditional loop
(b) - break

# Math

(+ a b) - add
(- a b) - subtract
(* a b) - multiply
(/ a b) - divide

# Comparison

(< a b) - less than
(> a b) - greater than
(<= a b) - less than or equal
(>= a b) - greater than or equal
(== a b) - equal
(!= a b) - not equal

# Chains

(ao variable value) - assign then output
(ia variable) - input then assign
(io variable) - input then output

# Variables

lowercase letters only (a-z)
no numbers or uppercase

# Values

strings in quotes: "hello"
variables: x, name, counter
base26 arithmetic: a=1, b=2, ..., z=26

# Comments

\# this is a comment

# Examples

(o "hello world")
(a x "hello")
(f greet name (o name))
(c greet "piko")
(l (<= counter "e") (o counter) (a counter (+ counter "a")))
(ao x "test")