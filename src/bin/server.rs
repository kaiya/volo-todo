#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

use std::net::SocketAddr;

use volo_todo::{S};

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    volo_gen::volotodo::ItemServiceServer::new(S)
        .run(addr)
        .await
        .unwrap();
}
