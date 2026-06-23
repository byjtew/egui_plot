# Animated Signal Example

This example demonstrates a continuously animated plot: a very noisy,
electric-style signal that scrolls to the left as new samples arrive on the
right.

## Features

- Real-time scrolling line, redrawn every frame via `request_repaint_after`
- FPS slider that controls the scroll speed by throttling the repaint rate
- Fixed view: the y-axis is pinned to `0..2` and the x-axis spans `0..100`
- Deterministic noise function, so the trace is reproducible (and screenshot-testable)

## Usage

The signal fluctuates around `1.0`, staying within `0.8..=1.2`. The sample
window scrolls left over time, so features drift from the right edge (newest)
toward the left edge (oldest). The **FPS** slider sets how many frames are drawn
per second, and therefore how fast the trace moves.

## Running

```bash
cargo run -p animated_signal
```
