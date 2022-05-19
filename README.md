# rcaml
rcaml is a toy OCaml REPL implementation.

# Goals
rcaml sets its goals from the viewpoints of syntax and type inference.

### syntax
Supports a very small subset syntax of OCaml.

- All basic types, lists, tuples, and recods
- Pattern match (may not support all syntax, though...)
- Global and local bound values, and functions including `rec`s

### Type inference
Supports simple and easy type inference.

- Statically-typed. Infers all types of bound values and functions at compile time.
- Raises an error when a type is invalid.
