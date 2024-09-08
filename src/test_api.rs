use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio;


#[derive(Serialize, Deserialize, Debug)]
struct Order {
    order_uid: String,
    track_number: String,
    entry: String,
    locale: String,
    internal_signature: String,
    customer_id: String,
    delivery_service: String,
    shardkey: String,
    sm_id: i32,
    date_created: String,
    oof_shard: String,
    items: Items,
    payment: Payment,
    delivery: Delivery,
}

#[derive(Serialize, Deserialize, Debug)]
struct Items {
    chrt_id: i32,
    track_number: String,
    price: i32,
    rid: String,
    name: String,
    sale: i32,
    size: String,
    total_price: i32,
    nm_id: i32,
    brand: String,
    status: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Payment {
    transaction: String,
    request_id: String,
    currency: String,
    provider: String,
    amount: i32,
    payment_dt: i64,
    bank: String,
    delivery_cost: i32,
    goods_total: i32,
    custom_fee: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Delivery {
    name: String,
    phone: String,
    zip: String,
    city: String,
    address: String,
    region: String,
    email: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = Client::new();

    // Пример заказа для отправки
    let order = Order {
        order_uid: "testOrder999".to_string(),
        track_number: "TRACK123".to_string(),
        entry: "web".to_string(),
        locale: "en".to_string(),
        internal_signature: "sweew".to_string(),
        customer_id: "cust_001".to_string(),
        delivery_service: "DHL".to_string(),
        shardkey: "key_001".to_string(),
        sm_id: 585,
        date_created: "2023-09-03T12:00:00Z".to_string(),
        oof_shard: "shard_001".to_string(),
        items: Items {
            chrt_id: 123,
            track_number: "TRACK123".to_string(),
            price: 1000,
            rid: "RID123".to_string(),
            name: "Item Name".to_string(),
            sale: 10,
            size: "M".to_string(),
            total_price: 900,
            nm_id: 456,
            brand: "Brand Name".to_string(),
            status: 404,
        },
        payment: Payment {
            transaction: "TRANS123".to_string(),
            request_id: "REQ123".to_string(),
            currency: "USD".to_string(),
            provider: "VISA".to_string(),
            amount: 900,
            payment_dt: 1633036800,
            bank: "Bank Name".to_string(),
            delivery_cost: 50,
            goods_total: 950,
            custom_fee: 0,
        },
        delivery: Delivery {
            name: "John Doe".to_string(),
            phone: "+1234567890".to_string(),
            zip: "12345".to_string(),
            city: "City".to_string(),
            address: "123 Main St".to_string(),
            region: "Region".to_string(),
            email: "email@example.com".to_string(),
        },
    };

    // Отправка POST-запроса для добавления заказа
    let response = client
        .post("http://127.0.0.1:3000/add_order")
        .json(&order)
        .send()
        .await?;

    // Проверка статуса ответа
    if response.status().is_success() {
        let resp_json: serde_json::Value = response.json().await?;
        println!(": {:?}", resp_json);
    } else {
        eprintln!("Ошибка при отправке запроса: {:?}", response.text().await?);
    }

    Ok(())
}
