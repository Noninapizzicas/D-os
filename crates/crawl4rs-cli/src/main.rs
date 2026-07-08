//! CLI de Crawl4RS.
//!
//! ```text
//! crawl4rs crawl <url> [OPTIONS]   Procesa una URL (o un HTML local con --html-file)
//! crawl4rs deep  <url> [OPTIONS]   Crawl profundo siguiendo enlaces (BFS/DFS)
//! crawl4rs serve [OPTIONS]         Lanza el servidor API — Fase 6
//! crawl4rs config                  Muestra la configuración por defecto
//! ```

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use tracing_subscriber::EnvFilter;

use crawl4rs_core::{BrowserFetcher, CrawlConfig, Crawler, DeepStrategy, StaticFetcher};

#[derive(Parser)]
#[command(
    name = "crawl4rs",
    version,
    about = "Convierte cualquier página web en Markdown para LLMs."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Procesa una única URL.
    Crawl(CrawlArgs),
    /// Crawl profundo siguiendo enlaces (BFS/DFS).
    Deep(DeepArgs),
    /// Lanza el servidor API.
    Serve {
        /// Puerto de escucha.
        #[arg(long, default_value_t = 8080)]
        port: u16,
    },
    /// Muestra la configuración por defecto en JSON.
    Config,
}

#[derive(clap::Args)]
struct CrawlArgs {
    /// URL a procesar.
    url: String,
    /// Consulta para el filtrado por relevancia (BM25).
    #[arg(long)]
    query: Option<String>,
    /// Procesa un fichero HTML local en lugar de descargar la URL.
    #[arg(long)]
    html_file: Option<String>,
    /// Imprime `fit_markdown` en lugar del markdown completo.
    #[arg(long)]
    fit: bool,
    /// Imprime el resultado completo como JSON.
    #[arg(long)]
    json: bool,
}

#[derive(clap::Args)]
struct DeepArgs {
    /// URL semilla desde la que arranca el recorrido.
    url: String,
    /// Consulta para el filtrado por relevancia (BM25).
    #[arg(long)]
    query: Option<String>,
    /// Estrategia de recorrido.
    #[arg(long, value_enum, default_value_t = Strategy::Bfs)]
    strategy: Strategy,
    /// Máximo de páginas a visitar.
    #[arg(long, default_value_t = 25)]
    max_pages: usize,
    /// Profundidad máxima (0 = sólo la semilla).
    #[arg(long, default_value_t = 2)]
    max_depth: usize,
    /// Seguir también enlaces a otros dominios.
    #[arg(long)]
    cross_domain: bool,
    /// Imprime el informe completo como JSON.
    #[arg(long)]
    json: bool,
}

#[derive(Clone, Copy, ValueEnum)]
enum Strategy {
    Bfs,
    Dfs,
}

impl From<Strategy> for DeepStrategy {
    fn from(s: Strategy) -> Self {
        match s {
            Strategy::Bfs => DeepStrategy::Bfs,
            Strategy::Dfs => DeepStrategy::Dfs,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    let cli = Cli::parse();
    match cli.command {
        Command::Crawl(args) => run_crawl(args).await,
        Command::Deep(args) => run_deep(args).await,
        Command::Serve { port } => {
            eprintln!("`serve` (API axum en :{port}) llega en la Fase 6 de la hoja de ruta.");
            std::process::exit(2);
        }
        Command::Config => {
            let json = serde_json::to_string_pretty(&CrawlConfig::default())?;
            println!("{json}");
            Ok(())
        }
    }
}

/// Ejecuta `f` con un crawler de navegador, cerrando Chromium de forma
/// ordenada al terminar (aunque `f` devuelva error).
async fn with_browser<F, Fut, T>(timeout_ms: u64, f: F) -> Result<T>
where
    F: FnOnce(Crawler) -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let fetcher = Arc::new(BrowserFetcher::new().with_timeout(Duration::from_millis(timeout_ms)));
    let crawler = Crawler::new(fetcher.clone());
    let out = f(crawler).await;
    if let Ok(fetcher) = Arc::try_unwrap(fetcher) {
        fetcher.shutdown().await;
    }
    out
}

async fn run_crawl(args: CrawlArgs) -> Result<()> {
    let config = CrawlConfig::default().with_query(args.query.clone());

    let result = match &args.html_file {
        Some(path) => {
            let html = std::fs::read_to_string(path)?;
            let crawler = Crawler::new(Arc::new(StaticFetcher::new(html)));
            crawler.crawl(&args.url, &config).await?
        }
        None => {
            let url = args.url.clone();
            with_browser(config.timeout_ms, |crawler| async move {
                Ok(crawler.crawl(&url, &config).await?)
            })
            .await?
        }
    };

    if args.json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else if args.fit {
        println!("{}", result.fit_markdown);
    } else {
        println!("{}", result.markdown);
    }
    Ok(())
}

async fn run_deep(args: DeepArgs) -> Result<()> {
    let config = CrawlConfig {
        query: args.query.clone(),
        max_pages: args.max_pages,
        max_depth: args.max_depth,
        deep_strategy: args.strategy.into(),
        same_domain: !args.cross_domain,
        ..Default::default()
    };

    let url = args.url.clone();
    let report = with_browser(config.timeout_ms, |crawler| async move {
        Ok(crawler.crawl_deep(&url, &config).await?)
    })
    .await?;

    if args.json {
        let json = serde_json::json!({
            "pages": report.pages,
            "errors": report.errors
                .iter()
                .map(|(u, e)| serde_json::json!({ "url": u, "error": e }))
                .collect::<Vec<_>>(),
        });
        println!("{}", serde_json::to_string_pretty(&json)?);
    } else {
        eprintln!(
            "Recorridas {} páginas ({} errores).\n",
            report.pages.len(),
            report.errors.len()
        );
        for page in &report.pages {
            println!("# {}\n", page.url);
            println!("{}\n", page.fit_markdown);
            println!("---\n");
        }
    }
    Ok(())
}
