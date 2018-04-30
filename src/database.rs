use redis;
use redis::{Commands, RedisResult};
use actix_web::*;
use actix::prelude::*;

use utils;

pub struct RedisClient {
    pub connection: redis::Connection,
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
        let hashed_uri = utils::generate_shorturl_8();
        let _: () = try!(self.connection.set(&hashed_uri, uri));
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

pub struct DbExecutor(pub redis::Connection);

impl Actor for DbExecutor {
    type Context = Context<Self>;
}

#[derive(Debug)]
pub struct StoreUrl {
    pub url: String,
}

impl Message for StoreUrl {
    type Result = Result<String, Error>;
}

impl Handler<StoreUrl> for DbExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: StoreUrl, _: &mut Self::Context) -> Self::Result {
        let hashed_uri = utils::generate_shorturl_8();
        let _: () = self.0.set(&hashed_uri, &msg.url).unwrap();
        Ok(format!("{:?}", msg))
    }
}
