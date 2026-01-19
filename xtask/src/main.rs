use futures_util::{SinkExt, StreamExt};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::{
    fs,
    net::IpAddr,
    path::{Path, PathBuf},
};
use tokio::sync::broadcast::{self, Receiver, Sender};
use warp::{Filter, Rejection, http::Response};

const CUSTOM_DOCS_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/../docs/rustdocs/");
const CARGO_DOCS_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/doc/");
const ROOT_PATH: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/../");

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut docs: bool = false;
    let mut docs_gen: bool = false;
    let mut docs_open: bool = false;
    let mut docs_run: bool = false;

    // TODO: Better arg parsing, or just use `clap`
    for i in 0..args.len() {
        match args[i].as_str() {
            "docs" => docs = true,
            "gen" => docs_gen = true,
            "open" => docs_open = true,
            "run" => docs_run = true,
            "full" => {
                docs_gen = true;
                docs_open = true;
                docs_run = true;
            }
            _ => {}
        };
    }

    if docs {
        if docs_gen {
            println!("Generating docs...");
            if fs::exists(CUSTOM_DOCS_PATH).unwrap() {
                move_dir_recursive(Path::new(CUSTOM_DOCS_PATH), Path::new(CARGO_DOCS_PATH))
                    .unwrap();
            }

            std::process::Command::new("cargo")
                .current_dir(ROOT_PATH)
                .arg("sys-gen-docs")
                .output()
                .expect("Cargo failed to generate docs");

            if fs::exists(CARGO_DOCS_PATH).unwrap() {
                move_dir_recursive(Path::new(CARGO_DOCS_PATH), Path::new(CUSTOM_DOCS_PATH))
                    .unwrap();
            }
            println!("Finished generating docs!");
        }

        if docs_open {
            println!("Opening http://localhost:8080/...");
            webbrowser::open("http://localhost:8080/").unwrap();
            println!("Opened docs!");
        }

        if docs_run {
            println!("Starting docs server...");
            run_docs_server().await;
        }
    }
}

fn move_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dst)?;

        for entry in src.read_dir()? {
            let entry: fs::DirEntry = entry?;
            let src_path: PathBuf = entry.path();
            let dst_path: PathBuf = dst.join(entry.file_name());
            if src_path.is_dir() {
                move_dir_recursive(&src_path, &dst_path)?;
            } else {
                fs::rename(&src_path, &dst_path)?;
            }
        }
    } else {
        fs::rename(src, dst)?;
    }

    fs::remove_dir_all(src)?;
    Ok(())
}

async fn run_docs_server() {
    let serve_dir: String = CUSTOM_DOCS_PATH.to_string();
    let port: String = "8080".to_string();
    let host: String = "127.0.0.1".to_string();

    let (tx, _rx): (Sender<String>, Receiver<String>) = broadcast::channel(100);
    {
        let tx: Sender<String> = tx.clone();
        let folder: String = serve_dir.to_string();

        std::thread::spawn(move || {
            let mut watcher: RecommendedWatcher =
                notify::recommended_watcher(move |res: Result<Event>| {
                    if let Ok(event) = res {
                        if matches!(
                            event.kind,
                            EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
                        ) {
                            if event.paths.iter().any(|path: &PathBuf| -> bool {
                                match path.extension().and_then(|e: &std::ffi::OsStr| e.to_str()) {
                                    Some(ext) => {
                                        matches!(ext, "html" | "css" | "js" | "jsx" | "ts" | "tsx")
                                    }
                                    None => false,
                                }
                            }) {
                                drop(tx.send("reload".to_string()));
                                println!("Reloading...");
                            }
                        }
                    }
                })
                .expect("Failed to create file watcher");

            watcher
                .watch(Path::new(&folder), RecursiveMode::Recursive)
                .expect("Failed to watch folder");

            loop {
                std::thread::park();
            }
        });
    };

    let static_files = warp::fs::dir(serve_dir.clone());
    let ws_route = warp::path("livereload").and(warp::ws()).map({
        let tx: Sender<String> = tx.clone();
        move |ws: warp::ws::Ws| {
            let mut rx: Receiver<String> = tx.subscribe();

            ws.on_upgrade(move |websocket: warp::filters::ws::WebSocket| async move {
                let (mut tx_ws, _) = websocket.split();
                while rx.recv().await.is_ok() {
                    drop(tx_ws.send(warp::ws::Message::text("reload")).await);
                }
            })
        }
    });

    let html_route = warp::path::tail().and_then({
        let serve_dir: String = serve_dir.clone();
        let port: String = port.clone();

        move |path: warp::path::Tail| {
            let serve_dir: String = serve_dir.clone();
            let port: String = port.clone();

            async move {
                let path_str: &str = path.as_str();
                let full_path: String = format!("{}/{}", serve_dir, path_str);

                if path_str.is_empty() {
                    return Ok::<_, Rejection>(
                        Response::builder()
                            .status(301)
                            .header("location", "/nesmur/index.html")
                            .header("content-type", "text/plain; charset=utf-8")
                            .body("Redirecting to /nesmur/index.html".to_string())
                            .unwrap(),
                    );
                } else if path_str.ends_with(".html") {
                    if let Ok(content) = fs::read_to_string(&full_path) {
                        let injected: String = {
                            let script: String = format!(
                                r#"<script>
                                    const ws = new WebSocket("ws://localhost:{}/livereload");
                                    ws.onmessage = () => window.location.reload();
                                </script>"#,
                                port
                            );

                            if content.contains("</body>") {
                                content.replace("</body>", &format!("{}\n</body>", script))
                            } else if content.contains("</html>") {
                                content.replace("</html>", &format!("{}\n</html>", script))
                            } else {
                                format!("{}\n{}", content, script)
                            }
                        };

                        return Ok::<_, Rejection>(
                            Response::builder()
                                .header("content-type", "text/html; charset=utf8")
                                .body(injected)
                                .unwrap(),
                        );
                    }
                }

                Err(warp::reject::not_found())
            }
        }
    });

    let routes = ws_route.or(html_route).or(static_files);
    let host: IpAddr = host.parse().expect("Invalid IP address");
    let port_u16: u16 = port.parse().expect("Invalid port number");

    println!("Starting docs server at http://{}:{}", host, port);
    warp::serve(routes).run((host, port_u16)).await;
}
