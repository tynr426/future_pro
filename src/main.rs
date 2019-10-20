extern crate tokio_core;

use futures::done;
use futures::prelude::*;
use futures::future::{err, ok};
use tokio_core::reactor::Core;
use std::error::Error;
mod error_defined;
use error_defined::{ErrorA,ErrorB};


fn main() {
    let mut reactor=Core::new().unwrap();
    //and_then 级联
    let chain_future=my_fut().and_then(|retval|my_fut_squared(retval));
    let retval = reactor.run(chain_future).unwrap();
    //如果普通的函数返回的是Result，那么可以使用futures提供的done函数，他自动的将Result转化为impl Future
     let chain_future2=my_fut().and_then(|retval|
     {
         done(my_fut_squared2(retval)).and_then(|ret|my_fut_squared(ret))
     });
    let retval2 = reactor.run(chain_future2).unwrap();
    println!("{:?},{:?}",retval,retval2);
    //泛型
    let future = fut_generic_own("Sampo", "jjj");
    let retval3 = reactor.run(future).unwrap();
    println!("fut_generic_own == {}", retval3);

    //错误处理
    let future2= fut_error_a().map_err(|e|{
        ErrorB::default()
    }).and_then(|()| fut_error_b());
    println!("error chain=={:?}",reactor.run(future2).unwrap_err());

}
fn my_fut()->impl Future<Item = u32, Error =Box<Error+'static>> {
    ok(100)
}
fn my_fut_squared(i:u32)->impl Future<Item = u32, Error =Box<Error+'static>> {
    ok(i*i)
}
fn my_fut_squared2(i:u32)->Result<u32,Box<Error+'static>> {
    Ok(i*i)
}
fn fut_generic_own<A> (a1: A, a2: A) -> impl Future<Item = A, Error = Box<Error+'static>> 
where A: std::cmp::PartialOrd
{   
    if a1 < a2 {
        ok(a1)
    }else {
        ok(a2)
    }
}

fn fut_error_a() -> impl Future<Item=(), Error=ErrorA> {
    err(ErrorA{})
}
fn fut_error_b() -> impl Future<Item=(), Error=ErrorB> {
    err(ErrorB{})
}