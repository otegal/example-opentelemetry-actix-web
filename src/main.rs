use opentelemetry::global;
use opentelemetry::sdk::export::trace::stdout;
use opentelemetry::trace;
use opentelemetry::trace::Tracer;
use opentelemetry::Context;

fn main() {
    let tracer = stdout::new_pipeline().install_simple();

    let cxt = Context::current();
    let root_span = tracer.start_with_context("root", &cxt);
    let active = trace::mark_span_as_active(root_span);
    println!("Hello, world!");
    println!("This is example");
    drop(active);

    global::shutdown_tracer_provider();
}
