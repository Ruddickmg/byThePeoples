use std::env;

const HASH_SECRET: &str = "HASH_SECRET";
const ARGON_LANES: &str = "ARGON_LANES";
const ARGON_TIME_COST: &str = "ARGON_TIME_COST";
const ARGON_MEMORY: &str = "ARGON_MEMORY";

type ArgonNumericInput = u32;

pub fn secret() -> String {
    String::from(match env::var(HASH_SECRET) {
        Ok(secret) => secret,
        Err(_) => String::from(""),
    })
}

pub fn lanes() -> ArgonNumericInput {
    match env::var(ARGON_LANES) {
        Ok(lanes) => lanes.parse::<ArgonNumericInput>().unwrap(),
        Err(_) => 8,
    }
}

pub fn time_cost() -> ArgonNumericInput {
    match env::var(ARGON_TIME_COST) {
        Ok(time_cost) => time_cost.parse::<ArgonNumericInput>().unwrap(),
        Err(_) => 10,
    }
}

pub fn memory_usage() -> ArgonNumericInput {
    match env::var(ARGON_MEMORY) {
        Ok(memory) => memory.parse::<ArgonNumericInput>().unwrap(),
        Err(_) => 2048,
    }
}
