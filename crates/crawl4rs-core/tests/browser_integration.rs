//! Tests de integración que lanzan un Chromium real.
//!
//! Están marcados `#[ignore]` para que la CI no dependa de un navegador
//! instalado. Para ejecutarlos localmente:
//!
//! ```bash
//! cargo test -p crawl4rs-core -- --ignored
//! ```
//!
//! El ejecutable se localiza vía `$CRAWL4RS_CHROME` o rutas conocidas.

#![cfg(feature = "browser")]

use std::sync::Arc;
use std::time::Duration;

use crawl4rs_core::{
    BrowserFetcher, BrowserPool, BrowserPoolConfig, CrawlConfig, Crawler, SessionManager,
    StealthConfig, StealthEngine,
};

/// Sirve un HTML fijo por HTTP en un puerto libre y devuelve su URL.
fn serve(body: &'static str) -> String {
    use std::io::{Read, Write};
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("puerto libre");
    let addr = listener.local_addr().unwrap();
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let mut stream = stream;
            let mut buf = [0u8; 4096];
            let _ = stream.read(&mut buf);
            let html =
                format!("<html><head><meta charset=\"utf-8\"></head><body>{body}</body></html>");
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\
                 Content-Length: {}\r\nConnection: close\r\n\r\n{html}",
                html.len(),
            );
            let _ = stream.write_all(resp.as_bytes());
        }
    });
    format!("http://{addr}/")
}

const DATA_URL: &str = "data:text/html,<html><body><article><h1>Hola CDP</h1>\
    <p>Contenido renderizado por un Chromium real controlado desde Rust.</p>\
    </article></body></html>";

#[tokio::test]
#[ignore = "lanza un Chromium real; ejecutar con --ignored"]
async fn navegador_real_extrae_html_y_markdown() {
    let fetcher = BrowserFetcher::new().with_timeout(Duration::from_secs(20));
    let crawler = Crawler::new(Arc::new(fetcher));

    let result = crawler
        .crawl(DATA_URL, &CrawlConfig::default())
        .await
        .expect("el crawl con navegador debe funcionar");

    assert!(result.html.contains("Hola CDP"));
    assert!(result.markdown.contains("# Hola CDP"));
}

#[tokio::test]
#[ignore = "lanza un Chromium real; ejecutar con --ignored"]
async fn pool_sirve_varias_paginas_concurrentes() {
    let pool = BrowserPool::launch(&BrowserPoolConfig {
        max_concurrent_pages: 2,
        ..Default::default()
    })
    .await
    .expect("el pool debe lanzarse");

    let urls = [
        "data:text/html,<h1>uno</h1>",
        "data:text/html,<h1>dos</h1>",
        "data:text/html,<h1>tres</h1>",
    ];
    let mut tareas = Vec::new();
    for url in urls {
        tareas.push(pool.fetch_page(url, Duration::from_secs(20)));
    }
    let paginas = futures::future::join_all(tareas).await;
    for (i, page) in paginas.into_iter().enumerate() {
        let page = page.expect("cada página debe descargarse");
        assert!(page.html.contains(["uno", "dos", "tres"][i]));
    }

    pool.close().await;
}

#[tokio::test]
#[ignore = "lanza un Chromium real; ejecutar con --ignored"]
async fn session_manager_persiste_cookies_y_localstorage() {
    let url = serve("<h1>sesión</h1>");
    let dir = std::env::temp_dir().join(format!("crawl4rs-test-sesion-{}", std::process::id()));
    let sesiones = SessionManager::new(&dir).expect("directorio de sesiones");

    // Perfil A: fija estado en la página y guárdalo.
    let pool_a = BrowserPool::launch(&BrowserPoolConfig::default())
        .await
        .expect("pool A");
    let page = pool_a.new_page(&url).await.expect("página A");
    let _ = page.wait_for_navigation().await;
    page.evaluate("document.cookie = 'sid=abc123; path=/'; localStorage.setItem('k', 'v42');")
        .await
        .expect("fijar estado");
    sesiones.save_cookies(&page, "perfil").await.unwrap();
    sesiones.save_local_storage(&page, "perfil").await.unwrap();
    pool_a.close().await;

    // Perfil B (navegador nuevo, perfil temporal distinto): restaura y comprueba.
    let pool_b = BrowserPool::launch(&BrowserPoolConfig::default())
        .await
        .expect("pool B");
    let page = pool_b.new_page(&url).await.expect("página B");
    let _ = page.wait_for_navigation().await;
    sesiones.restore_cookies(&page, "perfil").await.unwrap();
    sesiones
        .restore_local_storage(&page, "perfil")
        .await
        .unwrap();

    let cookie: String = page
        .evaluate("document.cookie")
        .await
        .unwrap()
        .into_value()
        .unwrap();
    let stored: String = page
        .evaluate("localStorage.getItem('k') || ''")
        .await
        .unwrap()
        .into_value()
        .unwrap();
    assert!(cookie.contains("sid=abc123"), "cookie restaurada: {cookie}");
    assert_eq!(stored, "v42");
    pool_b.close().await;

    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
#[ignore = "lanza un Chromium real; ejecutar con --ignored"]
async fn stealth_oculta_webdriver_y_expone_el_estado_en_la_pagina() {
    // La página escribe en su cuerpo el valor de navigator.webdriver y el UA,
    // de modo que aparezcan en el HTML/Markdown que devuelve el crawler.
    let url = serve(
        "<script>document.documentElement.innerHTML = \
         '<body><article><h1>wd:' + navigator.webdriver + '</h1>' + \
         '<p>ua:' + navigator.userAgent + '</p></article></body>';</script>",
    );

    let engine = Arc::new(StealthEngine::new(StealthConfig::default()));
    let fetcher = BrowserFetcher::new()
        .with_timeout(Duration::from_secs(20))
        .with_stealth(engine);
    let crawler = Crawler::new(Arc::new(fetcher));

    let config = CrawlConfig {
        word_count_threshold: 1,
        ..Default::default()
    };
    let result = crawler.crawl(&url, &config).await.expect("crawl stealth");

    // navigator.webdriver debe quedar como `undefined`, no `true`.
    assert!(
        result.html.contains("wd:undefined"),
        "html: {}",
        result.html
    );
    // El UA rotado (Chrome) debe reflejarse.
    assert!(result.html.contains("Chrome/"), "html: {}", result.html);
}
