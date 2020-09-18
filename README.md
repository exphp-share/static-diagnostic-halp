# static-diagnostic-halp

```
git clone https://github.com/exphp-share/static-diagnostic-halp
cd static-diagnostic-halp
cargo run
```

```
   Compiling mac v0.1.0 (/mnt/e/Downloaded Software/Touhou Project/thcrap/halp/mac)
   Compiling halp v0.1.0 (/mnt/e/Downloaded Software/Touhou Project/thcrap/halp)
error[E0080]: evaluation of constant value failed
 --> src/main.rs:1:1
  |
1 | #[mac::attr]
  | ^^^^^^^^^^^^ attempt to subtract with overflow
  |
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
```
