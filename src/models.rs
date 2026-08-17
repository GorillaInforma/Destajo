use serde::{Deserialize, Serialize};

// ── Trabajador ──────────────────────────────────────────────
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Trabajador {
    pub id: String,
    pub nombre: String,
    pub activo: bool,
    pub creado_en: String,
}

#[derive(Debug, Deserialize)]
pub struct NuevoTrabajador {
    pub nombre: String,
}

// ── Operación ───────────────────────────────────────────────
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Operacion {
    pub id: String,
    pub nombre: String,
    pub tarifa: String,  // ej: "12.50"
    pub activa: bool,
}

#[derive(Debug, Deserialize)]
pub struct NuevaOperacion {
    pub nombre: String,
    pub tarifa: String,
}

// ── Registro de producción ──────────────────────────────────
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Registro {
    pub id: String,
    pub trabajador_id: String,
    pub operacion_id: String,
    pub cantidad: i64,
    pub total: String,
    pub fecha: String,
    pub notas: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NuevoRegistro {
    pub trabajador_id: String,
    pub operacion_id: String,
    pub cantidad: i64,
    pub fecha: String,
    pub notas: Option<String>,
}

// ── Reporte ─────────────────────────────────────────────────
#[derive(Debug, Serialize)]
pub struct ResumenTrabajador {
    pub trabajador_id: String,
    pub nombre: String,
    pub total_piezas: i64,
    pub total_pago: String,
}
