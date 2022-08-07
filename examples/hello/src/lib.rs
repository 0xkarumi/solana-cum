use solana_cum::{entrypoint, input::InputParser, syscalls::sol_log_str};

entrypoint!(main);

fn main(_input: &mut InputParser) -> u64 {
    sol_log_str("uwu");
    0
}
