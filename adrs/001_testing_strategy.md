# ADR-0001: Testing Strategy

**Date**: 20/10/22  
**Status**: Current

## Decision

Tests will be organised into files according to the language feature they 
exercise, e.g. `string_literals.rs`, `row_types.rs`, etc.

The tests themselves will aim to cover the "user-facing" parts of the language.
The initial expectation is that there will be at least one test for each 
command in the binary.

## Context

If tests are organised by compiler/interpreter phase then we'll end up with
a small number of very large files. 

By doing it this way, it should theoretically be possible to test a new feature
in a purely additive way - i.e., without changing existing test files.

## Tradeoffs

The test interface to the `untitled_programming_langauge_project` needs to be
shared between multiple test files. Shared code in cargo integration tests 
needs to live in a module within the test directory, and for some reason the 
dead code warnings don't work super well with this setup. As a result, these
shared modules should be declared as `pub mod` in each file they're used from,
which prevents the warnings.