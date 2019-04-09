# `ACOre`

>  Ant Colony Optimization Rust Engine

## Purpose

The Ant Colony Optimization (aka ACO) is an algorithm based on brut force
resolution of NP-complet problems such as sales man problem.

This project aims to expose a common API to solve problems based on an ACO
engine.

## Features

### Default

Only the ACO engine.

### Parser

Adds a JSON parser to start solving problems defined by JSON objects.

## Roadmap

- [ ] Add unit tests for having full coverage
- [ ] Add benchmark tests to measure performances improvements
- [ ] Parallel ants exploration execution
- [ ] JSON / YAML nodes parser
- [ ] Optimal paths extractions in JSON / YAML format
- [ ] Parallel colonies exploration with mutual exclusion (clustering)
  - [ ] Mutual exclusion of explored paths for ants starting from different
        departure nodes
