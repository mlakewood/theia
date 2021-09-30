pub mod opentelemetry {
    pub mod proto {
        pub mod common {
            pub mod v1 {
                tonic::include_proto!("opentelemetry.proto.common.v1");
            }
        }

        pub mod resource {
            pub mod v1 {
                tonic::include_proto!("opentelemetry.proto.resource.v1");
            }
        }

        pub mod trace {
            pub mod v1 {
            tonic::include_proto!("opentelemetry.proto.trace.v1");
            }
        }

        pub mod collector {
            pub mod trace {
                pub mod v1 {
                    tonic::include_proto!("opentelemetry.proto.collector.trace.v1");
                }
            }
        }
    }
}

mod storage;

use opentelemetry::proto::collector::trace::v1::trace_service_server::{TraceServiceServer, TraceService};
use opentelemetry::proto::collector::trace::v1::{ExportTraceServiceRequest, ExportTraceServiceResponse};

use slog::{info, error, Logger};
use sloggers::Build;
use sloggers::terminal::{TerminalLoggerBuilder, Destination};
use sloggers::types::Severity;
use std::sync::{Arc, Mutex};
use storage::{MemoryStorage, StorageCommand};
use tonic::{transport::Server, Response};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use crate::storage::{StorageApi, StorageCommand::*};


pub struct Theia {
    log: Logger,
    storage_channel: Sender<StorageCommand>,
}


#[tonic::async_trait]
impl TraceService for Theia {
    async fn export(&self, request: tonic::Request<ExportTraceServiceRequest>) -> Result<tonic::Response<ExportTraceServiceResponse>, tonic::Status> {
        info!(self.log, "Export Endpoint hit:");

        let resource_spans = request.into_inner().resource_spans[0].clone();

        let cmd = StorageCommand::Insert {
            spans: resource_spans,
        };

        self.storage_channel.send(cmd).await.unwrap();

        let reply = ExportTraceServiceResponse {};

        Ok(Response::new(reply))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    // Build out the logger
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Debug);
    builder.destination(Destination::Stderr);
    let logger = builder.build().unwrap();
    let manager_logger = logger.clone();


    let (tx, mut rx) = mpsc::channel(32);
    
    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut storage = storage::MemoryStorage {
            spans: vec![],
        };
    
        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Insert { spans } => {
                    let result = storage.insert(spans).await;
                    match result {
                        Ok(_res) => {
                            info!(manager_logger, "Inserted spans correctly");
                            info!(manager_logger, "All Spans: {:?}", storage.spans.len());
                        },
                        Err(e) => error!(manager_logger, "Failed to insert spans! {:?}", e),
                    }
                    
                }
            }
        }
    });




    let theia = Theia {
        log: logger.clone(),
        storage_channel: tx.clone(),
    };

    info!(theia.log, "Theia listening on {}", addr);

    Server::builder()
        .add_service(TraceServiceServer::new(theia))
        .serve(addr)
        .await?;
    
    manager.await.unwrap();

    Ok(())
}
