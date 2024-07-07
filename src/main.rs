use std::fs;
use std::io::Read;

use axum::{
    body::Bytes,
    extract::Multipart,
    response::IntoResponse,
    routing::{get, put},
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
        .route("/upload", put(upload))
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
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let mut data = Bytes::new();
    let mut name = String::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        name = field.name().unwrap().to_string();
        if name == "file" {
            data = field.bytes().await.unwrap();
            break;
        }
    }

    if name != "file" {
        println!("not a file");
    }

    println!("File => {}", data.len());

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
                    {(Button::new().hx_get("/nice").hx_swap("innerHTML").build(html! { "Primary" }))}
                    {(Button::secondary().build(html! { "Secondary" }))}
                    {(Button::outline().build(html! { "Outline" }))}
                    {(Button::ghost().build(html! { "Ghost" }))}
                    {(Button::destructive().build(html! { "Destructive" }))}
                    {(Button::link().build(html! { "Link" }))}
                }

                div {
                    {(Input::new().class("w-fit").ty("text").placeholder("username").build())}
                }

                form
                    hx-encoding="multipart/form-data"
                    hx-swap="none"
                    hx-put="/upload"
                    _="on htmx:xhr:progress(loaded, total) set #progress.value to (loaded/total) * 100" {
                        {(FileUploader::new().class("border p-2").name("file").build())}

                        div class="flex gap-2 mt-2 items-center"{
                            {(Button::new().class("w-fit flex gap-2 items-center justify-center")
                              .build(html! {
                                  "Submit"
                                      // div role="status" id="upload-indicator" class="hidden htmx-indicator" {
                                      //     svg aria-hidden="true" class="inline w-6 h-6 text-gray-200 animate-spin dark:text-gray-600 fill-gray-600 dark:fill-gray-300" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg" {
                                      //         path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor" {}
                                      //         path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill" {}
                                      //     }
                                      //     span class="sr-only" { "Loading..." }
                                      // }
                              }))}
                            progress class="text-red-600 bg-blue-400" id="progress" value="0" max="100" {}
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
