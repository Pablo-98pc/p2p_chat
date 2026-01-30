# 🦀 Rust P2P Encrypted Chat

> **Chat de terminal a terminal, asíncrono y cifrado de extremo a extremo (E2EE).**

![Rust](https://img.shields.io/badge/Language-Rust-orange)
![Async](https://img.shields.io/badge/Async-Tokio-green)
![Security](https://img.shields.io/badge/Encryption-ChaCha20Poly1305-blue)

Una herramienta de mensajería instantánea descentralizada construida para demostrar el poder de la programación de sistemas moderna con Rust. Combina redes de bajo nivel (TCP), concurrencia asíncrona de alto rendimiento y criptografía moderna.

---

## 🚀 Características Principales

* **⚡ Arquitectura Asíncrona:** Construido sobre el runtime `Tokio`. Gestiona I/O de red y entrada de teclado simultáneamente sin bloquear hilos del sistema operativo.
* **🔒 Seguridad E2EE (End-to-End Encryption):**
    * Algoritmo: **ChaCha20-Poly1305** (Authenticated Encryption).
    * Cada mensaje utiliza un **Nonce único** de 96-bits para prevenir ataques de repetición.
    * Ni siquiera los metadatos (como el nombre de usuario) viajan en texto plano.
* **📡 Protocolo Personalizado:** Mensajería estructurada en JSON binario sobre streams TCP puros.
* **🤝 P2P Real:** Sin servidores centrales ni bases de datos. Comunicación directa socket-a-socket.

---

## 🛠️ Instalación

Asegúrate de tener [Rust & Cargo](https://www.rust-lang.org/) instalados.

1. Clona el repositorio:
   ```bash
   git clone [https://github.com/TU_USUARIO/p2p_chat.git](https://github.com/TU_USUARIO/p2p_chat.git)
   cd p2p_chat


## 📖 Guía de Uso

Para que dos personas se comuniquen de forma segura, deben compartir una **Clave Secreta** antes de iniciar la conexión.

### Paso 1: Generar la Clave (Alice)
Uno de los dos usuarios debe generar la clave maestra.

1.  Ejecuta el programa:
    ```bash
    cargo run --release
    ```
2.  Selecciona la opción **`(1) Generar NUEVA clave`**.
3.  Copia la cadena de texto generada (ej: `aW83nKl...==`).
4.  **Envíale esta clave a Bob** por un canal seguro (Signal, en persona, USB, etc.).

### Paso 2: Iniciar el Servidor (Alice)
Una vez compartida la clave, Alice se prepara para recibir la conexión:

1.  El programa te preguntará el modo. Elige **`(1) Server`**.
2.  El chat mostrará: `Listening on 0.0.0.0:8080...` y esperará.

### Paso 3: Conectar el Cliente (Bob)
Bob usa la clave que le dio Alice para conectarse:

1.  Ejecuta el programa.
2.  Selecciona la opción **`(2) Introducir clave EXISTENTE`**.
3.  Pega la clave exacta que generó Alice.
4.  Elige el modo **`(2) Client`**.
5.  Introduce la dirección IP de Alice.
    * Si estáis en la misma red WiFi: Usa la IP local (ej: `192.168.1.XX:8080`).
    * Si es en el mismo PC: Usa `127.0.0.1:8080`.

**¡Listo!** 🎉 Todo lo que escribáis a partir de este momento viaja encriptado y firmado.

## 🏗️ Stack Tecnológico

Este proyecto utiliza las librerías más robustas del ecosistema Rust:

| Crate | Uso |
| :--- | :--- |
| **`tokio`** | Runtime asíncrono para networking y gestión de tareas (Green threads). |
| **`chacha20poly1305`** | Implementación pura en Rust del algoritmo de cifrado autenticado (AEAD). |
| **`serde` / `serde_json`** | Serialización y deserialización eficiente de mensajes estructurados. |
| **`base64`** | Codificación segura para el transporte de claves y datos cifrados (texto-friendly). |
| **`anyhow`** | Gestión idiomática y robusta de errores. |