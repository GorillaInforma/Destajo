use axum::{
    Router,
    routing::{get, post, delete},
    extract::{Path, State, Query},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{AppState, errors::AppError, models::{Registro, NuevoRegistro}};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/registros", get(listar).post(crear))
        .route("/registros/:id", delete(eliminar))
}

#[derive(Deserialize)]
pub struct FiltroFecha {
    pub fecha: Option<String>,
    pub trabajador_id: Option<String>,
}

// GET /registros?fecha=2026-08-16&trabajador_id=xxx
async fn listar(
    State(state): State<Arc<AppState>>,
    Query(filtro): Query<FiltroFecha>,
) -> Result<Json<Vec<Registro>>, AppError> {
    // Construimos la query con filtros opcionales
    let registros = match (filtro.fecha, filtro.trabajador_id) {
        (Some(fecha), Some(tid)) => {
            sqlx::query_as::<_, Registro>(
                "SELECT id, trabajador_id, operacion_id, cantidad, total, fecha, notas
                 FROM registros WHERE fecha = ? AND trabajador_id = ? ORDER BY fecha DESC"
            )
            .bind(fecha)
            .bind(tid)
            .fetch_all(&state.db)
            .await?
        }
        (Some(fecha), None) => {
            sqlx::query_as::<_, Registro>(
                "SELECT id, trabajador_id, operacion_id, cantidad, total, fecha, notas
                 FROM registros WHERE fecha = ? ORDER BY fecha DESC"
            )
            .bind(fecha)
            .fetch_all(&state.db)
            .await?
        }
        (None, Some(tid)) => {
            sqlx::query_as::<_, Registro>(
                "SELECT id, trabajador_id, operacion_id, cantidad, total, fecha, notas
                 FROM registros WHERE trabajador_id = ? ORDER BY fecha DESC"
            )
            .bind(tid)
            .fetch_all(&state.db)
            .await?
        }
        (None, None) => {
            sqlx::query_as::<_, Registro>(
                "SELECT id, trabajador_id, operacion_id, cantidad, total, fecha, notas
                 FROM registros ORDER BY fecha DESC LIMIT 100"
            )
            .fetch_all(&state.db)
            .await?
        }
    };

    Ok(Json(registros))
}

// POST /registros
async fn crear(
    State(state): State<Arc<AppState>>,
    Json(body): Json<NuevoRegistro>,
) -> Result<Json<Registro>, AppError> {
    if body.cantidad <= 0 {
        return Err(AppError::BadRequest("La cantidad debe ser mayor a 0".into()));
    }

    // Buscar tarifa de la operación
    let tarifa: String = sqlx::query_scalar(
        "SELECT tarifa FROM operaciones WHERE id = ? AND activa = 1"
    )
    .bind(&body.operacion_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Operación no encontrada o inactiva".into()))?;

    // Calcular total con precisión decimal
    let tarifa_f: f64 = tarifa.parse().unwrap_or(0.0);
    let total = tarifa_f * body.cantidad as f64;
    let total_str = format!("{:.2}", total);

    let id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO registros (id, trabajador_id, operacion_id, cantidad, total, fecha, notas)
         VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&body.trabajador_id)
    .bind(&body.operacion_id)
    .bind(body.cantidad)
    .bind(&total_str)
    .bind(&body.fecha)
    .bind(&body.notas)
    .execute(&state.db)
    .await?;

    let registro = sqlx::query_as::<_, Registro>(
        "SELECT id, trabajador_id, operacion_id, cantidad, total, fecha, notas
         FROM registros WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(registro))
}

// DELETE /registros/:id
async fn eliminar(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let resultado = sqlx::query("DELETE FROM registros WHERE id = ?")
        .bind(&id)
        .execute(&state.db)
        .await?;

    if resultado.rows_affected() == 0 {
        return Err(AppError::NotFound("Registro no encontrado".into()));
    }

    Ok(Json(serde_json::json!({ "mensaje": "Registro eliminado" })))
}
