 use crate::models::{Order, Delivery, Items, Payment};
 use std::collections::HashMap;
 use std::error::Error;
 use std::sync::Arc;
 use axum::extract::State;
 use axum::http::StatusCode;
 use axum::Json;
 use deadpool_postgres::Client;
 use log::{error, info};
 use serde_json::json;
 use tokio_postgres::Transaction;
 use validator::Validate;
 use crate::SharedState;

//Обработчик для получения списка заказов
pub(crate) async fn get_status(State(client): State<SharedState>) -> Json<Vec<Order>> {
     info!("Получение статуса заказов");

    // Получаем доступ к клиенту базы данных
    let client = client.lock().await;

    // Запрос всех заказов
    let orders_rows = client.query("SELECT * FROM orders", &[]).await.unwrap();

    info!("Получены заказы из базы данных");

    // Создаем мапу для хранения заказов по order_uid
    let mut orders_map: HashMap<String, Order> = HashMap::new();

    // Заполняем мапу данными из таблицы orders
    for row in orders_rows {
        let order = Order {
            locale: row.get("locale"),
            internal_signature: row.get("internal_signature"),
            customer_id: row.get("customer_id"),
            delivery_service: row.get("delivery_service"),
            shardkey: row.get("shardkey"),
            sm_id: row.get("sm_id"),
            date_created: row.get("date_created"),
            oof_shard: row.get("oof_shard"),
            order_uid: row.get("order_uid"),
            track_number: row.get("track_number"),
            entry: row.get("entry"),
            items: Items {
                chrt_id: 0,
                track_number: String::new(),
                price: 0,
                rid: String::new(),
                name: String::new(),
                sale: 0,
                size: String::new(),
                total_price: 0,
                nm_id: 0,
                brand: String::new(),
                status: 0,
            },
            payment: Payment {
                transaction: String::new(),
                request_id: String::new(),
                currency: String::new(),
                provider: String::new(),
                amount: 0,
                payment_dt: 0,
                bank: String::new(),
                delivery_cost: 0,
                goods_total: 0,
                custom_fee: 0,
            },
            delivery: Delivery {
                name: String::new(),
                phone: String::new(),
                zip: String::new(),
                city: String::new(),
                address: String::new(),
                region: String::new(),
                email: String::new(),
            },
        };

        orders_map.insert(order.order_uid.clone(), order);
    }

    // Получаем данные из остальных таблиц
    let deliveries_rows = client.query("SELECT * FROM deliveries", &[]).await.unwrap();
    let payments_rows = client.query("SELECT * FROM payments", &[]).await.unwrap();
    let items_rows = client.query("SELECT * FROM items", &[]).await.unwrap();

    info!("Получены данные о доставке, платеже и товарах");


    // Обновляем мапу данными о доставке
    for row in deliveries_rows {
        let order_uid: String = row.get("order_uid");
        if let Some(order) = orders_map.get_mut(&order_uid) {
            order.delivery = Delivery {
                name: row.get("name"),
                phone: row.get("phone"),
                zip: row.get("zip"),
                city: row.get("city"),
                address: row.get("address"),
                region: row.get("region"),
                email: row.get("email"),
            };
        }
    }

    // Обновляем мапу данными о платеже
    for row in payments_rows {
        let order_uid: String = row.get("order_uid");
        if let Some(order) = orders_map.get_mut(&order_uid) {
            order.payment = Payment {
                transaction: row.get("transaction"),
                request_id: row.get("request_id"),
                currency: row.get("currency"),
                provider: row.get("provider"),
                amount: row.get("amount"),
                payment_dt: row.get("payment_dt"),
                bank: row.get("bank"),
                delivery_cost: row.get("delivery_cost"),
                goods_total: row.get("goods_total"),
                custom_fee: row.get("custom_fee"),
            };
        }
    }

    // Обновляем мапу данными о товарах
    for row in items_rows {
        let order_uid: String = row.get("order_uid");
        if let Some(order) = orders_map.get_mut(&order_uid) {
            order.items = Items {
                chrt_id: row.get("chrt_id"),
                track_number: row.get("track_number"),
                price: row.get("price"),
                rid: row.get("rid"),
                name: row.get("name"),
                sale: row.get("sale"),
                size: row.get("size"),
                total_price: row.get("total_price"),
                nm_id: row.get("nm_id"),
                brand: row.get("brand"),
                status: row.get("status"),
            };
        }
    }

    // Конвертируем мапу в вектор
    let orders: Vec<Order> = orders_map.into_values().collect();

    info!("Статус заказов успешно получен");

    Json(orders)
}

 pub(crate) async fn add_order(
    State(client): State<SharedState>,
    Json(order): Json<Order>,
) -> Json<serde_json::Value> {

     info!("Добавление нового заказа");

    // Обработка валидации
    if let Err(errors) = order.validate() {
        eprintln!("Ошибка валидации: {:?}", errors);
        return Json(json!({"status":" Ошибка валидации" , "errors": errors.to_string() }));
    }

     // Получаем доступ к клиенту базы данных
    let mut client = client.lock().await;
    // Начинаем транзакцию
    let transaction = client.transaction().await.unwrap();

     info!("Начало транзакции");

    // Вставка данных о заказе
    let order_result = transaction.execute(
        "INSERT INTO orders (order_uid, track_number, entry, locale, internal_signature, customer_id, delivery_service, shardkey, sm_id, date_created, oof_shard)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        &[
            &order.order_uid,
            &order.track_number,
            &order.entry,
            &order.locale,
            &order.internal_signature,
            &order.customer_id,
            &order.delivery_service,
            &order.shardkey,
            &order.sm_id,
            &order.date_created,
            &order.oof_shard,
        ],
    ).await;

    if let Err(err) = order_result {
         error!("Ошибка при добавлении заказа в базу данных: {:?}", err.to_string());
        transaction.rollback().await.unwrap();
        return Json(json!({ "status": "Ошибка при добавлении заказа" }));
    }

    // Вставка данных о доставке
    let delivery_result = transaction.execute(
        "INSERT INTO deliveries (order_uid, name, phone, zip, city, address, region, email)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        &[
            &order.order_uid,
            &order.delivery.name,
            &order.delivery.phone,
            &order.delivery.zip,
            &order.delivery.city,
            &order.delivery.address,
            &order.delivery.region,
            &order.delivery.email,
        ],
    ).await;

    if let Err(err) = delivery_result {
        error!("Ошибка при добавлении данных о доставке в базу данных: {:?}", err);
        transaction.rollback().await.unwrap();
        return Json(json!({ "status": "Ошибка при добавлении заказа" }));
    }

    // Вставка данных о платежах
    let payment_result = transaction.execute(
        "INSERT INTO payments (order_uid, transaction, request_id, currency, provider, amount, payment_dt, bank, delivery_cost, goods_total, custom_fee)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        &[
            &order.order_uid,
            &order.payment.transaction,
            &order.payment.request_id,
            &order.payment.currency,
            &order.payment.provider,
            &order.payment.amount,
            &order.payment.payment_dt,
            &order.payment.bank,
            &order.payment.delivery_cost,
            &order.payment.goods_total,
            &order.payment.custom_fee,
        ],
    ).await;

    if let Err(err) = payment_result {
        error!("Ошибка при добавлении данных о платеже в базу данных: {:?}", err);
        transaction.rollback().await.unwrap();
        return Json(json!({ "status": "Ошибка при добавлении заказа" }));
    }

    // Вставка данных о товарах
    let items_result = transaction.execute(
        "INSERT INTO items (order_uid, chrt_id, track_number, price, rid, name, sale, size, total_price, nm_id, brand, status)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
        &[
            &order.order_uid,
            &order.items.chrt_id,
            &order.items.track_number,
            &order.items.price,
            &order.items.rid,
            &order.items.name,
            &order.items.sale,
            &order.items.size,
            &order.items.total_price,
            &order.items.nm_id,
            &order.items.brand,
            &order.items.status,
        ],
    ).await;

    if let Err(err) = items_result {
         error!("Ошибка при добавлении данных о товарах в базу данных: {:?}", err);
        transaction.rollback().await.unwrap();
        return Json(json!({ "status": "Ошибка при добавлении заказа" }));
    }

    // Завершение транзакции
    transaction.commit().await.unwrap();
     info!("Транзакция завершена успешно");

    Json(json!({ "status": "Заказ успешно добавлен!" }))
}
































