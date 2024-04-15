## Challenges

- 4.4: C-style `/* block comments */`

- 6.2: C-style ternary operator `? :`

- 6.3: Add error productions to handle each binary operator appearing without a
  left-hand operand. In other words, detect a binary operator appearing at the
  beginning of an expression. Report that as an error, but also parse and
  discard a right-hand operand with the appropriate precedence.

- 7.2: concatenate strings to non-strings

- 7.3: runtime error for dividing by zero

## TODO

- [ ] rewrite commits to have personal email
- [ ] main.rs program flow is bad
- [ ] error handling is bad

## Rust-Analyzer

```
match self {
    Self::ScanErrs(v) => v.push(e),
}
```

fill match arms should fill these two lines
```
    Self::ParseErrors(_) => todo!(),
    Self::RuntimeError(_) => todo!(),
```

but instead it produces
```
    ErrorState::ScanErrs(_) => todo!(), // duplicated case
    ErrorState::ParseErrors(_) => todo!(),
    ErrorState::RuntimeError(_) => todo!(),
```

Unmerge

```
use foo::{bar::{a,b}, c};
```

produces

```
use foo::{bar::{a,b}};
use c;
```

W/ extra braces.
