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

use tonic::transport::Channel;

use opentelemetry::proto::collector::trace::v1::trace_service_client::{TraceServiceClient};
use opentelemetry::proto::collector::trace::v1::{ExportTraceServiceRequest};
use opentelemetry::proto::trace::v1::{ResourceSpans, InstrumentationLibrarySpans, Span};
use fake::{Fake, Faker};


async fn trace_client() -> Result<TraceServiceClient<Channel>, Box<dyn std::error::Error>> {
    return Ok(TraceServiceClient::connect("http://[::1]:50051").await?);
}

#[tokio::test]
async fn my_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = trace_client().await?;

    let request = ExportTraceServiceRequest {
        resource_spans: vec![ ResourceSpans {
            resource: None,
            instrumentation_library_spans: vec![ InstrumentationLibrarySpans {
                instrumentation_library: None,
                spans: vec![Span {
                    trace_id: vec![(8..20).fake::<u8>()], 
                    span_id: vec![(8..20).fake::<u8>()], 
                    trace_state: Faker.fake::<String>(), 
                    parent_span_id: vec![(8..20).fake::<u8>()], 
                    name: Faker.fake::<String>(), 
                    kind: Faker.fake(), 
                    start_time_unix_nano: Faker.fake(), 
                    end_time_unix_nano: Faker.fake(), 
                    attributes: vec![], 
                    dropped_attributes_count: Faker.fake(), 
                    events: vec![],  
                    dropped_events_count: Faker.fake(), 
                    links: vec![],  
                    dropped_links_count: Faker.fake(), 
                    status: None,  
                    }],
                schema_url: "other schema".to_string(),
             } ],
            schema_url: "foo scheme".to_string(),
        }]
    };

    let response = client.export(request).await?;

    println!("{:?}", response);

    Ok(())
}