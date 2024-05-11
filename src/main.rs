use anyhow::Context;
use askama::Template;
//use std::error::Error;

use axum::{
    //body::Body,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    // routing::post,
    Router,
    // extract::Json,
};
// use std::fs::File;
// use std::io::Read;
// use std::io::Write;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

//use serde::{Serializer, Deserializer};

use serde::{Deserialize, Serialize};
//
// use serde_json::{json, Value};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "with_axum_htmx_askama=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing router...");

    let assets_path = std::env::current_dir().unwrap();
    let port = 8000_u16;
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let router = Router::new()
        .route("/", get(index))
        .route("/get_data", get(get_data))
        .nest_service(
            "/static",
            ServeDir::new(format!("{}/static", assets_path.to_str().unwrap())),
        );

    info!("router initialized, now listening on port {}", port);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .context("error while starting server")?;

    Ok(())
}

//
//
//
#[derive(Debug, Serialize, Deserialize)]
struct Location {
    #[serde(rename = "type")]
    typel: String,
    id: String,
    latitude: f32,
    longitude: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Station {
    #[serde(rename = "type")]
    typel: String,
    id: String,
    name: String,
    location: Location,
    products: Products,
}

#[derive(Debug, Serialize, Deserialize)]
struct Products {
    #[serde(rename = "nationalExpress")]
    national_express: bool,
    national: bool,
    #[serde(rename = "regionalExp")]
    regional_exp: bool,
    regional: bool,
    suburban: bool,
    bus: bool,
    ferry: bool,
    subway: bool,
    tram: bool,
    taxi: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stop {
    #[serde(rename = "type")]
    typel: String,
    id: String,
    name: String,
    location: Location,
    products: Products,
}

#[derive(Debug, Serialize, Deserialize)]
struct Operator {
    #[serde(rename = "type")]
    typel: String,
    id: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Destination {
    #[serde(rename = "type")]
    typel: String,
    id: String,
    name: String,
    location: Location,
    products: Products,
    //station: Station,
}

#[derive(Debug, Serialize, Deserialize)]
struct Line {
    #[serde(rename = "type")]
    typel: String,
    id: String,
    #[serde(rename = "fahrtNr")]
    fahrt_nr: String,
    name: String,
    public: bool,
    #[serde(rename = "adminCode")]
    admin_code: String,
    #[serde(rename = "productName")]
    product_name: String,
    mode: String,
    product: String,
    operator: Operator,
}

#[derive(Debug, Serialize, Deserialize)]
struct CurrentTripPosition {
    #[serde(rename = "type")]
    typel: String,
    latitude: f32,
    longitude: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Remarks {
    #[serde(rename = "type")]
    typel: String,
    code: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BahnData {
    #[serde(rename = "tripId")]
    trip_id: String,
    stop: Stop,
    when: Option<String>,
    #[serde(rename = "plannedWhen")]
    planned_when: String,
    delay: Option<u16>,
    platform: Option<String>,
    planned_platform: Option<String>,
    #[serde(rename = "prognosisType")]
    prognosis_type: Option<String>,
    direction: String,
    provenance: Option<String>,
    line: Line,
    remarks: Vec<Remarks>,
    origin: Option<String>,
    destination: Destination,
    //#[serde(rename = "currentTripPosition")]
    //current_trip_position: CurrentTripPosition,

    //remarks: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BahnFilter {
    //line: String,
    line: String,
    time: String,
}

async fn get_bahndata() -> Result<Vec<BahnFilter>, reqwest::Error> {
    //async fn get_bahndata () -> Result<Vec<BahnData>, reqwest::Error> {
    let bahndata: Vec<BahnData> = reqwest::Client::new()
        //.get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .get("https://v5.db.transport.rest/stops/443427/departures?duration=40")
        .send()
        .await?
        .json()
        .await?;
    println!("{:#?}", bahndata);

    let bahnfilter: Vec<BahnFilter> = bahndata
        .iter()
        .filter(|item| {
            (item.destination.id == "445363"
                || item.destination.id == "443394"
                || item.destination.id == "360940"
                || item.destination.id == "443402"
                || item.destination.id == "443370")
            && item.when.is_some()
        })
        .map(|item| BahnFilter {
            //time: item.when.clone().unwrap_or("cancelled".to_string())[11..16].to_string(),
            time: item.when.clone().unwrap()[11..16].to_string(),
            line: item.line.name.clone(),
        })
        .collect();
    println!("{:#?}", bahnfilter);

    Ok(bahnfilter)
    //Ok(bahndata)
}

async fn get_data() -> String {
    let bahndata = get_bahndata().await.expect("fetching failed");
    serde_json::to_string(&bahndata).unwrap()

    //bahndata
    // let mut greeting_file = File::open("data.json").expect("file should open read only");
    // let mut data = String::new();
    // greeting_file.read_to_string(&mut data).unwrap();
    // data
}

// test site
async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};

    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
struct HtmlTemplate<T>(T);

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
