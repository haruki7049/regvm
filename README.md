# regvm

Simple register machine implementation (counter machine) for my practice.

# Build

```bash
cargo run
```

```txt
[haruki@tuf-chan:~/program-dir/github.com/haruki7049/regvm]$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/regvm`
[src/main.rs:20:5] machine = RegVM {
    pc: 2,
    registers: [
        33,
        0,
    ],
    instructions: [
        Dec {
            r: 1,
            next: 1,
            branch: 2,
        },
        Inc {
            r: 0,
            next: 0,
        },
        Halt,
    ],
}

[haruki@tuf-chan:~/program-dir/github.com/haruki7049/regvm]$
```
