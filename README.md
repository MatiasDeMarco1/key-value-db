# key-value-db

Una base de datos clave-valor simple escrita en Rust, con persistencia en disco y soporte para múltiples clientes concurrentes a través de TCP.

Proyecto hecho como práctica personal de Rust, aplicando ownership, manejo de errores, concurrencia y diseño modular.

## Features

- Comandos: `SET`, `GET`, `DELETE`, `EXIT`
- Persistencia mediante un append-only log (`db.log`): los datos sobreviven a un reinicio del servidor
- Servidor TCP: múltiples clientes pueden conectarse y operar sobre la misma base de datos en simultáneo
- Manejo de errores robusto: una conexión fallida o mal cerrada no tira abajo el servidor
- Cobertura de tests unitarios sobre el parser de comandos y la lógica de ejecución

## Cómo correrlo

```bash
cargo run
```

El servidor queda escuchando en `127.0.0.1:7878`.

## Cómo usarlo

Conectate con `netcat` (o `telnet`) desde otra terminal:

```bash
nc localhost 7878
```

Y mandá comandos:

SET nombre juan
Se insertó correctamente
GET nombre
Get: juan
DELETE nombre
Deleted juan
EXIT
Exit...


Podés abrir varias terminales conectadas al mismo tiempo — todas comparten la misma base de datos.

## Decisiones técnicas

- **Persistencia con append-only log**: en vez de reescribir todo el archivo en cada operación, cada `SET`/`DELETE` se agrega como una línea nueva. Al arrancar, el servidor reproduce el log completo para reconstruir el estado en memoria. Es el mismo principio que usan bases de datos reales (write-ahead log).
- **Concurrencia con `Arc<Mutex<HashMap>>`**: cada conexión se atiende en su propio thread (`std::thread::spawn`). El estado se comparte de forma segura con `Arc` (múltiples dueños) y `Mutex` (exclusión mutua). El lock se pide y libera en cada comando, no una vez por conexión, para no bloquear a otros clientes innecesariamente.
- **Control de flujo del loop con `std::ops::ControlFlow`**: en vez de un booleano ambiguo, se usa `ControlFlow<String, String>` para indicar explícitamente si la conexión debe seguir (`Continue`) o cerrarse (`Break`), llevando además el mensaje de respuesta en ambos casos.
- **Manejo de errores por conexión**: fallas de red (desconexión de un cliente, error de escritura) cortan solo esa conexión particular, sin afectar a los demás clientes ni al servidor.

## Tests

```bash
cargo test
```

Cubre el parser de comandos (casos válidos, case-insensitive, argumentos incorrectos) y la lógica de ejecución (`SET`, `GET`, `DELETE`, `EXIT`).

## Estructura del proyecto

src/
├── main.rs # arranque del servidor, manejo de conexiones y threads
├── command.rs # enum Command y parsing de texto a comandos
└── db.rs # lógica de ejecución de comandos sobre el HashMap
