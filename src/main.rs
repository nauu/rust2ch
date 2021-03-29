use clickhouse_rs::Block;
use rust2ch::ClickHouseEngine;
use structopt::StructOpt;
use std::error::Error;

/// A rust2ch example
#[derive(StructOpt,Debug)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt,Debug)]
enum Command {
    /// Add new product
    Add {product_name: String},
    /// List products
    List {id: String},
}

#[tokio::main]
 async fn main() -> Result<(), Box<dyn Error>> {

    let opt = Args::from_args();
    println!("{:#?}", opt);

    let ddl = r"
        CREATE TABLE IF NOT EXISTS t_product (
            product_id  UInt32,
            price       UInt32,
            product_name Nullable(FixedString(5))
        ) Engine=Memory";

    let block = Block::new()
        .column("product_id", vec![1_u32, 3, 5, 7, 9])
        .column("price", vec![2_u32, 4, 6, 8, 10])
        .column(
            "product_name",
            vec![Some("foo"), Some("foo"), None, None, Some("bar")],
        );

    let database_url = "tcp://10.37.129.9:9000/default?compression=lz4&ping_timeout=42ms";

    let ce = ClickHouseEngine::new(database_url);

    match opt.cmd {
        Some(Command::Add { product_name, .. }) => {
            println!("Add new product with product_name '{}'", &product_name);
            ce.ddl_str(ddl).await?;
            ce.insert_block("t_product", block).await?;
        }
        Some(Command::List { id }) => {
            let block = ce.query_str("SELECT * FROM t_product").await?;
            println!("count:{} ", block.rows().count());
            for row in block.rows() {
                let id: u32 = row.get("product_id")?;
                let amount: u32 = row.get("price")?;
                let name: Option<&str> = row.get("product_name")?;
                println!("Found  {}: {} {:?}", id, amount, name);
            }
        }
        _ => {}
    }

    Ok(())
}
