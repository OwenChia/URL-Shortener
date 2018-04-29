extern crate redis;
use redis::{Commands, RedisResult};

pub struct RedisClient {
    connection: redis::Connection,
}

impl RedisClient {
    pub fn new(dsn: &str) -> RedisClient {
        let redis_client = redis::Client::open(dsn).unwrap();
        let con = redis_client.get_connection().unwrap();
        RedisClient {
            connection: con,
        }
    }

    pub fn set(&mut self, uri: &String) -> RedisResult<String> {
        let hashed_uri = ::generate_shorturl_8();
        let _: () = try!(self.connection.set(&hashed_uri, uri));
        println!("{} -> {}", uri, hashed_uri);
        Ok(hashed_uri)
    }

    pub fn get(&self, hashed_uri: &String) -> RedisResult<String> {
        let uri: String = try!(self.connection.get(hashed_uri));
        Ok(uri)
    }

    pub fn del(&mut self, hashed_uri: &String) -> RedisResult<()> {
        let _:() = try!(self.connection.del(hashed_uri));
        Ok(())
    }
}

pub fn redis_connect(config: ::config::Config) -> redis::Connection {
    let redis_client = redis::Client::open(&config.redis_connection.unwrap()[..]).unwrap();
    let con = redis_client.get_connection().unwrap();
    con
}

pub fn store_url(con: &redis::Connection, uri: &String) -> redis::RedisResult<String> {
    let hashed_uri = ::generate_shorturl_8();
    let _: () = try!(con.set(&hashed_uri, uri));
    println!("{} -> {}", uri, hashed_uri);
    Ok(hashed_uri)
}

pub fn get_url(con: &redis::Connection, hashed_uri: &String) -> redis::RedisResult<String> {
    let uri: String = try!(con.get(hashed_uri));
    Ok(uri)
}

pub fn delete_url(con: &redis::Connection, hashed_uri: &String) -> redis::RedisResult<()> {
    let _:() = try!(con.del(hashed_uri));
    Ok(())
}

