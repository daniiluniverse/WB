use serde::{Deserialize, Serialize};
use validator_derive::Validate;
extern crate validator_derive;



#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Delivery {
    #[validate(length(min = 1, message = "поле name не может быть пустым"))]
    pub name: String,
    #[validate(length(min = 8, message = "введен слишком короткий номер"))]
    pub phone: String,
    #[validate(length(min = 1, message = "поле zip не может быть пустым"))]
    pub zip: String,
    #[validate(length(min = 1, message = "поле city не может быть пустым"))]
    pub city: String,
    #[validate(length(min = 1, message = "поле address не может быть пустым"))]
    pub address: String,
    #[validate(length(min = 1, message = "поле region не может быть пустым"))]
    pub region: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Payment {
    #[validate(length(min = 1, message = "поле transaction не может быть пустым"))]
    pub transaction: String,
    #[validate(length(min = 1, message = "поле request_id не может быть пустым"))]
    pub request_id: String,
    #[validate(length(min = 1, message = "поле currency не может быть пустым"))]
    pub currency: String,
    #[validate(length(min = 1, message = "поле provider не может быть пустым"))]
    pub provider: String,
    #[validate(range(min = 1, message = "поле amount должно быть больше 0"))]
    pub amount: i32,
    #[validate(range(min = 1, message = "поле payment_dt должно быть больше 0"))]
    pub payment_dt: i32,
    #[validate(length(min = 1, message = "поле region не может быть пустым"))]
    pub bank: String,
    #[validate(range(min = 1, message = "поле delivery_cost должно быть больше 0"))]
    pub delivery_cost: i32,
    #[validate(range(min = 1, message = "поле goods_total должно быть больше 0"))]
    pub goods_total: i32,
    #[validate(range(min = 1, message = "поле custom_fee должно быть больше 0"))]
    pub custom_fee: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Items {
    #[validate(range(min = 1, message = "поле chrt_id должно быть больше 0"))]
    pub chrt_id: i32,
    #[validate(length(min = 1, message = "поле track_number должно быть больше 0"))]
    pub track_number: String,
    #[validate(range(min = 1, message = "поле price должно быть больше 0"))]
    pub price: i32,
    #[validate(length(min = 1, message = "поле rid должно быть больше 0"))]
    pub rid: String,
    #[validate(length(min = 1, message = "поле name не может быть пустым"))]
    pub name: String,
    #[validate(range(min = 1, message = "поле sale должно быть больше 0"))]
    pub sale: i32,
    #[validate(length(min = 1, message = "поле size не может быть пустым"))]
    pub size: String,
    #[validate(range(min = 1, message = "поле total_price должно быть больше 0"))]
    pub total_price: i32,
    #[validate(range(min = 1, message = "поле nm_id должно быть больше 0"))]
    pub nm_id: i32,
    #[validate(length(min = 1, message = "поле brand не может быть пустым"))]
    pub brand: String,
    #[validate(range(min = 1, message = "поле status должно быть больше 0"))]
    pub status: i32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Order{
    #[validate(length(min = 1, message = "поле internal_signature не может быть пустым"))]
    pub locale: String,
    #[validate(length(min = 1, message = "поле internal_signature не может быть пустым"))]
    pub internal_signature: String,
    #[validate(length(min = 1, message = "поле customer_id не может быть пустым"))]
    pub customer_id: String,
    #[validate(length(min = 1, message = "поле delivery_service не может быть пустым"))]
    pub delivery_service: String,
    #[validate(length(min = 1, message = "поле shardkey не может быть пустым"))]
    pub shardkey: String,
    #[validate(range(min = 1, message = "поле sm_id должно быть больше 0"))]
    pub sm_id: i32,
    #[validate(length(min = 1, message = "поле date_created не может быть пустым"))]
    pub date_created: String,
    #[validate(length(min = 1, message = "поле oof_shard не может быть пустым"))]
    pub oof_shard: String,
    #[validate(length(min = 1, message = "поле order_uid не может быть пустым"))]
    pub order_uid: String,
    #[validate(length(min = 1, message = "поле track_number не может быть пустым"))]
    pub track_number: String,
    #[validate(length(min = 1, message = "поле entry не может быть пустым"))]
    pub entry: String,
    pub items: Items,
    pub payment: Payment,
    pub delivery: Delivery,
}


