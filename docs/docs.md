# Documentation

## Getting Started

### Installation
To install the language, download the executable from the [project’s repository](#) or use the following commands in your terminal (for systems supporting Rust binaries):

```bash
# Clone the repository
git clone https://github.com/GregShiner/cmpsc-470-final.git
cd cmpsc-470-final

# Install dependencies and compile the interpreter
cargo build --release

# Run the language interpreter
./target/release/cmpsc-470-final <filename>
```

## Hello World

Create a file called `hello.lisp` with the following contents:
```lisp
(display "Hello world!")
```

Then, if you have already compiled the binary:
```sh
./target/release/cmpsc-470-final hello.lisp
```

Or, compile the interpreter and run it with:
```sh
cargo run hello.lisp
```

This will print `Hello world!` to the console.

## Reference

### Basic syntax
Statements are written as s-expressions, enclosed in parentheses. Each expression can be an atomic value, an operation, or a nested expression. Code blocks are created with the begin keyword.

Example:
```lisp
(begin
  (display (+ 3 4))
  (display (+ 5 7)))
```

### Data Types Reference

- **Num**: Integer values.
- **Bool**: Boolean values (true or false).
- **Box**: Heap-allocated values that support ownership and borrowing.
- **Ref**: Immutable reference to a Box.
- **MutRef**: Mutable reference to a Box.
- **Moved**: Represents a moved (invalid) value. (Only used for internal representation; cannot be constructed on its own)

### Operators

#### Arithmetic Operators
| Operator | Purpose            | Example               |
| -------- | ------------------ | --------------------- |
| `+`      | Addition           | `(+ 5 3)`            |
| `*`      | Multiplication     | `(* 5 3)`            |
| `-`      | Subtraction        | `(- 5 3)`            |
| `/`      | Division           | `(/ 6 3)`            |

#### Comparison Operators
| Operator | Purpose                | Example               |
| -------- | ---------------------- | --------------------- |
| `=`      | Equality               | `(= 5 5)`            |
| `>`      | Greater than           | `(> 5 3)`            |
| `<`      | Less than              | `(< 3 5)`            |
| `>=`     | Greater than or equal  | `(>= 5 5)`           |
| `<=`     | Less than or equal     | `(<= 3 5)`           |

### Control Structures

#### If
Used for conditional branching
```lisp
(if (= x 0)
    "zero"
    "non-zero")
```

#### Begin
Evaluates multiple expressions in a sequence, and returns the value of the last expression.
```lisp
(begin
  (display "Step 1")
  (display "Step 2"))
```

### Functions and Procedures
Functions are always anonymous lambda functions that can be applied by applying arguments
```lisp
((lambda (x) (* x 2)) 5) ; Doubles the input
```
Define recursive functions using let-rec
```lisp
(let-rec ((factorial (lambda (n)
                        (if (= n 1)
                            1
                            (* n (factorial (- n 1)))))))
  (factorial 5))
```

## Best Practices
- **Memory Management**: Boxed values should be used judiciously because while they are still more performant than garbage collected values, they are still heap allocated which is slower.
- **Mutibility**: Since mutable references cannot exist alongside other references to the same value. Creating mutable references leads to complex problems in scenarios where you need multiple references.
- **Begin**: Pure functions that do not rely on sequencial operations can be heavily optimized and paralellized by the interpreter. Using `begin` expressions reduces the opportunities for optimization heavily.

## Grammar
```antlr
<exp> ::= <num>
        | <id>
        | true
        | false
        | (+ <exp> <exp>)
        | (* <exp> <exp>)
        | (lambda (<id>) <exp>)
        | (<exp> <exp>)  ; function application
        | (if <exp> <exp> <exp>)
        | (= <exp> <exp>)
        | (begin <exp>*)
        | (& <exp>)      ; immutable reference
        | (! <exp>)      ; mutable reference
        | (box <exp>)
        | (unbox <exp>)
        | (@ <exp>)      ; dereference
        | (:= <exp> <exp>) ; set mutable reference
```
