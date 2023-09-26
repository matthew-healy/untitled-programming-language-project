# ADR-003: Use single-argument lambdas in the AST

**Date**: 26/09/2023  
**Status**: Current

## Decision

Always treat lambdas/functions in the abstract syntax tree as if they only take
a single argument.

## Context

While spiking an implementation of a bidirectional typechecking algorithm, it
became clear that having lambdas with multiple arguments represented in the AST
was making things unnecessarily complicated, by requiring the type checking and
inference functions to take multiple "steps" at once.


## Tradeoffs

Since we can easily build single argument lambdas from the current surface 
syntax, and since it's relatively trivial to walk the AST to "gather up" nested
lambdas into a single multi-arg closure in the bytecode, there's no real
tradeoff here. It simplifies the AST at no major cost.
