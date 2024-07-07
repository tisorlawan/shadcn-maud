use std::fs;
use std::io::Read;

use axum::{
    body::Bytes,
    extract::Multipart,
    http::HeaderMap,
    response::IntoResponse,
    routing::{get, post, put},
};
use maud::{html, Markup, PreEscaped};
use shadcnui_maud::web::prelude::*;
use tower_http::services::ServeDir;

#[derive(argh::FromArgs, PartialEq, Debug)]
/// Top-level command.
struct Arg {
    #[argh(subcommand)]
    cmd: MySubCommandEnum,
}

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum MySubCommandEnum {
    Init(InitCmdArgs),
    Build(BuildCmdArgs),
    Serve(ServeCmdArgs),
}

#[derive(argh::FromArgs, PartialEq, Debug)]
/// Initialize.
#[argh(subcommand, name = "init")]
struct InitCmdArgs {
    /// list of the files (with format: `fname` `dir_path` `url_download`).
    #[argh(option, default = "std::path::PathBuf::from(\"./static/files.txt\")")]
    files_list_path: std::path::PathBuf,

    /// base dir.
    #[argh(option, default = "std::path::PathBuf::from(\"./static\")")]
    base_dir: std::path::PathBuf,
}

#[derive(argh::FromArgs, PartialEq, Debug)]
/// Build.
#[argh(subcommand, name = "build")]
struct BuildCmdArgs {
    /// list of the files (with format: `fname` `dir_path` `url_download`).
    #[argh(option, default = "std::path::PathBuf::from(\"./static/files.txt\")")]
    files_list_path: std::path::PathBuf,

    /// base dir.
    #[argh(option, default = "std::path::PathBuf::from(\"./static\")")]
    base_dir: std::path::PathBuf,
}

#[derive(argh::FromArgs, PartialEq, Debug)]
/// Serve.
#[argh(subcommand, name = "serve")]
struct ServeCmdArgs {
    /// port.
    #[argh(option, default = "3000")]
    port: usize,
}

async fn serve(opts: ServeCmdArgs) {
    let route = axum::Router::new()
        .route("/", get(root_page))
        .route("/nice", get(nice))
        .route("/upload", post(upload))
        .nest_service(
            "/static",
            ServeDir::new("./static/dist").precompressed_gzip(),
        )
        .layer(axum::extract::DefaultBodyLimit::max(1024 * 1024 * 512)); // max 512 MB

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", opts.port))
        .await
        .expect("bind server");
    axum::serve(listener, route).await.expect("serve app");
}

#[tokio::main]
async fn main() {
    let arg: Arg = argh::from_env();

    match arg.cmd {
        MySubCommandEnum::Init(opts) => init(opts).await,
        MySubCommandEnum::Build(opts) => build(opts).await,
        MySubCommandEnum::Serve(opts) => {
            serve(opts).await;
        }
    }
}

pub async fn upload(mut multipart: Multipart) -> impl axum::response::IntoResponse {
    let mut data = Bytes::new();
    let mut name = String::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        name = field.name().unwrap().to_string();
        println!("{:?}", name);
        if name == "file" {
            data = field.bytes().await.unwrap();
            // break;
        }
    }

    "nice".into_response()
}

async fn root_page() -> Markup {
    html! {
        html lang="id" translate="no";
        head {
            title { "My Page" }
            script src="/static/js/htmx.min.js" {}
            script src="/static/js/hyperscript.min.js" {}
            link rel="stylesheet" type="text/css" href="/static/css/style.css";
        }
        body class="bg-background flex flex-col h-screen items-center justify-center" {
            div class="flex flex-col gap-4" {
                span {
                    "This text should change color in dark mode"
                }
                div class="flex gap-2"{
                    {(ui_theme_toggle())}
                    {(Button::new().hx_get("/nice").hx_swap("innerHTML").build(html! {"Primary"}))}
                    {(Button::secondary().build(html! {"Secondary"}))}
                    {(Button::outline().build(html! {"Outline"}))}
                    {(Button::ghost().build(html! {"Ghost"}))}
                    {(Button::destructive().build(html! {"Destructive"}))}
                    {(Button::link().build(html! {"Link"}))}
                }

                div {
                    {(Input::new().class("w-fit").ty("text").placeholder("username").build())}
                }

                form
                    hx-encoding="multipart/form-data"
                    hx-swap="none"
                    hx-post="/upload"
                    _="on htmx:xhr:progress(loaded, total) set #progress.value to (loaded/total)*100" {
                        {(FileUploader::new().name("file").build())}
                        div class="flex flex-col gap-2 mt-2"{
                            {(Button::new().class("w-fit").build(html! { "Submit" }))}
                            progress id="progress" value="0" max="100" {}
                        }
                    }
            }
        }
        script {
            (PreEscaped(include_str!("../scripts/theme_toggle.js")))
        }
    }
}

