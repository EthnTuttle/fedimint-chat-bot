#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeInfoResponse {
    /// The public key of the associated lightning node
    #[prost(bytes = "vec", tag = "1")]
    pub pub_key: ::prost::alloc::vec::Vec<u8>,
    /// The alias of the lightning node
    #[prost(string, tag = "2")]
    pub alias: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayInvoiceRequest {
    #[prost(string, tag = "1")]
    pub invoice: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub max_delay: u64,
    #[prost(uint64, tag = "3")]
    pub max_fee_msat: u64,
    #[prost(bytes = "vec", tag = "4")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayInvoiceResponse {
    /// The preimage of the invoice
    #[prost(bytes = "vec", tag = "1")]
    pub preimage: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterceptHtlcRequest {
    /// The HTLC payment hash.
    /// Value is not guaranteed to be unique per intercepted HTLC
    #[prost(bytes = "vec", tag = "1")]
    pub payment_hash: ::prost::alloc::vec::Vec<u8>,
    /// The incoming HTLC amount in millisatoshi.
    /// This amount minus the `outgoing_amount_msat` is the fee paid for processing
    /// this intercepted HTLC
    #[prost(uint64, tag = "2")]
    pub incoming_amount_msat: u64,
    /// The outgoing HTLC amount in millisatoshi
    /// This is the amount we should forward to the Federation if we successfully
    /// process this intercepted HTLC
    #[prost(uint64, tag = "3")]
    pub outgoing_amount_msat: u64,
    /// The incoming HTLC expiry
    /// Determines block height when the node will automatically cancel and revert
    /// the intercepted HTLC to sender if it is not settled.
    #[prost(uint32, tag = "4")]
    pub incoming_expiry: u32,
    /// The short channel id of the HTLC.
    /// Use this value to confirm relevance of the intercepted HTLC
    #[prost(uint64, tag = "10")]
    pub short_channel_id: u64,
    /// The id of the incoming channel
    #[prost(uint64, tag = "12")]
    pub incoming_chan_id: u64,
    /// The index of the incoming htlc in the incoming channel
    #[prost(uint64, tag = "13")]
    pub htlc_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterceptHtlcResponse {
    /// The id of the incoming channel
    #[prost(uint64, tag = "4")]
    pub incoming_chan_id: u64,
    /// The index of the incoming htlc in the incoming channel
    #[prost(uint64, tag = "5")]
    pub htlc_id: u64,
    #[prost(oneof = "intercept_htlc_response::Action", tags = "1, 2, 3")]
    pub action: ::core::option::Option<intercept_htlc_response::Action>,
}
/// Nested message and enum types in `InterceptHtlcResponse`.
pub mod intercept_htlc_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Settle {
        /// The preimage for settling an intercepted HTLC
        #[prost(bytes = "vec", tag = "1")]
        pub preimage: ::prost::alloc::vec::Vec<u8>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Cancel {
        /// The reason for the cancellation of an intercepted HTLC
        #[prost(string, tag = "1")]
        pub reason: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Forward {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        /// Request to complete an intercepted HTLC with success result after
        /// processing
        ///
        /// Send this request when the gateway successfully processed intercepted
        /// HTLC GatewayLightning will settle/resolve the intercepted HTLC with
        /// reason provided.
        #[prost(message, tag = "1")]
        Settle(Settle),
        /// Request to complete an intercepted HTLC with failure result after
        /// processing
        ///
        /// Send this request when the gateway failed or canceled processing of
        /// intercepted HTLC. GatewayLightning will fail/cancel the intercepted HTLC
        /// with reason provided.
        #[prost(message, tag = "2")]
        Cancel(Cancel),
        /// Request to just forward the HTLC without failing or settling it.
        #[prost(message, tag = "3")]
        Forward(Forward),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRouteHintsResponse {
    /// The route hints to the associated lightning node
    #[prost(message, repeated, tag = "1")]
    pub route_hints: ::prost::alloc::vec::Vec<get_route_hints_response::RouteHint>,
}
/// Nested message and enum types in `GetRouteHintsResponse`.
pub mod get_route_hints_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteHintHop {
        /// The node_id of the non-target end of the route.
        #[prost(bytes = "vec", tag = "1")]
        pub src_node_id: ::prost::alloc::vec::Vec<u8>,
        /// The short_channel_id of this channel.
        #[prost(uint64, tag = "2")]
        pub short_channel_id: u64,
        /// Flat routing fee in millisatoshis.
        #[prost(uint32, tag = "3")]
        pub base_msat: u32,
        /// Liquidity-based routing fee in millionths of a routed amount.
        /// In other words, 10000 is 1%.
        #[prost(uint32, tag = "4")]
        pub proportional_millionths: u32,
        /// The difference in CLTV values between this node and the next node.
        #[prost(uint32, tag = "5")]
        pub cltv_expiry_delta: u32,
        /// The minimum value, in msat, which must be relayed to the next hop.
        #[prost(uint64, optional, tag = "6")]
        pub htlc_minimum_msat: ::core::option::Option<u64>,
        /// The maximum value in msat available for routing with a single HTLC.
        #[prost(uint64, optional, tag = "7")]
        pub htlc_maximum_msat: ::core::option::Option<u64>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RouteHint {
        /// Hops that make up a route hint to the associated lightning node
        #[prost(message, repeated, tag = "1")]
        pub hops: ::prost::alloc::vec::Vec<RouteHintHop>,
    }
}
/// Generated client implementations.
pub mod gateway_lightning_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    /// GatewayLightning is a service that provides limited access and functionality
    /// from a lightning node to Fedimint gateways
    #[derive(Debug, Clone)]
    pub struct GatewayLightningClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GatewayLightningClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GatewayLightningClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> GatewayLightningClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            GatewayLightningClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// GetNodeInfo returns the public key and alias of the associated lightning node
        pub async fn get_node_info(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::GetNodeInfoResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gatewaylnrpc.GatewayLightning/GetNodeInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetRouteHints returns the route hints to the associated lightning node
        pub async fn get_route_hints(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::GetRouteHintsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gatewaylnrpc.GatewayLightning/GetRouteHints",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// PayInvoice attempts to pay an invoice using the associated lightning node
        pub async fn pay_invoice(
            &mut self,
            request: impl tonic::IntoRequest<super::PayInvoiceRequest>,
        ) -> Result<tonic::Response<super::PayInvoiceResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gatewaylnrpc.GatewayLightning/PayInvoice",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        /// RouteHtlcs opens a bi-directional stream for the client to receive intercepted
        /// HTLCs. `InterceptHtlcRequest` is sent from the server to alert the client that
        /// an HTLC has been intercepted and needs to be processed. The client is expected
        /// to response with `InterceptHtlcResponse` after the HTLC has been processed with
        /// the appropriate action (Settle, Forward, Cancel).
        pub async fn route_htlcs(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::InterceptHtlcRequest>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gatewaylnrpc.GatewayLightning/RouteHtlcs",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn complete_htlc(
            &mut self,
            request: impl tonic::IntoRequest<super::InterceptHtlcResponse>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gatewaylnrpc.GatewayLightning/CompleteHtlc",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod gateway_lightning_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with GatewayLightningServer.
    #[async_trait]
    pub trait GatewayLightning: Send + Sync + 'static {
        /// GetNodeInfo returns the public key and alias of the associated lightning node
        async fn get_node_info(
            &self,
            request: tonic::Request<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::GetNodeInfoResponse>, tonic::Status>;
        /// GetRouteHints returns the route hints to the associated lightning node
        async fn get_route_hints(
            &self,
            request: tonic::Request<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::GetRouteHintsResponse>, tonic::Status>;
        ///
        /// PayInvoice attempts to pay an invoice using the associated lightning node
        async fn pay_invoice(
            &self,
            request: tonic::Request<super::PayInvoiceRequest>,
        ) -> Result<tonic::Response<super::PayInvoiceResponse>, tonic::Status>;
        /// Server streaming response type for the RouteHtlcs method.
        type RouteHtlcsStream: futures_core::Stream<
                Item = Result<super::InterceptHtlcRequest, tonic::Status>,
            >
            + Send
            + 'static;
        ///
        /// RouteHtlcs opens a bi-directional stream for the client to receive intercepted
        /// HTLCs. `InterceptHtlcRequest` is sent from the server to alert the client that
        /// an HTLC has been intercepted and needs to be processed. The client is expected
        /// to response with `InterceptHtlcResponse` after the HTLC has been processed with
        /// the appropriate action (Settle, Forward, Cancel).
        async fn route_htlcs(
            &self,
            request: tonic::Request<super::EmptyRequest>,
        ) -> Result<tonic::Response<Self::RouteHtlcsStream>, tonic::Status>;
        async fn complete_htlc(
            &self,
            request: tonic::Request<super::InterceptHtlcResponse>,
        ) -> Result<tonic::Response<super::EmptyResponse>, tonic::Status>;
    }
    ///
    /// GatewayLightning is a service that provides limited access and functionality
    /// from a lightning node to Fedimint gateways
    #[derive(Debug)]
    pub struct GatewayLightningServer<T: GatewayLightning> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: GatewayLightning> GatewayLightningServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GatewayLightningServer<T>
    where
        T: GatewayLightning,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/gatewaylnrpc.GatewayLightning/GetNodeInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetNodeInfoSvc<T: GatewayLightning>(pub Arc<T>);
                    impl<
                        T: GatewayLightning,
                    > tonic::server::UnaryService<super::EmptyRequest>
                    for GetNodeInfoSvc<T> {
                        type Response = super::GetNodeInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_node_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNodeInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gatewaylnrpc.GatewayLightning/GetRouteHints" => {
                    #[allow(non_camel_case_types)]
                    struct GetRouteHintsSvc<T: GatewayLightning>(pub Arc<T>);
                    impl<
                        T: GatewayLightning,
                    > tonic::server::UnaryService<super::EmptyRequest>
                    for GetRouteHintsSvc<T> {
                        type Response = super::GetRouteHintsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_route_hints(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRouteHintsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gatewaylnrpc.GatewayLightning/PayInvoice" => {
                    #[allow(non_camel_case_types)]
                    struct PayInvoiceSvc<T: GatewayLightning>(pub Arc<T>);
                    impl<
                        T: GatewayLightning,
                    > tonic::server::UnaryService<super::PayInvoiceRequest>
                    for PayInvoiceSvc<T> {
                        type Response = super::PayInvoiceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PayInvoiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pay_invoice(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PayInvoiceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gatewaylnrpc.GatewayLightning/RouteHtlcs" => {
                    #[allow(non_camel_case_types)]
                    struct RouteHtlcsSvc<T: GatewayLightning>(pub Arc<T>);
                    impl<
                        T: GatewayLightning,
                    > tonic::server::ServerStreamingService<super::EmptyRequest>
                    for RouteHtlcsSvc<T> {
                        type Response = super::InterceptHtlcRequest;
                        type ResponseStream = T::RouteHtlcsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).route_htlcs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RouteHtlcsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gatewaylnrpc.GatewayLightning/CompleteHtlc" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteHtlcSvc<T: GatewayLightning>(pub Arc<T>);
                    impl<
                        T: GatewayLightning,
                    > tonic::server::UnaryService<super::InterceptHtlcResponse>
                    for CompleteHtlcSvc<T> {
                        type Response = super::EmptyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InterceptHtlcResponse>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).complete_htlc(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CompleteHtlcSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: GatewayLightning> Clone for GatewayLightningServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: GatewayLightning> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GatewayLightning> tonic::server::NamedService for GatewayLightningServer<T> {
        const NAME: &'static str = "gatewaylnrpc.GatewayLightning";
    }
}
