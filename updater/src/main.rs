use google_rest_api_generator as api_generator;
use serde::{Deserialize, de::DeserializeOwned};
use structopt::StructOpt;
use log::{debug, info};
use std::path::PathBuf;
use discovery_parser::DiscoveryRestDesc;
use rayon::prelude::*;
use std::fmt;

const DISCOVERY_API_LIST: &str = "https://www.googleapis.com/discovery/v1/apis";

#[derive(Debug, StructOpt)]
#[structopt(name = "api_updater", about = "Updates a set of google api bindings if necessary")]
struct Opt {
    /// The base directory to place generated crates.
    #[structopt(long="output_dir", parse(from_os_str), default_value="gen")]
    output_dir: PathBuf,

    /// The base directory that contains the discovery documents used to generate the crate in
    /// output_dir.
    #[structopt(long="discovery_doc_dir", parse(from_os_str), default_value="discovery_docs")]
    discovery_doc_dir: PathBuf,

    /// Only apis that contain this string
    #[structopt(long="contains")]
    contains: Option<String>,

    /// Only apis that contain this string
    #[structopt(long="force")]
    force: bool,
}

#[derive(Debug)]
struct StringError(String);

impl StringError {
    fn as_str(&self) -> &str {
        &self.0
    }
}

impl<T> From<T> for StringError where T: fmt::Display {
    fn from(err: T) -> StringError {
        StringError(err.to_string())
    }
}

impl From<StringError> for String {
    fn from(x: StringError) -> String {
        x.0
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiList {
    items: Vec<ApiSpec>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiSpec {
    name: String,
    version: String,
    discovery_rest_url: String,
}

const BLACKLIST: &[&str] = &[
    "google_chromewebstore_v1_1",
];

fn http_get<U, T>(client: &reqwest::Client, url: U) -> Result<T, StringError>
where
    U: reqwest::IntoUrl,
    T: DeserializeOwned,
{
    Ok(client.get(url).send()?.error_for_status()?.json()?)
}


fn main() -> Result<(), StringError> {
    let opt = Opt::from_args();
    env_logger::builder().default_format_timestamp_nanos(true).init();

    let client = reqwest::Client::new();
    info!("getting list of apis");
    let all_apis: ApiList = http_get(&client, DISCOVERY_API_LIST)?;

    let res: Result<(), StringError> = all_apis.items.par_iter().map(|api_spec| {
        handle_api_spec(&opt, &client, api_spec)
    }).collect();
    res
}

fn handle_api_spec(opt: &Opt, client: &reqwest::Client, api_spec: &ApiSpec) -> Result<(), StringError> {
    let api_name = format!("google_{}_{}", &api_spec.name, &api_spec.version).replace('.', "_");
    if BLACKLIST.contains(&api_name.as_str()) {
        info!("Skipping {} because it's in the blacklist", &api_name);
        return Ok(());
    }
    if let Some(filter) = opt.contains.as_ref() {
        if !api_name.contains(filter) {
            info!("Skipping {} because it's not in the contains list", &api_name);
            return Ok(());
        }
    }
    info!("starting {}", api_name);
    let current_discovery_doc: DiscoveryRestDesc = match http_get(client, &api_spec.discovery_rest_url) {
        Ok(x) => x,
        Err(err) => {
            eprintln!("failed to get discovery doc for {}: {}", &api_name, err.as_str());
            return Ok(());
        },
    };
    let mut discovery_doc_path = opt.discovery_doc_dir.join(&api_name);
    discovery_doc_path.set_extension("json");
    if !opt.force {
        let prev_discovery_doc: Option<DiscoveryRestDesc> = match std::fs::read_to_string(&discovery_doc_path) {
            Ok(contents) => Some(serde_json::from_str(&contents)?),
            Err(_err) => {
                info!("No previous discovery document found for {}", &api_name);
                None
            }
        };
        if Some(&current_discovery_doc) == prev_discovery_doc.as_ref() {
            debug!("Current discovery document matches previous for {}", &api_name);
            return Ok(());
        }
    }
    let api_dir = opt.output_dir.join(&api_name);
    std::fs::create_dir_all(&api_dir)?;
    api_generator::generate(&api_name, &current_discovery_doc, &api_dir)?;
    std::fs::write(&discovery_doc_path, serde_json::to_string_pretty(&current_discovery_doc)?)?;
    Ok(())
}
