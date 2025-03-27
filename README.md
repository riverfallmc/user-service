![logo](./assets/logo.png)
# User Service
Микросервис, отвечающий за хранение пользователей, систему дружбы, систему приватности, отображение статуса пользователя.

## Содержимое
* [Сборка](#сборка)
* [Деплой](#деплой)
* [Настройка](#настройка)
  * [Переменные окружения](#переменные-окружения)
* [Описание эндпоинтов](#эндпоинты)

# Сборка
Микросервис написан на Rust, так что чтобы\
собрать его вам необходимо лишь установить ``cargo`` на ваш ПК,
и прописать следующую команду

```bash
cargo build --release
```

После успешной сборки вы сможете найти артефакт по этому пути ``./target/release/user_service``.

# Деплой
Команды для деплоя уже есть в нашем [Puff-файле](./puff.yml).

[Узнать подробнее что такое Puff-файл](https://github.com/smokingplaya/puff)

```bash
# Собирает сервис и пушит его в регистр под тегом latest
puff deploy
```

# Настройка

## Переменные окружения
``RUST_LOG: string`` - Уровень логгирования.\
``DATABASE_URL: string`` - URL для подключения к PostgreSQL.
``REDIS_URL: string`` - URL для подключения к Redis.
``WSS_URL: string`` - Домен до сервиса wss (ex. localhost:3000).

# Эндпоинты
<!-- todo -->