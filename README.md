WB Tech: level # 0 (Golang)

Тестовое задание
================

Необходимо разработать демонстрационный сервис с простейшим интерфейсом, отображающий данные о заказе. [Модель данных в формате JSON](https://drive.google.com/file/d/1rrA7SJUoaGQwDriyY56MAeLT0J_OQkZF/view?usp=sharing) прилагается к заданию.

Что нужно сделать:

1.  Развернуть локально PostgreSQL

1.  Создать свою БД

2.  Настроить своего пользователя

3.  Создать таблицы для хранения полученных данных

3.  Разработать сервис

1.  Реализовать подключение и подписку на канал в nats-streaming

2.  Полученные данные записывать в БД

3.  Реализовать кэширование полученных данных в сервисе (сохранять in memory)

4.  В случае падения сервиса необходимо восстанавливать кэш из БД

5.  Запустить http-сервер и выдавать данные по id из кэша

5.  Разработать простейший интерфейс отображения полученных данных по id заказа

Советы
------

1.  Данные статичны, исходя из этого подумайте насчет модели хранения в кэше и в PostgreSQL. Модель в файле model.json

2.  Подумайте как избежать проблем, связанных с тем, что в канал могут закинуть что-угодно

3.  Чтобы проверить работает ли подписка онлайн, сделайте себе отдельный скрипт, для публикации данных в канал

4.  Подумайте как не терять данные в случае ошибок или проблем с сервисом

5.  Nats-streaming разверните локально (не путать с Nats)


Запуск 
============
Запуск окружения

```bash
docker compose up -d
```

Запуск сервиса

```bash
go run cmd/app/main.go
```

Отправка заказа в NATS

```bash
go run cmd/order-generator/main.go
```

Пример запроса на получение данных

```http
GET http://localhost:8081/order/84c58a96-9d02-11ee-8290-eedf1aa1603b
```

Ответ

```json
{
    "customer_id": "test",
    "date_created": "2021-11-26T06:22:19Z",
    "delivery": {
        "address": "Ploshad Mira 15",
        "city": "Kiryat Mozkin",
        "email": "test@gmail.com",
        "name": "Test Testov",
        "phone": "+9720000000",
        "region": "Kraiot",
        "zip": "2639809"
    },
    "delivery_service": "meest",
    "entry": "WBIL",
    "internal_signature": "",
    "items": [
        {
            "brand": "Vivienne Sabo",
            "chrt_id": 9934930,
            "name": "Mascaras",
            "nm_id": 2389212,
            "price": 453,
            "rid": "ab4219087a764ae0btest",
            "sale": 30,
            "size": "0",
            "status": 202,
            "total_price": 317,
            "track_number": "WBILMTESTTRACK"
        }
    ],
    "locale": "en",
    "oof_shard": "1",
    "order_uid": "b563feb7b2b84b6test",
    "payment": {
        "amount": 1817,
        "bank": "alpha",
        "currency": "USD",
        "custom_fee": 0,
        "delivery_cost": 1500,
        "goods_total": 317,
        "payment_dt": 1637907727,
        "provider": "wbpay",
        "request_id": "",
        "transaction": "b563feb7b2b84b6test"
    },
    "shardkey": "9",
    "sm_id": 99,
    "track_number": "WBILMTESTTRACK"
}
```