async fn init(opts: InitCmdArgs) {
    for (i, line) in fs::read_to_string(&opts.files_list_path)
        .expect("open files_list_path")
        .lines()
        .enumerate()
    {
        let cols = line.split_whitespace().collect::<Vec<_>>();
        if cols.is_empty() {
            continue;
        }
        if cols.len() != 3 {
            eprintln!(
                "Invalid line {} from {}: expected 3 columns, get {}",
                i + 1,
                &opts.files_list_path.to_str().expect("valid utf-8"),
                cols.len()
            );
            std::process::exit(1);
        }

        let fname = cols[0];
        let dir_path = cols[1];
        let url = cols[2];

        if !url.starts_with("https://") {
            continue;
        }

        let base_dir = opts.base_dir.join("files").join(dir_path);
        std::fs::create_dir_all(&base_dir).expect("create dir");

        let contents = ureq::get(url)
            .call()
            .expect("download file")
            .into_string()
            .expect("extract resul");

        std::fs::write(base_dir.join(fname), contents).expect("write result");
    }
}

async fn build(opts: BuildCmdArgs) {
    for (i, line) in fs::read_to_string(&opts.files_list_path)
        .expect("open files_list_path")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
    {
        let cols = line.split_whitespace().collect::<Vec<_>>();
        if cols.len() != 3 {
            eprintln!(
                "Invalid line {} from {}: expected 3 columns, get {}",
                i + 1,
                &opts.files_list_path.to_str().expect("valid utf-8"),
                cols.len()
            );
            std::process::exit(1);
        }

        let fname = cols[0];
        let dir_path = cols[1];
        let url = cols[2];

        let dist_dir = opts.base_dir.join("dist").join(dir_path);
        std::fs::create_dir_all(&dist_dir)
            .unwrap_or_else(|e| panic!("mkdir dist_dir: {e}: {dist_dir:?}"));

        let fpath = if url.starts_with("https://") {
            opts.base_dir.join("files").join(dir_path).join(fname)
        } else {
            opts.base_dir.join(url)
        };

        gzip(&fpath, &dist_dir).unwrap_or_else(|e| panic!("gzip: {e}: {dist_dir:?}"));
        if fpath.to_str().is_some_and(|s| s.contains("dist")) {
            std::fs::remove_file(&fpath).expect("remove file");
        }
    }
}

/// Compress `file_path` using gz compresssion.
/// The resulting file will be written to `output_dir` with additional `.gz` extension
pub fn gzip(
    file_path: impl AsRef<std::path::Path>,
    output_dir: impl AsRef<std::path::Path>,
) -> Result<(), std::io::Error> {
    let spath = file_path.as_ref();
    let output_dir = output_dir.as_ref();

    let mut z = flate2::bufread::GzEncoder::new(
        std::io::BufReader::new(std::fs::File::open(spath)?),
        flate2::Compression::default(),
    );
    let mut buffer = Vec::new();
    z.read_to_end(&mut buffer)?;

    let spath_fname = spath
        .file_name()
        .expect("filename")
        .to_str()
        .expect("valid utf-8 fname");
    let spath_extension = spath
        .extension()
        .expect("extension")
        .to_str()
        .expect("valid utf-8 extension");

    fs::write(
        std::path::PathBuf::from(output_dir)
            .join(spath_fname)
            .with_extension(format!("{}.gz", spath_extension)),
        buffer,
    )?;
    Ok(())
}

async fn nice() -> &'static str {
    "Nice"
}
