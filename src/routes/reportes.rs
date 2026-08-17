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

// GET /reportes/quincena?desde=2026-08-01&hasta=2026-08-15
async fn reporte_quincena(
    State(state): State<Arc<AppState>>,
    Query(rango): Query<RangoFecha>,
) -> Result<Json<Vec<ResumenTrabajador>>, AppError> {
    // Traemos registros en el rango con nombre del trabajador
    let rows = sqlx::query!(
        r#"
        SELECT
            t.id   AS trabajador_id,
            t.nombre,
            SUM(r.cantidad) AS total_piezas,
            SUM(CAST(r.total AS REAL)) AS total_pago
        FROM registros r
        JOIN trabajadores t ON t.id = r.trabajador_id
        WHERE r.fecha BETWEEN ? AND ?
        GROUP BY t.id, t.nombre
        ORDER BY t.nombre
        "#,
        rango.desde,
        rango.hasta
    )
    .fetch_all(&state.db)
    .await?;

    let resumen: Vec<ResumenTrabajador> = rows
        .into_iter()
        .map(|row| ResumenTrabajador {
            trabajador_id: row.trabajador_id,
            nombre: row.nombre,
            total_piezas: row.total_piezas.unwrap_or(0),
            total_pago: format!("{:.2}", row.total_pago.unwrap_or(0.0)),
        })
        .collect();

    Ok(Json(resumen))
}

// GET /reportes/dia?fecha=2026-08-16
async fn reporte_dia(
    State(state): State<Arc<AppState>>,
    Query(filtro): Query<FiltroDia>,
) -> Result<Json<Vec<ResumenTrabajador>>, AppError> {
    let rows = sqlx::query!(
        r#"
        SELECT
            t.id   AS trabajador_id,
            t.nombre,
            SUM(r.cantidad) AS total_piezas,
            SUM(CAST(r.total AS REAL)) AS total_pago
        FROM registros r
        JOIN trabajadores t ON t.id = r.trabajador_id
        WHERE r.fecha = ?
        GROUP BY t.id, t.nombre
        ORDER BY t.nombre
        "#,
        filtro.fecha
    )
    .fetch_all(&state.db)
    .await?;

    let resumen: Vec<ResumenTrabajador> = rows
        .into_iter()
        .map(|row| ResumenTrabajador {
            trabajador_id: row.trabajador_id,
            nombre: row.nombre,
            total_piezas: row.total_piezas.unwrap_or(0),
            total_pago: format!("{:.2}", row.total_pago.unwrap_or(0.0)),
        })
        .collect();

    Ok(Json(resumen))
}
