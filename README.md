# slOS

silly little operating system

## Hosted mode

```shell
% cargo run -p slos-hosted -- kernel
```

Optionally:

- Enabling the `init_examples` feature will do some extra runtime tests, including filesystem device node tests (which print to the console)
- Setting `RUST_LOG=trace` will get tracing messages from everything _except_ the virtual interrupt dispatcher
- Setting `RUST_LOG=trace,slos_hosted::host::interrupts=trace` will get tracing messages from everything

To run the hosted kernel in "I want all the debugging!" mode, this is what you want:

```shell
% env RUST_LOG="trace,slos_hosted::host::interrupts=trace" cargo run -p slos-hosted --features init_examples -- kernel
```

![Screenshot of slOS running in hosted mode](https://oops-all-kittens.sfo2.digitaloceanspaces.com/slos/20210827_03h25m49s_grim.png)

### The hosted debugging REPL

```shell
% cargo run -p slos-hosted -- repl
```

As an example:

```
slos-hosted> help
b: echo
fs: file-read, file-write-test, …

slos-hosted> b echo hello world!
hello world!

slos-hosted> fs mount-new-memoryfs /
slos-hosted> fs mount-list
/ - Some(UnsafeContainer(SimpleMemoryFs { … }))

slos-hosted> fs file-write-test /test
slos-hosted> fs file-read /test
[bytes] [104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33]
[string] "hello world!"

slos-hosted> exit
```

## Bare-metal targets

None yet! Coming soon.

## License

[Simplified BSD License, with a patent grant](./LICENSE)
