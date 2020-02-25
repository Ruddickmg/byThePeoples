use crate::connection;
use crate::models::playground;
use crate::models::playground::Episode;
use redis::{Client, Commands};

fn get_connection() -> redis::RedisResult<redis::Connection> {
    let client = Client::open(format!("redis://{}/", connection::REDIS_IP))?;
    Ok(client.get_connection()?)
}

pub fn cache_human(human: &playground::Human) -> redis::RedisResult<playground::Human> {
    let mut con = get_connection()?;
    let playground::Human {
        id,
        name,
        appears_in,
        home_planet,
    }: playground::Human = human.clone();
    let films: Vec<String> = human
        .appears_in
        .iter()
        .map(|place| place.to_string())
        .collect();
    let _: () = con.set(format!("{}-name", &id), &human.name)?;
    let _: () = con.set(format!("{}-home_planet", &id), &human.home_planet)?;
    let _: () = con.set(format!("{}-appears_in", &id), films)?;
    Ok(playground::Human {
        id,
        name,
        appears_in,
        home_planet,
    })
}

pub fn get_human_from_cache(id: String) -> redis::RedisResult<playground::Human> {
    let mut con = get_connection()?;
    let name: String = con.get(format!("{}-name", &id))?;
    let home_planet: String = con.get(format!("{}-home_planet", &id))?;
    let films: String = con.get(format!("{}-appears_in", &id))?;
    println!("response {}, {}, : {}", name, home_planet, films);
    let appears_in: Vec<Episode> = vec![films]
        .iter()
        .map(|film| Episode::from_string(film))
        .filter(|episode| match episode {
            Ok(_) => true,
            Err(_) => false,
        })
        .map(|f| f.unwrap())
        .collect();

    println!("response {}, {:?}, {}", name, appears_in, home_planet);

    Ok(playground::Human {
        id,
        name,
        appears_in,
        home_planet,
    })
}
