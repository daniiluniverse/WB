WB Tech: level # 0 


Необходимо разработать демонстрационный сервис с простейшим интерфейсом, возвращающий данные о заказе.

Требования
1. Заказы должны быть иммутабельны (не меняются после создания, только добавляются). Исходя из этого, подумайте насчет модели хранения в кэше и в PostgreSQL. Модель в файле model.json

2. Подумайте как избежать проблем, связанных с тем, что в ручку (http-endpoint) могут закинуть что-угодно

3. Для тестирования сделайте себе отдельный скрипт для публикации данных через API
 
4. Подумайте, как не терять данные в случае ошибок или проблем с сервисом
 

Запуск
============
Для запуска окружение введите команду:

```bash
$env:RUST_LOG="debug"; cargo run --bin L0_WB
```

Для создание тестового запроса введите команду:

```bash
cargo run --bin test_api
```    

Запроса для получаения данных:

```http
http://127.0.0.1:3000/
```

Ответ:

```json
{
        "locale": "ru",
        "internal_signature": "hhjjnkj",
        "customer_id": "cust_001",
        "delivery_service": "DHL",
        "shardkey": "3232",
        "sm_id": 10,
        "date_created": "2023-09-02T00:00:00Z",
        "oof_shard": "NKN;lsd",
        "order_uid": "5",
        "track_number": "TRACK123",
        "entry": "Super",
        "items": {
            "chrt_id": 4,
            "track_number": "TRACK7923",
            "price": 10000,
            "rid": "RID4223",
            "name": "Спортивный костюм",
            "sale": 15,
            "size": "М",
            "total_price": 85,
            "nm_id": 11,
            "brand": "Адидас",
            "status": 2
        },
        "payment": {
            "transaction": "ok",
            "request_id": "iwelksd",
            "currency": "sdsdg",
            "provider": "Ростелеком",
            "amount": 99,
            "payment_dt": 12358668,
            "bank": "Сбер",
            "delivery_cost": 23,
            "goods_total": 54,
            "custom_fee": 1
        },
        "delivery": {
            "name": "web",
            "phone": "en",
            "zip": "sweew",
            "city": "Воронеж",
            "address": "Мичурина 76",
            "region": "Центральный",
            "email": "вывывы"
        }
```
