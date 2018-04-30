use actix::prelude::*;
use actix_web::*;
use redis::{Connection, Commands, RedisError};

use utils;

pub struct DbExecutor(pub Connection);

impl Actor for DbExecutor {
    type Context = Context<Self>;
}

#[derive(Debug)]
pub struct StoreUrl {
    pub url: String,
}

impl Message for StoreUrl {
    type Result = Result<String, RedisError>;
}

impl Handler<StoreUrl> for DbExecutor {
    type Result = Result<String, RedisError>;

    fn handle(&mut self, msg: StoreUrl, _: &mut Self::Context) -> Self::Result {
        let hashed_url = utils::generate_shorturl_8();
        let con = &self.0;

        let _: () = try!(con.set(&hashed_url, &msg.url));
        Ok(format!("{:?} -> {:?}", msg, hashed_url))
    }
}

#[derive(Debug)]
pub struct GetUrl {
    pub hashed_url: String,
}

impl Message for GetUrl {
    type Result = Result<String, RedisError>;
}

impl Handler<GetUrl> for DbExecutor {
    type Result = Result<String, RedisError>;

    fn handle(&mut self, msg: GetUrl, _: &mut Self::Context) -> Self::Result {
        let con = &self.0;

        let result: String = try!(con.get(&msg.hashed_url));
        Ok(result)
    }
}

#[derive(Debug)]
pub struct DelUrl {
    pub hashed_url: String,
}

impl Message for DelUrl {
    type Result = Result<String, RedisError>;
}

impl Handler<DelUrl> for DbExecutor {
    type Result = Result<String, RedisError>;

    fn handle(&mut self, msg: DelUrl, _: &mut Self::Context) -> Self::Result {
        let con = &self.0;

        let _:() = try!(con.del(&msg.hashed_url));
        Ok(format!("{:?}", msg))
    }
}
