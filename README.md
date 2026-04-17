🇷🇺 Русская версия
digital-wm-core

Core backend системы Digital World Medicine

digital-wm-core — это центральный сервер (ядро) экосистемы Digital World Medicine, построенный на Rust (Axum). Он предназначен для управления API, интеграций сервисов, Telegram-ботов и медицинских вычислительных модулей.

🚀 Назначение проекта

Проект выполняет роль центрального backend-ядра, которое объединяет все сервисы экосистемы:

FDT (фотодинамическая терапия)
SMP (медицина катастроф / скорая помощь)
PANIC (система кризисной поддержки)
Diagnostic Worker (вычислительные медицинские процессы)
⚙️ Основные возможности
Высокопроизводительный асинхронный сервер на Rust
REST API через Axum
Поддержка интеграции с микросервисами
Готовность к Telegram webhook архитектуре
Лёгкая масштабируемость под distributed system
Основа для медицинских AI/аналитических систем
🧬 Архитектура

digital-wm-core выступает как:

API Gateway
Оркестратор сервисов
Точка входа для внешних клиентов
Центр маршрутизации данных
🛠 Технологии
Rust
Axum
Tokio
Serde
▶️ Запуск
cargo run

Сервер запускается по адресу:

http://0.0.0.0:8080
🔌 Пример API
Echo endpoint
POST /echo
Content-Type: application/json
{
  "message": "hello"
}

Ответ:

{
  "message": "hello"
}
📌 Видение проекта

digital-wm-core — это не просто сервер.

Это ядро медицинской цифровой системы, которое в будущем объединяет:

диагностику
автоматизацию медицинских процессов
кризисные сценарии
интеллектуальные ассистенты
🤝 Автор

MSL72Rph
© Digital World Medicine

🇬🇧 English version
digital-wm-core

Core backend system of Digital World Medicine

digital-wm-core is the central backend engine of the Digital World Medicine ecosystem, built with Rust (Axum). It is designed to power APIs, service integrations, Telegram bots, and medical computation modules.

🚀 Project purpose

This project acts as a central backend core that connects all ecosystem services:

FDT (photodynamic therapy systems)
SMP (emergency medicine / disaster response)
PANIC (crisis support system)
Diagnostic Worker (medical computation engine)
⚙️ Key features
High-performance async Rust server
REST API built with Axum
Microservice integration ready
Telegram webhook compatibility
Scalable distributed architecture
Foundation for medical AI and analytics systems
🧬 Architecture role

digital-wm-core functions as:

API Gateway
Service orchestrator
External entry point
Data routing hub
🛠 Technologies
Rust
Axum
Tokio
Serde
▶️ Run
cargo run

Server runs at:

http://0.0.0.0:8080
🔌 API example
Echo endpoint
POST /echo

Request:

{
  "message": "hello"
}

Response:

{
  "message": "hello"
}
📌 Vision

digital-wm-core is not just a backend.

It is a core engine for a digital medical ecosystem that will eventually support:

diagnostics
medical automation
emergency response systems
intelligent assistants
🤝 Author

MSL72Rph
© Digital World Medicine
