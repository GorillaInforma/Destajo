use axum::{
    Router,
    routing::get,
    extract::{State, Query},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::{AppState, errors::AppError, models::ResumenTrabajador};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/reportes/quincena", get(reporte_quincena))
        .route("/reportes/dia", get(reporte_dia))
}

#[derive(Deserialize)]
pub struct RangoFecha {
    pub desde: String,
    pub hasta: String,
}

#[derive(Deserialize)]
pub struct FiltroDia {
    pub fecha: String,
}

#[derive(sqlx::FromRow)]
struct FilaReporte {
    trabajador_id: String,
    nombre: String,
    total_piezas: i64,
    total_pago: f64,
}

async fn reporte_quincena(
    State(state): State<Arc<AppState>>,
    Query(rango): Query<RangoFecha>,
) -> Result<Json<Vec<ResumenTrabajador>>, AppError> {
    let rows = sqlx::query_as::<_, FilaReporte>(
        "SELECT t.id AS trabajador_id, t.nombre,
         COALESCE(SUM(r.cantidad), 0) AS total_piezas,
         COALESCE(SUM(CAST(r.total AS REAL)), 0.0) AS total_pago
         FROM trabajadores t
         LEFT JOIN registros r ON r.trabajador_id = t.id AND r.fecha BETWEEN ? AND ?
         GROUP BY t.id, t.nombre ORDER BY t.nombre"
    )
    .bind(&rango.desde)
    .bind(&rango.hasta)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(rows.into_iter().map(|r| ResumenTrabajador {
        trabajador_id: r.trabajador_id,
        nombre: r.nombre,
        total_piezas: r.total_piezas,
        total_pago: format!("{:.2}", r.total_pago),
    }).collect()))
}

async fn reporte_dia(
    State(state): State<Arc<AppState>>,
    Query(filtro): Query<FiltroDia>,
) -> Result<Json<Vec<ResumenTrabajador>>, AppError> {
    let rows = sqlx::query_as::<_, FilaReporte>(
        "SELECT t.id AS trabajador_id, t.nombre,
         COALESCE(SUM(r.cantidad), 0) AS total_piezas,
         COALESCE(SUM(CAST(r.total AS REAL)), 0.0) AS total_pago
         FROM trabajadores t
         LEFT JOIN registros r ON r.trabajador_id = t.id AND r.fecha = ?
         GROUP BY t.id, t.nombre ORDER BY t.nombre"
    )
    .bind(&filtro.fecha)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(rows.into_iter().map(|r| ResumenTrabajador {
        trabajador_id: r.trabajador_id,
        nombre: r.nombre,
        total_piezas: r.total_piezas,
        total_pago: format!("{:.2}", r.total_pago),
    }).collect()))
}
