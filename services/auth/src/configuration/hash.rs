const HASH_SECRET: &str = "HASH_SECRET";
const ARGON_LANES: &str = "ARGON_LANES";
const ARGON_TIME_COST: &str = "ARGON_TIME_COST";
const ARGON_MEMORY: &str = "ARGON_MEMORY";

type ArgonNumericInput = u32;

pub fn secret() -> String {
    String::from(environment::env_or_default(HASH_SECRET, "secret"))
}

pub fn lanes() -> ArgonNumericInput {
    environment::env_or_default(ARGON_LANES, 8)
        .parse::<ArgonNumericInput>()
        .unwrap()
}

pub fn time_cost() -> ArgonNumericInput {
    environment::env_or_default(ARGON_TIME_COST, 10)
        .parse::<ArgonNumericInput>()
        .unwrap()
}

pub fn memory_usage() -> ArgonNumericInput {
    environment::env_or_default(ARGON_MEMORY, 2048)
        .parse::<ArgonNumericInput>()
        .unwrap()
}
