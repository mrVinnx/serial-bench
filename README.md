# Serialization Performance Benchmark

A comprehensive performance comparison of popular data serialization formats in Rust: JSON, MessagePack, Protocol Buffers (Protobuf), FlatBuffers, and an wrapped FlatBuffers variant.

## Overview

This benchmark tests serialization, deserialization, and roundtrip performance using a complex data structure containing:
- Integer and string fields
- Vector of floating-point numbers (1000 elements)
- String-to-string key-value map (metadata)

## Results

**Performance**

Performance measurements with statistical analysis using Criterion:

| Format | Serialize | Deserialize | Roundtrip |
|--------|-----------|-------------|-----------|
| **Protobuf** | **590 ns** | 2.26 µs | 2.82 µs |
| **FlatBuffers** | 915 ns | **660 ns** | **1.61 µs** |
| **FlatBuffers (wrapper)** | 1.11 µs | 691 ns | 1.82 µs |
| **MessagePack** | 3.23 µs | 3.91 µs | 7.71 µs |
| **JSON** | 27.96 µs | 9.44 µs | 36.84 µs |

## Performance Comparison (JSON = 1x baseline)

| Format | Serialize Speedup | Deserialize Speedup | Roundtrip Speedup |
|--------|------------------|-------------------|------------------|
| **Protobuf** | **47.4x** faster | **4.2x** faster | **13.1x** faster |
| **FlatBuffers** | **30.6x** faster | **14.3x** faster | **22.8x** faster |
| **FlatBuffers (wrapper)** | **25.1x** faster | **13.7x** faster | **20.3x** faster |
| **MessagePack** | **8.7x** faster | **2.4x** faster | **4.8x** faster |
| **JSON** | 1x (baseline) | 1x (baseline) | 1x (baseline) |

## Setup

- Generate data files:
```bash
# Build project to generate code
cargo build
```

- Run Benchmark
```bash
cargo bench
```
