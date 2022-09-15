#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

use std::collections::HashMap;

use volo_gen::volotodo::{Item, ItemService, GetItemRequest, GetItemResponse};

pub struct S;

#[volo::async_trait]
impl ItemService for S {
    // 这部分是我们需要增加的代码
    async fn get_item(
        &self,
        _req: volo_grpc::Request<GetItemRequest>,
    ) -> core::result::Result<volo_grpc::Response<GetItemResponse>, volo_grpc::Status>
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
        Ok(volo_grpc::Response::new(GetItemResponse { item: it.into() }))
    }
}