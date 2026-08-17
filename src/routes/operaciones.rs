use axum::{
    Router,
    routing::{get, post, put},
    extract::{Path, State},
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{AppState, errors::AppError, models::{Operacion, NuevaOperacion}};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/operaciones", get(listar).post(crear))
        .route("/operaciones/:id", get(obtener).put(actualizar_tarifa))
}

// GET /operaciones
async fn listar(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Operacion>>, AppError> {
    let ops = sqlx::query_as::<_, Operacion>(
        "SELECT id, nombre, tarifa, activa FROM operaciones ORDER BY nombre"
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(ops))
}

// POST /operaciones
async fn crear(
    State(state): State<Arc<AppState>>,
    Json(body): Json<NuevaOperacion>,
) -> Result<Json<Operacion>, AppError> {
    if body.nombre.trim().is_empty() {
        return Err(AppError::BadRequest("El nombre no puede estar vacío".into()));
    }

    // Validar que tarifa sea número válido
    body.tarifa.parse::<f64>()
        .map_err(|_| AppError::BadRequest("Tarifa inválida, debe ser un número".into()))?;

    let id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO operaciones (id, nombre, tarifa, activa) VALUES (?, ?, ?, 1)"
    )
    .bind(&id)
    .bind(body.nombre.trim())
    .bind(&body.tarifa)
    .execute(&state.db)
    .await?;

    let op = sqlx::query_as::<_, Operacion>(
        "SELECT id, nombre, tarifa, activa FROM operaciones WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(op))
}

// GET /operaciones/:id
async fn obtener(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Operacion>, AppError> {
    let op = sqlx::query_as::<_, Operacion>(
        "SELECT id, nombre, tarifa, activa FROM operaciones WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Operación no encontrada".into()))?;

    Ok(Json(op))
}

// PUT /operaciones/:id  (actualizar tarifa)
async fn actualizar_tarifa(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(body): Json<NuevaOperacion>,
) -> Result<Json<Operacion>, AppError> {
    body.tarifa.parse::<f64>()
        .map_err(|_| AppError::BadRequest("Tarifa inválida".into()))?;

    sqlx::query("UPDATE operaciones SET tarifa = ? WHERE id = ?")
        .bind(&body.tarifa)
        .bind(&id)
        .execute(&state.db)
        .await?;

    let op = sqlx::query_as::<_, Operacion>(
        "SELECT id, nombre, tarifa, activa FROM operaciones WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Operación no encontrada".into()))?;

    Ok(Json(op))
}
