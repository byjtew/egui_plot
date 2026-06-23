# Scatterplot Matrix (SPLOM) Example

This example renders a scatterplot matrix (SPLOM), ported from the classic
[D3 SPLOM example](https://observablehq.com/@d3/splom/2). It uses a grid of
`egui_plot::Plot` widgets, one per cell, since `egui_plot` has no native SPLOM.

## Features

- N×N grid of scatterplots for several numeric variables
- Off-diagonal cells plot variable `col` (x) against variable `row` (y)
- Diagonal cells label the variable
- Points colored by category (penguin species)
- Controls for cell size, point radius, and diagonal labels

## Usage

The example uses a small hand-picked subset of the Palmer Penguins dataset
(four measurements, three species):

- **Cell size**: side length in pixels of each square cell
- **Point radius**: radius of the scatter points
- **Diagonal**: toggle the variable name labels on the diagonal

## Running

```bash
cargo run -p scatterplot_matrix
```
