// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// server interface

pub trait LayoutsApi {
    fn get_layouts(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::layouts::GetLayoutsRequest>, resp: ::grpc::ServerResponseUnarySink<super::layouts::Layouts>) -> ::grpc::Result<()>;
}

// client

pub struct LayoutsApiClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for LayoutsApiClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        LayoutsApiClient {
            grpc_client: grpc_client,
        }
    }
}

impl LayoutsApiClient {
    pub fn get_layouts(&self, o: ::grpc::RequestOptions, req: super::layouts::GetLayoutsRequest) -> ::grpc::SingleResponse<super::layouts::Layouts> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/mizer.LayoutsApi/GetLayouts"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct LayoutsApiServer;


impl LayoutsApiServer {
    pub fn new_service_def<H : LayoutsApi + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/mizer.LayoutsApi",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/mizer.LayoutsApi/GetLayouts"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).get_layouts(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}
