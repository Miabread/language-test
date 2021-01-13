# boolean

## Boolean

Standard boolean data type with two variants, `True` and `False`.

Booleans are commonly used to represent flags or toggles.
For use with conditionals, please check `optional::if`.

### Examples

```
let has_desk = Boolean::True;
let has_chair = Boolean::False;
```

## not()

The logical negation operator. Inverts a `Boolean`.

This function is similar to the common `!` operator in other languages.

### Examples

```
assert_equals(True, not(False));
```

## and()

The logical AND operator. Returns `True` only if both inputs are `True`.

This function is similar to the common `&&` operator in other languages.
Note that the right argument is placed inside a block to enable short-circuiting behavior.

### Examples

```
assert_equals(True, and(True) { True });
assert_equals(False, and(False) { True });
```

## or()

The logical OR operator. Returns `True` if either inputs are `True`.

This function is similar to the common `||` operator in other languages.
Note that the right argument is placed inside a block to enable short-circuiting behavior.

### Examples

```
assert_equals(True, or(True) { True });
assert_equals(True, or(False) { True });
assert_equals(False, or(False)  { False });
```
