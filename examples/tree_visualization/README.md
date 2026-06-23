# Tree Visualization Example

This example demonstrates how to draw a **tangled tree** visualization with
`egui_plot`: a node-link diagram for a directed acyclic graph where a node can
have several parents. It is a port of
[Nitaku's "Tangled Tree Visualization II"](https://observablehq.com/@nitaku/tangled-tree-visualization-ii).

## Features

- Lays out nodes level by level, stacking them vertically
- Groups edges that share the same set of parents into *bundles*
- Routes each edge as a rounded, metro-map-style link with a halo so crossings
  stay readable
- Interactive controls for corner radius, link width, per-bundle coloring, and
  labels

## How it works

The sample data is a small influence graph of programming languages. Each level
is a list of nodes; a node lists the ids of its parents in earlier levels. The
layout assigns every node an `(x, y)` position, computes a vertical trunk
(`bundle`) for each shared parent set, and emits one polyline per edge built
from straight segments and quarter-circle arcs. The polylines are drawn with
`Line`, the nodes with `Points`, and the labels with `Text`.

## Running

```bash
cargo run -p tree_visualization
```
