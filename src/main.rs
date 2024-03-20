use std::{env, fmt::format, iter};

use clap::Parser;
use tokio_postgres::{types::ToSql, Client, Error, NoTls, Row};

async fn query<T>(
    client: &Client,
    query_str: &str,
    params: &Vec<Box<(dyn ToSql + Sync)>>,
    from_row: impl Fn(Row) -> T,
) -> Result<Vec<T>, Error> {
    let param_slice = &params.iter().map(|x| x.as_ref()).collect::<Vec<_>>();
    let rows = client.query(query_str, &param_slice).await?;
    Ok(rows.into_iter().map(from_row).collect())
}

#[derive(Debug)]
struct Item {
    pub id: i32,
    pub name: String,
    pub money_in_bank: i32, // just for demo, never use int for actual money
}

fn from_row(row: Row) -> Item {
    Item {
        id: row.get("id"),
        name: row.get("name"),
        money_in_bank: row.get("money_in_bank"),
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    pub id_eq: Option<i32>,

    #[clap(long)]
    pub name_eq: Option<String>,

    #[clap(long)]
    pub name_like: Option<String>,

    #[clap(long)]
    pub money_in_bank_lt: Option<i32>,

    #[clap(long)]
    pub money_in_bank_gt: Option<i32>,
}

fn build_query(query: &String, cond: String) -> String {
    if query.is_empty() {
        cond
    } else {
        format!("{query} and {cond} ")
    }
}

fn query_param_pair_from_args(args: Args) -> (String, Vec<Box<(dyn ToSql + Sync)>>) {
    let mut query: String = "".to_string();
    let mut idx = 1;
    let mut params: Vec<Box<(dyn ToSql + Sync)>> = vec![];
    if let Some(x) = args.id_eq {
        let cond = format!("id = ${idx}");
        if query.is_empty() {
            query = cond;
        } else {
            query = format!("{query} and {cond} ");
        }
        idx = idx + 1;
        params.push(Box::new(x));
    }
    if let Some(x) = args.name_eq {
        let cond = format!("name = ${idx}");
        if query.is_empty() {
            query = cond;
        } else {
            query = format!("{query} and {cond} ");
        }
        idx = idx + 1;
        params.push(Box::new(x));
    }
    if let Some(x) = args.name_like {
        let cond = format!("name like ${idx}");
        if query.is_empty() {
            query = cond;
        } else {
            query = format!("{query} and {cond} ");
        }
        idx = idx + 1;
        params.push(Box::new(x));
    }
    if let Some(x) = args.money_in_bank_lt {
        let cond = format!("money_in_bank < ${idx}");
        if query.is_empty() {
            query = cond;
        } else {
            query = format!("{query} and {cond} ");
        }
        idx = idx + 1;
        params.push(Box::new(x));
    }
    if let Some(x) = args.money_in_bank_gt {
        let cond = format!("money_in_bank > ${idx}");
        if query.is_empty() {
            query = cond;
        } else {
            query = format!("{query} and {cond} ");
        }
        params.push(Box::new(x));
    }
    (query, params)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let (client, conn) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres", NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("connection error: {}", e);
        }
    });
    let (where_str, params) = query_param_pair_from_args(args);
    let base_query = "select * from items";
    let full_query = if where_str.is_empty() {
        base_query.to_string()
    } else {
        format!("{base_query} where {where_str}")
    };
    println!("{full_query}");
    let items = query(&client, &full_query.as_str(), &params, from_row).await?;
    for i in items {
        println!("{:?}", i)
    }
    Ok(())
}
