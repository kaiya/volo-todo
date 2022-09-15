#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

use std::{collections::HashMap};

use volo_gen::volotodo::{Item, self};

pub struct S;

#[volo::async_trait]
impl volo_gen::volotodo::ItemService for S {
    // 这部分是我们需要增加的代码
    async fn get_item(
        &self,
        _req: volo_grpc::Request<volo_gen::volotodo::GetItemRequest>,
    ) -> core::result::Result<volo_grpc::Response<volo_gen::volotodo::GetItemResponse>, volo_grpc::Status>
    {
        let mut m = HashMap::new();
        m.insert("key".to_string(), "value".to_string());
        let it = Item{
            id: 1,
            title: String::from("kaiya"),
            content: String::from("content"),
            // extra: HashMap::new(),
            extra: m,
        };
        Ok(volo_grpc::Response::new(volotodo::GetItemResponse { item: it.into() }))
    }
}