use super::models::{CreateJigDto, Jig};
use sqlx::PgPool;

#[derive(Clone)]
pub struct JigRepository {
    pool: PgPool,
}

impl JigRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Jig>, sqlx::Error> {
        sqlx::query_as::<_, Jig>("SELECT * FROM jigs WHERE deleted IS NULL ORDER BY no DESC")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn create(&self, dto: &CreateJigDto) -> Result<Jig, sqlx::Error> {
        sqlx::query_as::<_, Jig>(
            "INSERT INTO jigs (prov, cantidad, tablero, color, mf, types, pin, note) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) 
             RETURNING *",
        )
        .bind(&dto.prov)
        .bind(dto.cantidad)
        .bind(&dto.tablero)
        .bind(&dto.color)
        .bind(&dto.mf)
        .bind(&dto.types)
        .bind(&dto.pin)
        .bind(&dto.note)
        .fetch_one(&self.pool)
        .await
    }
}
