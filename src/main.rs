use clap::Parser;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use log::error;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IpInfo {
    pub query: String,
    pub country: String,
    pub region_name: String,
    pub city: String,
    pub isp: String,
    pub org: String,
    #[serde(rename = "as")]
    pub as_field: String,
    pub mobile: bool,
    pub proxy: bool,
    pub hosting: bool,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    ip: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    let query = format!("http://ip-api.com/json/{}?fields=country,regionName,city,isp,org,as,mobile,proxy,hosting,query", args.ip);
    let body = match reqwest::get(query).await {
        Ok(res) => {
            res
        }
        Err(e) => {
            error!("{}", e);
            return
        }
    };

    let result: IpInfo = match serde_json::from_str(body.text().await.unwrap().as_str()) {
        Ok(res) => {
            res
        }
        Err(e) => {
            error!("{}", e);
            return
        }
    };

    let result_table = vec![
        vec!["Request ip".cell(), result.query.cell().justify(Justify::Right)],
        vec!["Country".cell(), result.country.cell().justify(Justify::Right)],
        vec!["Region".cell(), result.region_name.cell().justify(Justify::Right)],
        vec!["City".cell(), result.city.cell().justify(Justify::Right)],
        vec!["Provider".cell(), result.isp.cell().justify(Justify::Right)],
        vec!["Organization".cell(), result.org.cell().justify(Justify::Right)],
        vec!["AS".cell(), result.as_field.cell().justify(Justify::Right)],
        vec!["Is mobile".cell(), result.mobile.cell().justify(Justify::Right)],
        vec!["Is proxy".cell(), result.proxy.cell().justify(Justify::Right)],
        vec!["Is hosting".cell(), result.hosting.cell().justify(Justify::Right)],
        ]
    .table()
    .title(vec![
        "Name".cell().bold(true),
        "Response".cell().bold(true).justify(Justify::Right)
    ])
    .bold(true);

    match print_stdout(result_table){
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
        }
    };
}
