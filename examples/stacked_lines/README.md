# Stacked Lines Example

This example demonstrates how to build a stacked area chart on top of the
`FilledArea` plot item by stacking several series.

## Features

- Stacks multiple series so each one sits on top of the cumulative total below it
- Draws each series as a filled band, with optional boundary lines on top
- Interactive controls for the number of points, fill opacity, and boundaries

## Usage

Each series is a function of `x`. They are stacked in order: the lower boundary
of a series is the running total of all series below it, and its upper boundary
adds the series' own values on top. This is useful for visualizing how parts
contribute to a whole.

- **points**: number of sampling points
- **alpha**: opacity of the filled bands
- **show top lines**: toggle the boundary line drawn at the top of each band

## Running

```bash
cargo run -p stacked_lines
```
