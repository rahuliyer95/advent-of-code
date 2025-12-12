# Day 10

The [good_lp](https://docs.rs/good_lp/latest/good_lp/) crate requires [cbc](https://github.com/coin-or/Cbc)
solver installed in your system.

If you face compilation errors because of CBC library not being found add the path to `RUSTFLAGS`
environment variable. e.g.: `export RUSTFLAGS="-C link-arg=-L/opt/homebrew/lib"`
