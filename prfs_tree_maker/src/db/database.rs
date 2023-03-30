use super::models::Account;
use crate::TreeMakerError;
use rust_decimal::Decimal;
use std::collections::BTreeMap;
use tokio_postgres::{Client as PGClient, NoTls};

pub struct Database {
    pub pg_client: PGClient,
}

impl Database {
    pub async fn connect() -> Result<Database, TreeMakerError> {
        let postgres_pw = std::env::var("POSTGRES_PW")?;

        let pg_config = format!(
            "host=database-1.cstgyxdzqynn.ap-northeast-2.rds.amazonaws.com user=postgres password={}",
            postgres_pw,
        );

        let (pg_client, connection) = tokio_postgres::connect(&pg_config, NoTls).await?;

        let d = Database { pg_client };

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                println!("connection error: {}", e);
            }
        });

        return Ok(d);
    }
}

impl Database {
    pub async fn get_accounts(&self, where_clause: &str) -> Result<Vec<Account>, TreeMakerError> {
        let stmt = format!("SELECT * from balances_20230327 where {}", where_clause);
        println!("stmt: {}", stmt);

        let rows = self.pg_client.query(&stmt, &[]).await?;
        // let mut v = Vec::with_capacity(rows.len());

        let v: Vec<Account> = rows
            .iter()
            .map(|r| {
                let addr: String = r.try_get("addr").expect("addr should be present");
                let wei: Decimal = r.try_get("wei").expect("wei should be present");

                let acc = Account { addr, wei };
                acc
            })
            .collect();

        Ok(v)
    }

    pub async fn insert_accounts(
        &self,
        balances: BTreeMap<String, Account>,
        update_on_conflict: bool,
    ) -> Result<u64, TreeMakerError> {
        let mut values = Vec::with_capacity(balances.len());
        for (_, acc) in balances {
            let val = format!("('{}', {})", acc.addr, acc.wei);
            values.push(val);
        }

        let stmt = if update_on_conflict {
            format!(
                "INSERT INTO balances_20230327 (addr, wei) VALUES {} ON CONFLICT(addr) {}",
                values.join(","),
                "DO UPDATE SET wei = excluded.wei, updated_at = now()",
            )
        } else {
            format!(
                "INSERT INTO balances_20230327 (addr, wei) VALUES {} ON CONFLICT DO NOTHING",
                values.join(",")
            )
        };
        // println!("stmt: {}", stmt);

        let rows_updated = match self.pg_client.execute(&stmt, &[]).await {
            Ok(r) => r,
            Err(err) => {
                tracing::error!("Error executing stmt, err: {}, stmt: {}", err, stmt);

                return Err(err.into());
            }
        };

        Ok(rows_updated)
    }

    pub async fn insert_nodes(
        &self,
        accounts: Vec<Account>,
        update_on_conflict: bool,
    ) -> Result<u64, TreeMakerError> {
        let mut values = Vec::with_capacity(accounts.len());
        for acc in accounts {
            let val = format!("('{}', {})", acc.addr, acc.wei);
            values.push(val);
        }

        let stmt = if update_on_conflict {
            format!(
                "INSERT INTO balances_20230327 (addr, wei) VALUES {} ON CONFLICT(addr) {}",
                values.join(","),
                "DO UPDATE SET wei = excluded.wei, updated_at = now()",
            )
        } else {
            format!(
                "INSERT INTO balances_20230327 (addr, wei) VALUES {} ON CONFLICT DO NOTHING",
                values.join(",")
            )
        };
        // println!("stmt: {}", stmt);

        let rows_updated = match self.pg_client.execute(&stmt, &[]).await {
            Ok(r) => r,
            Err(err) => {
                tracing::error!("Error executing stmt, err: {}, stmt: {}", err, stmt);

                return Err(err.into());
            }
        };

        Ok(rows_updated)
    }
}
