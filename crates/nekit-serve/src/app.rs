use std::{
    io::Error as IoError,
    path::{Path, PathBuf},
};

use axum::{Router, serve};
use clap::Parser;
use thiserror::Error;
use tokio::{
    net::TcpListener,
    runtime::{Builder, Runtime},
};

use crate::routes::router;

pub type Host<'h> = &'h str;
pub type Port = u16;

pub type Address<'a> = (Host<'a>, Port);

pub const DEFAULT_DIRECTORY: &str = "static";
pub const DEFAULT_HOST: Host = "0.0.0.0";
pub const DEFAULT_PORT: Port = 6969;

#[derive(Debug, Parser)]
#[command(about, author, version)]
pub struct App {
    #[arg(short = 'd', long, env = "NEKIT_SERVE_DIRECTORY", default_value = DEFAULT_DIRECTORY)]
    pub directory: PathBuf,

    #[arg(short = 'H', long, env = "NEKIT_SERVE_HOST", default_value = DEFAULT_HOST)]
    pub host: String,

    #[arg(short = 'P', long, env = "NEKIT_SERVE_PORT", default_value_t = DEFAULT_PORT)]
    pub port: Port,
}

#[derive(Debug, Error)]
#[error("failed to build runtime: {error}")]
pub struct BuildError {
    #[from]
    error: IoError,
}

impl BuildError {
    pub const fn new(error: IoError) -> Self {
        Self { error }
    }

    pub fn get(self) -> IoError {
        self.error
    }
}

#[derive(Debug, Error)]
#[error("failed to serve: {error}")]
pub struct ServeError {
    #[from]
    error: IoError,
}

impl ServeError {
    pub const fn new(error: IoError) -> Self {
        Self { error }
    }

    pub fn get(self) -> IoError {
        self.error
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Build(#[from] BuildError),
    Serve(#[from] ServeError),
}

impl App {
    pub fn runtime() -> Result<Runtime, BuildError> {
        let runtime = Builder::new_multi_thread().enable_all().build()?;

        Ok(runtime)
    }

    pub fn directory_path(&self) -> &Path {
        self.directory.as_path()
    }

    pub fn address(&self) -> Address<'_> {
        (self.host.as_str(), self.port)
    }

    pub fn router(&self) -> Router {
        router(self.directory_path())
    }

    pub async fn serve(&self) -> Result<(), ServeError> {
        let router = self.router();

        let listener = TcpListener::bind(self.address()).await?;

        serve(listener, router).await?;

        Ok(())
    }

    pub fn run(&self) -> Result<(), Error> {
        let runtime = Self::runtime()?;

        runtime.block_on(self.serve())?;

        Ok(())
    }
}
