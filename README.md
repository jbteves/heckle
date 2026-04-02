# heckle

A meme case conversion library.

`heckle` exists to facilitate lulz with meme cases, such as spONgEBoB caSE and BILLY MAYS MODE. It is intended to be unicode-aware, internally consistent-ish, and reasonably useless.

## Cases

### sPoNGeBoB CAsE

Randomly alternates letter casing with no more than 3 consecutive same-case characters. If you want more than 3 characters, too bad, it's hardcoded.

Also it's not really random because the RNG is always seeded with 42. That's the answer to life, the universe, and everything, and also the answer to meme case RNG seeding.

```rust
use heckle::ToSpongebobCase;
assert_eq!("hello world".to_spongebob_case(), "HELlo wORLd");
```

### BILLY MAYS MODE

Uppercases every character that has a case, dropping non-alphabetic, non-whitespace characters. BUT WAIT! you might say; well, too bad, no more.

```rust
use heckle::ToBillyMaysMode;
assert_eq!("wait, there's more!!!".to_billy_mays_mode(), "WAIT THERES MORE");
```

## Word boundaries

Unlike the popular and useful [`heck`](https://github.com/withoutboats/heck) library, `heckle` does not care about word boundaries. In BILLY MAYS MODE, non-alphabetic, non-whitespace characters are dropped entirely. In Spongebob Case, only alphabetic characters with a real case mapping participate in the run counter — everything else passes through unchanged.

## Performance

Benchmarks compare `heckle` against [`heck`](https://github.com/withoutboats/heck) as a baseline. `heck` is a mature, widely-used case-conversion library with a similar char-iteration / `String`-building profile, making it a reasonable reference point even though the two libraries perform different operations.

### Methodology

Benchmarks are implemented with [Criterion.rs](https://github.com/bheisler/criterion.rs) (`benches/conversions.rs`). Each conversion is run against five representative inputs:

| Label | Description | Length |
|---|---|---|
| `short` | `"hello, world!"` | 13 chars |
| `medium` | pangram with trailing punctuation | 90 chars |
| `punctuation_heavy` | punctuation interspersed throughout every word | 69 chars |
| `long` | pangram repeated 222× | ~9,990 chars |
| `multiline` | multi-sentence, multi-line block repeated 50× | ~3,400 chars |

Three `heck` conversions are included as baselines: `to_snake_case`, `to_upper_camel_case`, and `to_shouty_snake_case`.

To reproduce:

```
cargo bench
```

### Results

> **Machine:** MacBook M1 Pro. Results will vary on other hardware. The relative ordering between conversions is expected to be stable, but absolute numbers and ratios may differ on x86 or under different memory pressure.

Median wall-clock time per call:

| Input | `spongebob_case` | `billy_mays_mode` | `heck` snake | `heck` upper_camel | `heck` shouty_snake |
|---|---|---|---|---|---|
| short | 175 ns | **112 ns** | 183 ns | 187 ns | 195 ns |
| medium | 1,124 ns | **709 ns** | 1,281 ns | 1,237 ns | 1,298 ns |
| punctuation_heavy | 470 ns | **305 ns** | 739 ns | 618 ns | 694 ns |
| long | 122 µs | **74 µs** | 129 µs | 119 µs | 127 µs |
| multiline | 37.4 µs | **25.8 µs** | 41.9 µs | 40.9 µs | 41.4 µs |

### Analysis

**`billy_mays_mode` is the fastest conversion across every input**, running ~35–40% faster than `heck` on typical inputs. The reasons are structural: Billy Mays is a simple filter-and-uppercase pass with no word-boundary detection and no case-transition state machine. The `pending_space` trick (deferring whitespace emission rather than trimming after the fact) means the hot loop is a single branch per character with no backtracking.

**`spongebob_case` is roughly on par with `heck`**, within ~5–10% in either direction. The main extra cost over `heck`'s boundary detection is one `gen_bool(0.5)` RNG call per alphabetic character. The thread-local `SmallRng` access is cheap, but not free.

**Punctuation-heavy input is where the gap is largest.** On `punctuation_heavy`, `billy_mays_mode` (305 ns) is 57% faster than `heck` snake (739 ns), and even `spongebob_case` (470 ns) beats all three `heck` variants. `heck` must track word-boundary state across every punctuation character. Both `heckle` conversions discard non-alphabetic, non-whitespace characters in a single branch and move on, so punctuation is essentially free.

## A Note on AI Usage

This is pure vibe-coded slop. Thanks, Claude!
