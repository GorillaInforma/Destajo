use sqlx::SqlitePool;

pub async fn init(pool: &SqlitePool) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS trabajadores (
            id          TEXT PRIMARY KEY,
            nombre      TEXT NOT NULL,
            activo      INTEGER NOT NULL DEFAULT 1,
            creado_en   TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS operaciones (
            id          TEXT PRIMARY KEY,
            nombre      TEXT NOT NULL,
            tarifa      TEXT NOT NULL,   -- decimal como texto para evitar errores de punto flotante
            activa      INTEGER NOT NULL DEFAULT 1
        );

        CREATE TABLE IF NOT EXISTS registros (
            id              TEXT PRIMARY KEY,
            trabajador_id   TEXT NOT NULL REFERENCES trabajadores(id),
            operacion_id    TEXT NOT NULL REFERENCES operaciones(id),
            cantidad        INTEGER NOT NULL,
            total           TEXT NOT NULL,   -- decimal como texto
            fecha           TEXT NOT NULL,
            notas           TEXT
        );
        "#,
    )
    .execute(pool)
    .await?;

    tracing::info!("Base de datos inicializada");
    Ok(())
}
