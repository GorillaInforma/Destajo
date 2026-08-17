# Destajo Calzado API

API REST en Rust para registro de pago por destajo en fábrica de calzado.

## Stack

- **Rust** + **Axum** (servidor web async)
- **SQLite** via **sqlx** (base de datos local)
- **Fly.io** (deploy)

## Correr localmente (Termux)

```bash
# 1. Instalar Rust si no lo tienes
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Instalar SQLite
pkg install sqlite

# 3. Copiar variables de entorno
cp .env.example .env

# 4. Compilar y correr
cargo run
```

El servidor corre en `http://localhost:3000`

## Endpoints

### Trabajadores
| Método | Ruta | Descripción |
|--------|------|-------------|
| GET | `/trabajadores` | Listar todos |
| POST | `/trabajadores` | Crear trabajador |
| GET | `/trabajadores/:id` | Obtener uno |
| PUT | `/trabajadores/:id` | Activar/desactivar |

**Crear trabajador:**
```json
POST /trabajadores
{ "nombre": "María López" }
```

### Operaciones (tipos de trabajo)
| Método | Ruta | Descripción |
|--------|------|-------------|
| GET | `/operaciones` | Listar todas |
| POST | `/operaciones` | Crear operación |
| PUT | `/operaciones/:id` | Actualizar tarifa |

**Crear operación:**
```json
POST /operaciones
{ "nombre": "Costura lateral", "tarifa": "3.50" }
```

### Registros de producción
| Método | Ruta | Descripción |
|--------|------|-------------|
| GET | `/registros` | Listar (con filtros) |
| POST | `/registros` | Registrar producción |
| DELETE | `/registros/:id` | Eliminar registro |

**Registrar producción:**
```json
POST /registros
{
  "trabajador_id": "uuid-aqui",
  "operacion_id": "uuid-aqui",
  "cantidad": 45,
  "fecha": "2026-08-16",
  "notas": null
}
```

**Filtrar registros:**
```
GET /registros?fecha=2026-08-16
GET /registros?trabajador_id=xxx
GET /registros?fecha=2026-08-16&trabajador_id=xxx
```

### Reportes
| Método | Ruta | Descripción |
|--------|------|-------------|
| GET | `/reportes/dia?fecha=YYYY-MM-DD` | Producción del día |
| GET | `/reportes/quincena?desde=YYYY-MM-DD&hasta=YYYY-MM-DD` | Quincena |

**Respuesta de reporte:**
```json
[
  {
    "trabajador_id": "uuid",
    "nombre": "María López",
    "total_piezas": 234,
    "total_pago": "819.00"
  }
]
```

## Deploy en Fly.io

```bash
# Instalar flyctl
curl -L https://fly.io/install.sh | sh

# Login
fly auth login

# Primera vez
fly launch

# Crear volumen para SQLite
fly volumes create destajo_data --size 1

# Deploy
fly deploy

# Ver logs
fly logs
```

## Estructura del proyecto

```
destajo/
├── src/
│   ├── main.rs          # Entry point, router, CORS
│   ├── db.rs            # Init de tablas SQLite
│   ├── models.rs        # Structs de datos
│   ├── errors.rs        # Manejo de errores HTTP
│   └── routes/
│       ├── trabajadores.rs
│       ├── operaciones.rs
│       ├── registros.rs
│       └── reportes.rs
├── Cargo.toml
├── Dockerfile
└── fly.toml
```
