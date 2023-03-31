use std::env;
use sqlx::PgPool;
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
    let si = sqlx::query_as!(
        Item,
        r#"
            insert into inventory(name, price)
            values ($1, $2)
            returning id, name, price
        "#,
        i.name,
        i.price
    )
    .fetch_one(pool)
    .await?;

    Ok(si)
}

async fn get(pool: &PgPool, id: i64) -> AppResult<Item> {
    let i = sqlx::query_as!(
        Item,
        r#"
            select id, name, price
            from inventory
            where id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(i)
}

fn tracer_init() {
    env::set_var(
        "RUST_LOG",
        "sqlx=debug,tokio=trace,debug",
    );
    tracing_subscriber::fmt::init();
}


