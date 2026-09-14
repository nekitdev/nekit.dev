use std::io::Error as IoError;

use axum::{Router, serve};
use clap::Parser;
use thiserror::Error;
use tokio::{
    net::TcpListener,
    runtime::{Builder, Runtime},
};

use crate::routes::router;

pub type Address<'a> = (Host<'a>, Port);

pub type Host<'h> = &'h str;
pub type Port = u16;

pub const DEFAULT_HOST: Host = "0.0.0.0";
pub const DEFAULT_PORT: Port = 6913;

#[derive(Debug, Parser)]
#[command(about, author, version)]
pub struct App {
    #[arg(short = 'k', long, env = "NEKIT_BATTERY_KEY")]
    pub key: String,

    #[arg(short = 'H', long, env = "NEKIT_BATTERY_HOST", default_value = DEFAULT_HOST)]
    pub host: String,

    #[arg(short = 'P', long, env = "NEKIT_BATTERY_PORT", default_value_t = DEFAULT_PORT)]
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

    pub fn key(&self) -> &str {
        self.key.as_str()
    }

    pub fn address(&self) -> Address<'_> {
        (self.host.as_str(), self.port)
    }

    pub fn router(&self) -> Router {
        router(self.key())
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
