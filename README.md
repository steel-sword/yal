# YAL - Yet Another LISP

## Introduction

Yal is interpretable LISP-like language. My goal for making a LISP is... If someone can make a LISP, why not?

Download the project and build it by command `cargo build --release`.
You can find the interpreter in `target/release`, open the folder and run `./yal <filename>`

## Syntax

### Comments

Any text after the `#` symbol will be ignored until the line end.

### Basic types

- Number. Actually, it's `f64` of rust, i was too lazy to make different types of numbers so i made it like a JS.
  You can write it like that: `1` or like that: `1.35`
- Str. Write your text between quotes like that: `"your string"` and you'll get a string.
- Nil. Write `nil` or `()`. It's nil. Just nil.

### Dotted pair and linked list

Dotted pair is the basic data structure in LISP.
In Java syntax (everyone knows Java syntax) it could look like that:

```java
class DottedPair {
    Object left;
    Object right;

    public DottedPair(Object left, Object right) {
        this.left = left;
        this.right = right;
    }

    public Object getLeft() { return left; }

    public Object getRight() { return right; }
}
```

Write `(1 . 2)`. You've got a pair of `1` and `2`.
Then write `(1 . (2 . (3 . ())))`. You've got a linked list.
It looks ugly but there is a short syntax for our linked list: `(1 2 3)`. It gets the same result as the previous example.

You can make irregular linked list: `(1 2 . 3)`.
The result will be: `(1 . (2 . 3))` instead of `(1 . (2 . (3 . ())))`

So, the main paradigm of LISP is that everything is a linked list. Everything! The interpreter reads your file and considers every first element of a linked list as referense to a function and the rest as its parameters and then calls it. If any parameter is a list too, it will calculate it the same way before it calls the outer.
For example:
```
(println 1)
```
This will print the `1` to stdout.

Or:
```
(println (+ 1 2 3))
```
This will print a sum of these numbers. Yes, `+` is a function too.
So, the interpreter calls `(+ 1 2 3)` first and gets `6`, and then calls `(println 6)`.

Let's try something more difficult:
```
(println (/ (+ (* 5 5) (* 10 15)) (+ (* 3 3) (* 15 2))))
```
You could say: 'it looks like a prefix notation'. Yes, it is!
Let's look at some built-in functions:

### Built-in functions
- `println`: Prints every parameter in the next line
- `print`: Print every parameter at the same line and doesn't move to the next one.
- `+`, `-`, `*`, `/`, `%`: i think, if you are programer, you don't need me to explain what it is
- `input`: reads a line from stdin. It has no parameters, just write it like that `(input)`.
- `=`: Returns `1` if all parameters are equal, else returns `nil`
- `!=`: Returns `1` if all parametes are not equal the first one, else returns `nil`
- `>`: Returns `1` if every parameter is less than previous one, else returns `nil`
- `>=`: Returs `1` if every parameter is less than or equals to the previous one, else returns `nil`
- `<`, `<=`: I hope you understood the idea.
- `cmp`: Takes two parameters and returns `-1` if the first one is less than the second one, or returns `1` if the first one is greater than the second one, or returns `0` if they are equal.
- `pair`: Another way to create a dotted pair. Takes rwo parameters. I make my LISP for fun and didn't like classic `cons`, so i named it `pair`.
- `left` and `right`: I didn't like classic `cdr` and `car` and named it `left` and `right`
  They return left or right value of the pair.
  `(println (left (1 . 2)))` will print `1`.
  `(println (right (1 . 2)))` will print `2`.
- `concat`: Convers all its parameters to string and concatenates them.
- `number`: Tries to convert the parameter to Number
- `str`: Tries to convert the parameter to Str
- `split`: Gets string and splits it by the spaces or by the optional second parameter

### Special Forms
Sometimes you need special ways of linked list calculation, not a function call and YAL (actually, any LISP) can help you.

First one is the `if` special form.
It works like that: Calculates the first parameter and if it is true (anything except `nil`) it calculates the first parameter and returns it, else it calculates the second parameter and returns it too.
For example:
```
(println (if (= 5 5) 10 15))
```
This code will print `10`, not `15`.
But why we need it to be a special form and calculated in this order? Couldn't we make an `if` function?
This example will explain you
```
(if (= (input) "hi") (println "Hello world") (println "Goodbye"))
```
If `if` was a function, the interpreter would calculate its parameters before it called the `if`, so it would print both strings: `"Hello world"` and `"Goodbye"`. But we need just one of them! So there most be special order of calculation.

Other ones are `and` and `or` special forms. They work like C-like `&&` and `||`. I think there is no need to explain the reason.
Example of using
```
(if (or (long_calculation 100) (long_calculation 200))
    (if (and (long_calulation 400) (long_calulation 500))
        (println "X")
    )
    # else 
    (if (and (long_calulation 150) (long_calulation 250))
        (println "Y")
    )
)
```
P.S: In my own language i write with my own code-style. I think LISP-style isn't understandable in a deeply nested code.

Then we have `let` form. I didn't copy a classic `let` from LISP and made it in my oun style.
`let` just declares a variable.
For example
```
(let x 100)
(println x)
```
This code will print `100`.
A variable couldn't be changed after its definition.

But what if you want something like a classic `let` to make a block of code?
YAL has a special form for that - `do` form.
`do` form creates a new scope and returns the last calculated expression in it.
You can declare variables in the scope and they will be deleted after the scope ending.
Example:
```
(let data
    (if (= some_var "some text") (do
        (let x (* 100 (some_calc 200)))
        (let y (* 10 (some_calc 20)))
        (let z (+ x y))
        (println "x and y:" x y)
        z
    ) (do
        (println "something went wrong")
        nil
    ))
)
```
Here we declared a variable `data` witch is result of calculation of `if` form. But we wanted to print something during the calculation and used `do` form. So `do` form looks like a `{}` block in C-like languages.

The next special form is great. Let's look at it in the next chapter

### `def` form
`def` form is cool. It can declare your custom functions. It looks like `define` from Scheme language.

```
(def (function_name arg1 arg2 arg3 arg_n) (expression))
```

For example:

```
(def (pow number) (* number number))
(println (pow 5))
```
This code will print `25`
But we have just one expression, what if we want to write a long code?
You can use `do` form!

```
(def (print_params p1 p2 p3) (do
    (let x (* p1 p2))
    (let y (/ p2 p3))
    (pritnln (concat "x=" x " y=" y))
))
(print_params 20 10 5)
```
This code will print `x=200 y=2`.
Like a Scheme you can make vararg using an irregular list for parameters.
```
(def (foo x y . z) (println (concat x " " y " " z)))
(foo 1 2 3 4 5)
```
This code will print `1 2 (3 4 5)`. So, with this syntax `z` parameter became a vararg and took the rest got parameters as a linked list.
YAL is a functional programming language. It can make recursions, clojures, lambdas with syntax `(lambda (args) (expression))`. Try it by your own.

### Structs

You can declare a struct by `(struct StructName (field1 field2 field_n))` syntax.
You can create an instance by calling a function `(new StructName field1_value field2_value field_n_value)`.
You can get a field form function using a special form `(:: struct_instance field_name)`
For example:
```
(struct Position (x y))

(def (get_middle_position p1 p2)
    (new
        Position
        (/ (+ (:: p1 x) (:: p2 x)) 2)
        (/ (+ (:: p1 y) (:: p2 y)) 2)
    )
)

(let position1 (new Position 10 20))
(let position2 (new Position 15 25))
(let position3 (get_middle_position position1 position2))
(println position3)
```
This code will print `(Position ((x 12.5) (y 22.5) ))`
