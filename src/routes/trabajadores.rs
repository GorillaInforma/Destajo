use axum::{
    Router,
    routing::{get, post, put},
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use crate::{AppState, errors::AppError, models::{Trabajador, NuevoTrabajador}};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/trabajadores", get(listar).post(crear))
        .route("/trabajadores/:id", get(obtener).put(desactivar))
}

// GET /trabajadores
async fn listar(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Trabajador>>, AppError> {
    let trabajadores = sqlx::query_as::<_, Trabajador>(
        "SELECT id, nombre, activo, creado_en FROM trabajadores ORDER BY nombre"
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(trabajadores))
}

// POST /trabajadores
async fn crear(
    State(state): State<Arc<AppState>>,
    Json(body): Json<NuevoTrabajador>,
) -> Result<Json<Trabajador>, AppError> {
    if body.nombre.trim().is_empty() {
        return Err(AppError::BadRequest("El nombre no puede estar vacío".into()));
    }

    let id = Uuid::new_v4().to_string();
    let ahora = Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO trabajadores (id, nombre, activo, creado_en) VALUES (?, ?, 1, ?)"
    )
    .bind(&id)
    .bind(body.nombre.trim())
    .bind(&ahora)
    .execute(&state.db)
    .await?;

    let trabajador = sqlx::query_as::<_, Trabajador>(
        "SELECT id, nombre, activo, creado_en FROM trabajadores WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(trabajador))
}

// GET /trabajadores/:id
async fn obtener(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Trabajador>, AppError> {
    let trabajador = sqlx::query_as::<_, Trabajador>(
        "SELECT id, nombre, activo, creado_en FROM trabajadores WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Trabajador no encontrado".into()))?;

    Ok(Json(trabajador))
}

// PUT /trabajadores/:id  (toggle activo/inactivo)
async fn desactivar(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Trabajador>, AppError> {
    sqlx::query(
        "UPDATE trabajadores SET activo = NOT activo WHERE id = ?"
    )
    .bind(&id)
    .execute(&state.db)
    .await?;

    let trabajador = sqlx::query_as::<_, Trabajador>(
        "SELECT id, nombre, activo, creado_en FROM trabajadores WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Trabajador no encontrado".into()))?;

    Ok(Json(trabajador))
}
