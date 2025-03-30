# Static vs Dynamic

This is a simple benchmark to compare the performance of static vs dynamic dispatch in Rust.

## Results

```
cargo bench

     Running src/bench.rs (target/release/deps/bench-b9bfa2db41e653e9)
Gnuplot not found, using plotters backend
stuff_static            time:   [402.81 µs 403.61 µs 404.44 µs]
                        change: [-6.3127% -5.5500% -4.8104%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 31 outliers among 1000 measurements (3.10%)
  24 (2.40%) low mild
  4 (0.40%) high mild
  3 (0.30%) high severe

stuff_dyn               time:   [397.50 µs 398.28 µs 399.13 µs]
                        change: [-2.6282% -2.0069% -1.4176%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 33 outliers among 1000 measurements (3.30%)
  1 (0.10%) low severe
  17 (1.70%) low mild
  6 (0.60%) high mild
  9 (0.90%) high severe
```

## Conclusion

There's a slight performance improvement for static dispatch, but it's not enough to justify the complexity of static dispatch.

## Notes

I'm not sure if this is accurate and I'd love to see a more thorough benchmark.
