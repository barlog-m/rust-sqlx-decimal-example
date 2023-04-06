use std::env;
use sqlx::{PgPool, FromRow, Row, postgres::{PgRow, types::PgMoney}};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

type AppError = Box<dyn std::error::Error + Send + Sync + 'static>;
type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
struct Item {
    id: i64,
    name: String,
    price: Decimal,
}

impl FromRow<'_, PgRow> for Item {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let id: i64 = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let money: PgMoney = row.try_get("price")?;
        let locale_frac_digits = 2;
        let price = money.to_decimal(locale_frac_digits);
        Ok(Item { id, name, price })
    }
}

#[tokio::main]
async fn main() -> AppResult<()> {
    tracer_init();

    let db_url = "postgres://postgres:password@localhost:5432/postgres";
    let pool = PgPool::connect(db_url).await?;

    let new_item = Item {
        id: -1,
        name: "foo".to_string(),
        price: dec!(2.99),
    };

    let stored_item = add(&pool, new_item).await?;
    let i = get(&pool, stored_item.id).await?;

    tracing::info!("{:?}", i);

    Ok(())
}

async fn add(pool: &PgPool, i: Item) -> AppResult<Item> {
    let row = sqlx::query(
        r#"
            insert into inventory(name, price)
            values ($1, $2)
            returning id, name, price
        "#
        )
        .bind(i.name)
        .bind(PgMoney::from_decimal(i.price, 2))
        .fetch_one(pool)
        .await?;

    let si = Item::from_row(&row)?;

    Ok(si)
}

async fn get(pool: &PgPool, id: i64) -> AppResult<Item> {
    let row = sqlx::query(
        r#"
            select id, name, price
            from inventory
            where id = $1
        "#)
    .bind(id)
    .fetch_one(pool)
    .await?;

    let i = Item::from_row(&row)?;

    Ok(i)
}

fn tracer_init() {
    env::set_var(
        "RUST_LOG",
        "sqlx=debug,tokio=trace,debug",
    );
    tracing_subscriber::fmt::init();
}

