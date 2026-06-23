# Stacked Normalized Horizontal Bar Demo

This example demonstrates how to create a stacked **normalized** (100%) horizontal
bar chart, inspired by
[D3's stacked-normalized-horizontal-bar](https://observablehq.com/@d3/stacked-normalized-horizontal-bar/2).

Each category is a full-width horizontal bar whose stacked segments show each
part's share of the total. The per-category values are normalized so every bar
sums to 100%, making proportions easy to compare across categories.

Each segment is its own `BarChart`, drawn with `.horizontal()` and stacked on the
previous segments with `.stack_on(...)`. Custom axes render the value axis as a
percentage and label the category axis, and the bar width is adjustable.

## Running

Native
```sh
cargo run -p stacked_horizontal_bar
```

Web (WASM)
```sh
cd examples/stacked_horizontal_bar
trunk serve
```

![](screenshot.png)
