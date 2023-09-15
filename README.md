See also [index.html](./target/criterion/report/index.html)

```sh
$ cargo bench
   Compiling rust-hash-playground v0.1.0 (/tmp)
    Finished bench [optimized] target(s) in 0.90s
     Running benches/main.rs (target/release/deps/main-3b2a9888e46ae14b)
Gnuplot not found, using plotters backend
hashbrown insert        time:   [2.0949 ms 2.0978 ms 2.1008 ms]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) low mild
  1 (1.00%) high mild
  2 (2.00%) high severe

hashbrown get           time:   [331.09 µs 331.67 µs 332.40 µs]
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) low severe
  3 (3.00%) low mild
  6 (6.00%) high severe

Benchmarking rustc_hash insert: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 8.1s, enable flat sampling, or reduce sample count to 50.
rustc_hash insert       time:   [1.5973 ms 1.6001 ms 1.6033 ms]
Found 17 outliers among 100 measurements (17.00%)
  4 (4.00%) low mild
  4 (4.00%) high mild
  9 (9.00%) high severe

rustc_hash get          time:   [235.09 µs 235.59 µs 236.16 µs]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe
```
