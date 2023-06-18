use std::collections::HashMap;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::sdk::{self, Resource};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use tracing_actix_web::TracingLogger;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = init_tracer()?;

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/sleep", actix_web::web::get().to(sleep))
            .wrap(TracingLogger::default())
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await?;

    Ok(())
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello everyone!")
}

async fn sleep() -> actix_web::Result<HttpResponse> {
    tracing::info_span!("Sleeping...");
    wait().await;
    tracing::info_span!("awake!");
    Ok(HttpResponse::Ok().body("Done"))
}

async fn wait() {
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
}

fn init_tracer() -> std::io::Result<()> {
    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

    let exporter = opentelemetry_otlp::new_exporter()
        .http()
        .with_endpoint("http://localhost:4317/v1/traces")
        .with_headers(HashMap::from([(
            "api-key".into(),
            "YOUR_API_KEY_IF_NEED".to_string(),
        )]));

    let tracer =
        opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(exporter)
            .with_trace_config(sdk::trace::config().with_resource(Resource::new(vec![
                KeyValue::new("service.name", "SERVICE_NAME_IF_NEED"),
            ])))
            .install_batch(opentelemetry::runtime::Tokio)
            .expect("Error - Failed to create tracer.");

    Registry::default()
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    Ok(())
}
