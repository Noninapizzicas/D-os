//! CLI de Crawl4RS.
//!
//! ```text
//! crawl4rs crawl <url> [OPTIONS]   Procesa una URL (o un HTML local con --html-file)
//! crawl4rs deep  <url> [OPTIONS]   Crawl profundo (BFS/DFS) — Fase 3
//! crawl4rs serve [OPTIONS]         Lanza el servidor API — Fase 6
//! crawl4rs config                  Muestra la configuración por defecto
//! ```

use std::sync::Arc;

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

use crawl4rs_core::{BrowserFetcher, CrawlConfig, Crawler, StaticFetcher};

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
    Deep(CrawlArgs),
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
        Command::Deep(_) => {
            eprintln!("`deep` (crawl profundo BFS/DFS) llega en la Fase 3 de la hoja de ruta.");
            std::process::exit(2);
        }
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

async fn run_crawl(args: CrawlArgs) -> Result<()> {
    let config = CrawlConfig::default().with_query(args.query.clone());

    let result = match &args.html_file {
        Some(path) => {
            let html = std::fs::read_to_string(path)?;
            let crawler = Crawler::new(Arc::new(StaticFetcher::new(html)));
            crawler.crawl(&args.url, &config).await?
        }
        None => {
            let fetcher = Arc::new(
                BrowserFetcher::new()
                    .with_timeout(std::time::Duration::from_millis(config.timeout_ms)),
            );
            let crawler = Crawler::new(fetcher.clone());
            let result = crawler.crawl(&args.url, &config).await;
            // Cierre ordenado de Chromium antes de propagar el resultado.
            drop(crawler);
            if let Ok(fetcher) = Arc::try_unwrap(fetcher) {
                fetcher.shutdown().await;
            }
            result?
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
