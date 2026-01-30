use colored::Colorize;
use std::error::Error;
use std::io::Write;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

mod crypto;

mod protocol;

use protocol::Message;

// --- CONFIGURACIÓN (Usamos std::io explícitamente para el menú) ---
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🦀 P2P Chat v0.3 - Encrypted");
    println!("-------------------------------");

    // --- PASO 1: CONFIGURACIÓN DE SEGURIDAD (¡LO PRIMERO!) ---
    println!("🔐 Configuración de Seguridad:");
    println!("(1) Generar NUEVA clave (Para ser el primero)");
    println!("(2) Introducir clave EXISTENTE (Si alguien te la pasó)");
    print!("> ");
    std::io::stdout().flush()?;

    let mut key_input = String::new();
    std::io::stdin().read_line(&mut key_input)?;

    // Obtenemos la clave final (ya sea generada o leída)
    let key = match key_input.trim() {
        "1" => {
            let k = crypto::generate_key();
            println!("\n⚠️  COPIA ESTA CLAVE Y PÁSASELA AL OTRO: \n{}\n", k);
            k // Devuelve la clave generada
        }
        "2" => {
            println!("Introduce la clave:");
            print!("> ");
            std::io::stdout().flush()?;
            let mut k = String::new();
            std::io::stdin().read_line(&mut k)?;
            k.trim().to_string() // Devuelve la clave que escribiste
        }
        _ => {
            println!("Opción no válida. Saliendo.");
            return Ok(());
        }
    };

    println!("Introduce tu Username.");
    println!("> ");

    let mut username_input = String::new();
    std::io::stdin().read_line(&mut username_input)?;

    let username = match username_input.trim() {
        "" => {
            println!("Username no válido");
            return Ok(());
        }
        _ => username_input.trim(),
    };

    // --- PASO 2: MODO DE CONEXIÓN ---
    println!("{}", "🌐 Selección de Modo:".yellow());
    println!("(1) Server (Esperar conexión)");
    println!("(2) Client (Conectar a otro)");
    print!("> ");
    std::io::stdout().flush()?;

    let mut mode_input = String::new();
    std::io::stdin().read_line(&mut mode_input)?;

    // AHORA PASAMOS LA CLAVE A LAS FUNCIONES
    match mode_input.trim() {
        // Tienes que modificar run_server y run_client para que acepten este argumento
        "1" => run_server(key, username.to_string()).await?,
        "2" => run_client(key, username.to_string()).await?,
        _ => println!("Opción no válida. Bye."),
    }

    Ok(())
}

async fn run_server(key: String, username: String) -> Result<(), Box<dyn Error>> {
    let addr = "0.0.0.0:8080";
    println!("Listening on {}...", addr);

    let listener = TcpListener::bind(addr).await?;

    // Aceptamos la conexión
    let (socket, addr) = listener.accept().await?;
    println!("New client connected: {:?}", addr);

    // Llamamos a la lógica del chat (la misma para ambos)
    handle_chat(socket, key, username).await?;

    Ok(())
}

async fn run_client(key: String, username: String) -> Result<(), Box<dyn Error>> {
    println!("{}", "Enter Server IP (e.g., 127.0.0.1:8080):".yellow());
    std::io::stdout().flush()?;

    let mut addr = String::new();
    std::io::stdin().read_line(&mut addr)?;
    let addr = addr.trim();

    println!("Connecting to {}...", addr.yellow());

    let socket = TcpStream::connect(addr).await?;
    println!("{}", "Connected!".yellow());

    // Llamamos a la lógica del chat
    handle_chat(socket, key, username).await?;

    Ok(())
}

// --- EL NÚCLEO ASÍNCRONO (Aquí ocurre la magia) ---
async fn handle_chat(
    socket: TcpStream,
    key: String,
    username: String,
) -> Result<(), Box<dyn Error>> {
    // 1. DIVIDE Y VENCERÁS
    // Partimos el socket en dos: uno para leer (reader) y otro para escribir (writer)
    let (reader, mut writer) = socket.into_split();

    let key_reader = key.clone();

    // 2. TAREA EN SEGUNDO PLANO (El Oído)
    // Creamos un hilo verde que SOLO escucha lo que llega de la red
    let mut reader = BufReader::new(reader);

    tokio::spawn(async move {
        let mut line = String::new();
        loop {
            line.clear();
            // Leemos una línea del socket (esperamos mensaje del otro)
            match reader.read_line(&mut line).await {
                Ok(0) => return, // 0 bytes significa que el otro cerró conexión
                Ok(_) => {
                    let encrypted_msg = line.trim();
                    match crypto::decrypt(encrypted_msg, &key_reader) {
                        Ok(json_string) => {
                            if let Some(msg) = Message::from_json(&json_string) {
                                println!("\n [{}] : {}", msg.username.cyan(), msg.content);
                                print!("> ");
                                std::io::stdout().flush().unwrap();
                            }
                        }
                        Err(_) => {
                            println!("{}", "Error de seguridad: No se puede desencriptar".red())
                        }
                    }
                }
                Err(_) => return,
            }
        }
    });

    // 3. TAREA PRINCIPAL
    // Mientras el spawn de arriba escucha, nosotros usamos este hilo
    // para leer el teclado y mandar mensajes.

    let mut stdin = BufReader::new(tokio::io::stdin());
    let mut line = String::new();

    loop {
        line.clear();
        // Leemos del teclado de forma asíncrona
        let bytes = stdin.read_line(&mut line).await?;
        if bytes == 0 {
            break;
        } // Si cerramos el input (Ctrl+D)

        let content = line.trim();

        let msg = Message::new(&username, content);

        let json_to_send = msg.to_json();

        let encrypted_packet = crypto::encrypt(&json_to_send, &key)?;

        // Escribimos al socket (mandamos mensaje al otro)
        writer
            .write_all(format!("{}\n", encrypted_packet).as_bytes())
            .await?;
    }

    Ok(())
}
