# Beating C in 80 Lines of Rust

After reading Chris Penner's article titled, "[Beating C With 80 Lines of Haskell][haskell]", I just couldn't help
myself and decided to replicate the experiment using Rust.

First, the results using `wc(1)`:
```sh
$ time wc enwik9
 13147025 129394476 1000000000 enwik9

real	0m2.655s
user	0m2.440s
sys	0m0.199s
```

Next, using `wc-rs`:

```sh
$ time ./target/release/wc-rs enwik9
13147025 129394476 1000000000 enwik9

real	0m2.212s
user	0m2.086s
sys	0m0.121s
```

I'm only using stdlib, and using only a single thread. Not too shabby.

## Test Files

I used [enwik8][enwik8] and [enwik9][enwik9] to test.


[haskell]: https://chrispenner.ca/posts/wc
[enwik8]: http://mattmahoney.net/dc/enwik8.zip
[enwik9]: http://mattmahoney.net/dc/enwik9.zip
