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
