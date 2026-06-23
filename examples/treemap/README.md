# Treemap Example

This example demonstrates a treemap, ported from D3's [Treemap](https://observablehq.com/@d3/treemap/2) example.

A treemap tiles a rectangle with one cell per leaf of a hierarchy, where each cell's area is proportional to the leaf's value. Cells are colored by their top-level category.

## Features

- Implements the **squarified** treemap layout (Bruls et al.) for nice cell aspect ratios
- Renders each leaf as a filled `Polygon` with a thin border in a single `Plot`
- Colors cells by their top-level category and labels each cell with its name and value
- Interactive controls to toggle labels, adjust label size, fill opacity and border thickness

## Usage

The plot uses a small hardcoded software-package hierarchy (analogous to D3's flare dataset). Adjust:
- **labels**: show or hide leaf labels
- **label size**: font size of the labels
- **fill opacity**: translucency of the cells
- **border**: cell border thickness

## Running

```bash
cargo run -p treemap
```
