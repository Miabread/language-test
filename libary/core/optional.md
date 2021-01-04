# optional

## Optional

An optional represents a value that may or not be present.

It is either a `Some(T)` variant containing a present value or the `Nothing` variant representing the lack thereof.
Optionals have many uses cases, including:

- "Nullable" values
- Conditionals
- Partial functions
- Simple errors
- Optional arguments

### Examples

Pattern matching on a `Optional`:

```
let maybe_color = Some("red");

match(maybe_color) {
    Some(color) -> print("The color is \(red)"),
    Nothing -> print("There is no color"),
};
```

## if()

Evaluate a block if a condition is `True`.

This is similar to `if` statements in other languages, but notably this `if` can return a value.
The result of the function is either a `Some(R)` if the block ran or a `Nothing` otherwise.
`if` is commonly used in conjunction with `else`.

### Examples

Simple conditionals:

```
if(equal(x, 123)) {
    print("x matches the number");
};

if(equal(y, 456)) {
    print("y matches the number");
};
```

Returning values:

```
let foo = if(True) { 123 };
let bar = if(False) { 456 };

assert_equal(foo, Some(123));
assert_equal(bar, Nothing);
```

## else()

Evaluate a block if a optional is `Nothing`.

This is used to unwrap a `Optional` by providing a default value.
It also is commonly used in conjunction with `if` to provide a fallback if a condition was `False`.

### Examples

Unwrapping optionals:

```
let foo = Some(123);
let bar = Nothing;

let foo = foo else { 456 };
let bar = bar else { "default" };

assert_equal(foo, 123);
assert_equal(bar, "default");
```

Fallback in conditionals:

```
let foo = if(False) {
    123
} else {
    456
};

asset_equal(foo, 456);
```

## else_if()

Equivalent to calling `if` inside an `else` block.

This function exists to mimic conditionals in other languages and serves no other purpose.
Unlike `if`, the conditional argument is placed inside a block to enable short-circuiting behavior.

### Examples

```
let foo = if(False) { "first" }
    else_if { False } { "second" }
    else_if { True } { "third" }
    else { "last" };

assert_equal(foo, "third");
```

Equivalent:

```
let foo = if(False) {
    "first"
} else {
    if(False) {
        "second"
    } else {
        if(False) {
            "third"
        } else {
            "last"
        }
    }
};

assert_equal(foo, "third");
```
