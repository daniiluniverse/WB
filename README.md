

Задание LO
================

Необходимо разработать демонстрационный сервис с простейшим интерфейсом, отображающий данные о заказе. 

Требования

1.  Заказы должны быть иммутабельны (не меняются после создания, только добавляются). Исходя из этого, подумайте насчет модели хранения в кэше и в PostgreSQL. Модель в файле model.json

2.  Подумайте как избежать проблем, связанных с тем, что в ручку (http-endpoint) могут закинуть что-угодно

3.  Для тестирования сделайте себе отдельный скрипт для публикации данных через API

4.  Подумайте, как не терять данные в случае ошибок или проблем с сервисом



Запуск 
============
Запуск окружения

```bash
$env:RUST_LOG="debug"; cargo run L0_WB  Windows
RUST_LOG=debug cargo run L0_WB  Mac

```

Тестовый запрос POST

```bash
cargo run --bin test_api
```


Пример запроса на получение данных

```http
GET http://127.0.0.1:3000/
```

Ответ

```json
{
    "locale": "ru",
    "internal_signature": "sweew",
    "customer_id": "cust_001",
    "delivery_service": "DHL",
    "shardkey": "key_001",
    "sm_id": 66,
    "date_created": "2023-09-03T12:00:00Z",
    "oof_shard": "shard_001",
    "order_uid": "555",
    "track_number": "TRACK123",
    "entry": "web",
    "items": {
      "chrt_id": 123,
      "track_number": "TRACK123",
      "price": 1000,
      "rid": "RID123",
      "name": "Товар Имя",
      "sale": 10,
      "size": "M",
      "total_price": 900,
      "nm_id": 456,
      "brand": "Бренд",
      "status": 404
    },
    "payment": {
      "transaction": "TRANS123",
      "request_id": "REQ123",
      "currency": "USD",
      "provider": "VISA",
      "amount": 900,
      "payment_dt": 1633036800,
      "bank": "Название банка",
      "delivery_cost": 50,
      "goods_total": 950,
      "custom_fee": 0
    },
    "delivery": {
      "name": "John Doe",
      "phone": "+1234567890",
      "zip": "12345",
      "city": "Город",
      "address": "123 Main St",
      "region": "Регион",
      "email": "email@example.com"
    }
```
