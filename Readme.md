🛡️ Rust P2P Encrypted Chat
Una herramienta de chat terminal-to-terminal descentralizada, asíncrona y cifrada de extremo a extremo (E2EE), escrita en Rust 🦀.

🚀 Características
Arquitectura P2P: Sin servidores centrales. Conexión directa TCP.

Asíncrono (Non-blocking): Construido sobre Tokio para gestionar I/O de red y teclado simultáneamente sin hilos del SO pesados.

Seguridad E2EE:

Cifrado: ChaCha20 (Stream Cipher).

Autenticación: Poly1305 (MAC).

Gestión de Nonces aleatorios por mensaje para evitar ataques de repetición.

Protocolo: JSON serializado sobre TCP.

🛠️ Instalación y Uso
Necesitas tener Rust instalado.

Bash
git clone https://github.com/TU_USUARIO/p2p_chat.git
cd p2p_chat
cargo run --release
🔐 Cómo iniciar una sesión segura
El chat requiere que ambas partes compartan una Clave Secreta antes de conectar.

Alice (Servidor):

Ejecuta el programa.

Selecciona (1) Generar nueva clave.

Copia la clave generada y envíasela a Bob por un canal seguro (Signal, en persona, etc.).

Selecciona (1) Server y espera.

Bob (Cliente):

Ejecuta el programa.

Selecciona (2) Introducir clave existente.

Pega la clave de Alice.

Selecciona (2) Client e introduce la IP de Alice (ej: 127.0.0.1:8080 o su IP pública).

¡Chat Seguro! * Todo lo que escriban a partir de ahora viaja encriptado. Si un atacante intercepta los paquetes TCP, solo verá ruido binario aleatorio.

🏗️ Stack Tecnológico
tokio: Runtime asíncrono.

serde / serde_json: Serialización de mensajes.

chacha20poly1305: Implementación pura en Rust del algoritmo de cifrado autenticado (AEAD).

base64: Codificación de transporte.