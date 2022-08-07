# `solana-cum 🍍`

cum library for solana

## why

solana is like a next gen programmable database.
a bunch of cool shit is on it like [pyth] and [zeta markets].

devs were kinda lazy so there is only 4 databases running: devnet, testnet, mainnet, pythnet.
too many people came and solano crashed so they added compute units to triage who can use it.

solana has like a billion cu per second but jump and alameda is using all of them to liquidate your apes bozo 💀💀

so cum (compute unit minimizing) makes your programs run faster and more healthy

  [pyth]: https://pyth.network/
  [zeta markets]: https://zeta.markets/

## example

boring and safe hello world: **20.6 kB size**, **468 compute units**

```rs
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::{entrypoint, msg};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("uwu");
    Ok(())
}
```

cum hello world: **1.2 kB size** (-94.3%), **105 compute units** (-77.6%)

```rs
use solana_cum::{entrypoint, input::InputParser, syscalls::sol_log_str};

entrypoint!(main);

fn main(_input: &mut InputParser) -> u64 {
    sol_log_str("uwu");
    0
}
```

## how

these are the best ways to cum:

- wrap everything in unsafe blocks
- use inline assembly wherever you can
- jerk each other off on ct
- checked math is for virgins
- never deserialize
- add rationalizing comments for your post-nut-clarity-self
- want idiomatic and readable code? get some skill instead

(credit to the eth hoes for teaching me these techniques)

### unchecked math

this is `x.saturating_add(y)`

```
0000000000000138 <greer>
      39        bf 13 00 00 00 00 00 00 r3 = r1
      40        0f 23 00 00 00 00 00 00 r3 += r2
      41        b7 02 00 00 01 00 00 00 r2 = 0x1
      42        2d 31 01 00 00 00 00 00 if r1 > r3 goto +0x1 <LBB1_2>
      43        b7 02 00 00 00 00 00 00 r2 = 0x0
0000000000000160 <LBB1_2>
      44        b7 00 00 00 ff ff ff ff r0 = -0x1
      45        55 02 01 00 00 00 00 00 if r2 != 0x0 goto +0x1 <LBB1_4>
      46        bf 30 00 00 00 00 00 00 r0 = r3
```

this is `x + y`

```
0000000000000138 <greer>
      39        bf 20 00 00 00 00 00 00 r0 = r2
      40        0f 10 00 00 00 00 00 00 r0 += r1
```

### data alignment

data alignment means "address divisible by a power of two".
hardware requires or prefers alignment for certain operations.

the eBPF arch has no such requirements though so we don't need to fight alignment issues.

rustc tries to be smart and "align pointers" (waste space) with _modulos_ and _bit shifts_.

`#[repr(packed)]` fixes this.

### memory mapping

serialization and "backwards-compatibility" is a major cope by dumbass devs who need to fix bugs 🥴

use [`core::mem::transmute`](https://doc.rust-lang.org/core/mem/fn.transmute.html) to alias structs with accounts instead.

### don't panic

`panic` and convenience methods like `.unwrap()` use `std::fmt` which is super bloaty.
just call `abort` and use `.unwrap_unchecked()`.

## broken

### sbfv2 flag

if u get this

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "Executable constructor failed: ElfError(RelativeJumpOutOfBounds(32))"', rbpf-cli/src/main.rs:263:6
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

run this

```shell
printf '\x20' | dd conv=notrunc of=./target/deploy/hello.so bs=1 seek=48
```
