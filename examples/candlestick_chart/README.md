# Candlestick Chart Example

This example demonstrates a financial OHLC candlestick chart built on top of the
`BoxPlot` item.

## Features

- Each candle maps onto a `BoxElem`: high/low are the whiskers, open/close the body
- Green (bullish) candles when the price rose, red (bearish) when it fell
- Optional simple moving-average overlay
- Controls for the number of candles and the moving-average window

## Usage

The series is synthetic — a deterministic random walk where each candle opens at
the previous close. Toggle the moving average and adjust its window, or change
how many candles are shown.

## Running

```bash
cargo run -p candlestick_chart
```
