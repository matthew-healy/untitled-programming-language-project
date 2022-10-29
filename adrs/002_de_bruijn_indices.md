# ADR-0002: De Bruijn Indices

**Date**: 28/10/22
**Status**: Current

## Decision

Use De Bruijn indices for bindings. 

## Context

Previously we modelled the environment as a `HashMap` from raw identifiers
to values. This required us to pass the identifiers around, e.g. as part of both
the `Let` and `EndLet` operations, cloning them each time. Using De Bruijn
indices avoids the need to pass heavy raw identifers around, as we're always
just looking at an index of the environment. This also avoids the need to ever
worry about renaming.

## Tradeoffs

We need to calculate the De Bruijn indices in a separate interpreter step. We
also need to differentiate between the "raw" AST and the "processed" AST where
the variables have been replaced with their indices. This currently requires
duplication of the AST, though it may be possible to find a smart generic 
solution for this later.
