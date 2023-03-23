#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    #[prost(string, tag = "1")]
    pub offset: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shard {
    #[prost(string, tag = "1")]
    pub index_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub shard_id: u64,
    #[prost(string, tag = "4")]
    pub leader_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub follower_id: ::prost::alloc::string::String,
    #[prost(enumeration = "ShardStatus", tag = "6")]
    pub shard_status: i32,
    #[prost(message, optional, tag = "7")]
    pub consumer_position: ::core::option::Option<Position>,
    #[prost(message, optional, tag = "8")]
    pub closed_position: ::core::option::Option<Position>,
    #[prost(message, optional, tag = "9")]
    pub closed_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "10")]
    pub create_timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenShardRequest {
    #[prost(string, tag = "1")]
    pub index_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub shard_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenShardResponse {
    #[prost(message, optional, tag = "1")]
    pub shard: ::core::option::Option<Shard>,
}
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShardRequest {
    #[prost(string, tag = "1")]
    pub index_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub shard_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShardResponse {
    #[prost(message, optional, tag = "1")]
    pub shard: ::core::option::Option<Shard>,
}
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateShardRequest {
    #[prost(message, optional, tag = "1")]
    pub shard: ::core::option::Option<Shard>,
}
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateShardResponse {
    #[prost(message, optional, tag = "1")]
    pub shard: ::core::option::Option<Shard>,
}
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseShardRequest {
    #[prost(string, tag = "1")]
    pub index_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub shard_id: u64,
    #[prost(message, optional, tag = "4")]
    pub position: ::core::option::Option<Position>,
}
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseShardResponse {
    #[prost(message, optional, tag = "1")]
    pub shard: ::core::option::Option<Shard>,
}
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShardRequest {
    #[prost(string, tag = "1")]
    pub index_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub shard_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShardResponse {}
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListShardsRequest {
    #[prost(string, tag = "1")]
    pub index_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub shard_id: u64,
    #[prost(enumeration = "ShardStatus", optional, tag = "6")]
    pub shard_status: ::core::option::Option<i32>,
}
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListShardsResponse {
    #[prost(message, repeated, tag = "1")]
    pub shards: ::prost::alloc::vec::Vec<Shard>,
}
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ShardStatus {
    Open = 0,
    Closed = 1,
}
impl ShardStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ShardStatus::Open => "OPEN",
            ShardStatus::Closed => "CLOSED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPEN" => Some(Self::Open),
            "CLOSED" => Some(Self::Closed),
            _ => None,
        }
    }
}
/// BEGIN quickwit-codegen
#[cfg_attr(any(test, feature = "testsuite"), mockall::automock)]
#[async_trait::async_trait]
pub trait ShardService: std::fmt::Debug + dyn_clone::DynClone + Send + Sync + 'static {
    async fn open_shard(
        &mut self,
        request: OpenShardRequest,
    ) -> crate::ingest::Result<OpenShardResponse>;
    async fn get_shard(
        &mut self,
        request: GetShardRequest,
    ) -> crate::ingest::Result<GetShardResponse>;
    async fn update_shard(
        &mut self,
        request: UpdateShardRequest,
    ) -> crate::ingest::Result<UpdateShardResponse>;
    async fn close_shard(
        &mut self,
        request: CloseShardRequest,
    ) -> crate::ingest::Result<CloseShardResponse>;
    async fn delete_shard(
        &mut self,
        request: DeleteShardRequest,
    ) -> crate::ingest::Result<DeleteShardResponse>;
    async fn list_shards(
        &mut self,
        request: ListShardsRequest,
    ) -> crate::ingest::Result<ListShardsResponse>;
}
dyn_clone::clone_trait_object!(ShardService);
#[cfg(any(test, feature = "testsuite"))]
impl Clone for MockShardService {
    fn clone(&self) -> Self {
        MockShardService::new()
    }
}
#[derive(Debug, Clone)]
pub struct ShardServiceClient {
    inner: Box<dyn ShardService>,
}
impl ShardServiceClient {
    pub fn new<T>(instance: T) -> Self
    where
        T: ShardService,
    {
        Self { inner: Box::new(instance) }
    }
    pub fn from_channel(
        channel: tower::timeout::Timeout<tonic::transport::Channel>,
    ) -> Self {
        ShardServiceClient::new(
            ShardServiceGrpcClientAdapter::new(
                shard_service_grpc_client::ShardServiceGrpcClient::new(channel),
            ),
        )
    }
    pub fn from_mailbox<A>(mailbox: quickwit_actors::Mailbox<A>) -> Self
    where
        A: quickwit_actors::Actor + std::fmt::Debug + Send + Sync + 'static,
        ShardServiceMailbox<A>: ShardService,
    {
        ShardServiceClient::new(ShardServiceMailbox::new(mailbox))
    }
    pub fn tower() -> ShardServiceTowerBlockBuilder {
        ShardServiceTowerBlockBuilder::default()
    }
    #[cfg(any(test, feature = "testsuite"))]
    pub fn mock() -> MockShardService {
        MockShardService::new()
    }
}
#[async_trait::async_trait]
impl ShardService for ShardServiceClient {
    async fn open_shard(
        &mut self,
        request: OpenShardRequest,
    ) -> crate::ingest::Result<OpenShardResponse> {
        self.inner.open_shard(request).await
    }
    async fn get_shard(
        &mut self,
        request: GetShardRequest,
    ) -> crate::ingest::Result<GetShardResponse> {
        self.inner.get_shard(request).await
    }
    async fn update_shard(
        &mut self,
        request: UpdateShardRequest,
    ) -> crate::ingest::Result<UpdateShardResponse> {
        self.inner.update_shard(request).await
    }
    async fn close_shard(
        &mut self,
        request: CloseShardRequest,
    ) -> crate::ingest::Result<CloseShardResponse> {
        self.inner.close_shard(request).await
    }
    async fn delete_shard(
        &mut self,
        request: DeleteShardRequest,
    ) -> crate::ingest::Result<DeleteShardResponse> {
        self.inner.delete_shard(request).await
    }
    async fn list_shards(
        &mut self,
        request: ListShardsRequest,
    ) -> crate::ingest::Result<ListShardsResponse> {
        self.inner.list_shards(request).await
    }
}
#[cfg(any(test, feature = "testsuite"))]
impl From<MockShardService> for ShardServiceClient {
    fn from(mock: MockShardService) -> Self {
        ShardServiceClient::new(mock)
    }
}
pub type BoxFuture<T, E> = std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<T, E>> + Send + 'static>,
>;
impl tower::Service<OpenShardRequest> for Box<dyn ShardService> {
    type Response = OpenShardResponse;
    type Error = crate::ingest::ShardServiceError;
    type Future = BoxFuture<Self::Response, Self::Error>;
    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
    fn call(&mut self, request: OpenShardRequest) -> Self::Future {
        let mut svc = self.clone();
        let fut = async move { svc.open_shard(request).await };
        Box::pin(fut)
    }
}
impl tower::Service<GetShardRequest> for Box<dyn ShardService> {
    type Response = GetShardResponse;
    type Error = crate::ingest::ShardServiceError;
    type Future = BoxFuture<Self::Response, Self::Error>;
    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
    fn call(&mut self, request: GetShardRequest) -> Self::Future {
        let mut svc = self.clone();
        let fut = async move { svc.get_shard(request).await };
        Box::pin(fut)
    }
}
impl tower::Service<UpdateShardRequest> for Box<dyn ShardService> {
    type Response = UpdateShardResponse;
    type Error = crate::ingest::ShardServiceError;
    type Future = BoxFuture<Self::Response, Self::Error>;
    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
    fn call(&mut self, request: UpdateShardRequest) -> Self::Future {
        let mut svc = self.clone();
        let fut = async move { svc.update_shard(request).await };
        Box::pin(fut)
    }
}
impl tower::Service<CloseShardRequest> for Box<dyn ShardService> {
    type Response = CloseShardResponse;
    type Error = crate::ingest::ShardServiceError;
    type Future = BoxFuture<Self::Response, Self::Error>;
    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
    fn call(&mut self, request: CloseShardRequest) -> Self::Future {
        let mut svc = self.clone();
        let fut = async move { svc.close_shard(request).await };
        Box::pin(fut)
    }
}
impl tower::Service<DeleteShardRequest> for Box<dyn ShardService> {
    type Response = DeleteShardResponse;
    type Error = crate::ingest::ShardServiceError;
    type Future = BoxFuture<Self::Response, Self::Error>;
    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
    fn call(&mut self, request: DeleteShardRequest) -> Self::Future {
        let mut svc = self.clone();
        let fut = async move { svc.delete_shard(request).await };
        Box::pin(fut)
    }
}
impl tower::Service<ListShardsRequest> for Box<dyn ShardService> {
    type Response = ListShardsResponse;
    type Error = crate::ingest::ShardServiceError;
    type Future = BoxFuture<Self::Response, Self::Error>;
    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }
    fn call(&mut self, request: ListShardsRequest) -> Self::Future {
        let mut svc = self.clone();
        let fut = async move { svc.list_shards(request).await };
        Box::pin(fut)
    }
}
/// A tower block is a set of towers. Each tower is stack of layers (middlewares) that are applied to a service.
#[derive(Debug)]
struct ShardServiceTowerBlock {
    open_shard_svc: quickwit_common::tower::BoxService<
        OpenShardRequest,
        OpenShardResponse,
        crate::ingest::ShardServiceError,
    >,
    get_shard_svc: quickwit_common::tower::BoxService<
        GetShardRequest,
        GetShardResponse,
        crate::ingest::ShardServiceError,
    >,
    update_shard_svc: quickwit_common::tower::BoxService<
        UpdateShardRequest,
        UpdateShardResponse,
        crate::ingest::ShardServiceError,
    >,
    close_shard_svc: quickwit_common::tower::BoxService<
        CloseShardRequest,
        CloseShardResponse,
        crate::ingest::ShardServiceError,
    >,
    delete_shard_svc: quickwit_common::tower::BoxService<
        DeleteShardRequest,
        DeleteShardResponse,
        crate::ingest::ShardServiceError,
    >,
    list_shards_svc: quickwit_common::tower::BoxService<
        ListShardsRequest,
        ListShardsResponse,
        crate::ingest::ShardServiceError,
    >,
}
impl Clone for ShardServiceTowerBlock {
    fn clone(&self) -> Self {
        Self {
            open_shard_svc: self.open_shard_svc.clone(),
            get_shard_svc: self.get_shard_svc.clone(),
            update_shard_svc: self.update_shard_svc.clone(),
            close_shard_svc: self.close_shard_svc.clone(),
            delete_shard_svc: self.delete_shard_svc.clone(),
            list_shards_svc: self.list_shards_svc.clone(),
        }
    }
}
#[async_trait::async_trait]
impl ShardService for ShardServiceTowerBlock {
    async fn open_shard(
        &mut self,
        request: OpenShardRequest,
    ) -> crate::ingest::Result<OpenShardResponse> {
        self.open_shard_svc.ready().await?.call(request).await
    }
    async fn get_shard(
        &mut self,
        request: GetShardRequest,
    ) -> crate::ingest::Result<GetShardResponse> {
        self.get_shard_svc.ready().await?.call(request).await
    }
    async fn update_shard(
        &mut self,
        request: UpdateShardRequest,
    ) -> crate::ingest::Result<UpdateShardResponse> {
        self.update_shard_svc.ready().await?.call(request).await
    }
    async fn close_shard(
        &mut self,
        request: CloseShardRequest,
    ) -> crate::ingest::Result<CloseShardResponse> {
        self.close_shard_svc.ready().await?.call(request).await
    }
    async fn delete_shard(
        &mut self,
        request: DeleteShardRequest,
    ) -> crate::ingest::Result<DeleteShardResponse> {
        self.delete_shard_svc.ready().await?.call(request).await
    }
    async fn list_shards(
        &mut self,
        request: ListShardsRequest,
    ) -> crate::ingest::Result<ListShardsResponse> {
        self.list_shards_svc.ready().await?.call(request).await
    }
}
#[derive(Debug, Default)]
pub struct ShardServiceTowerBlockBuilder {
    open_shard_layer: Option<
        quickwit_common::tower::BoxLayer<
            Box<dyn ShardService>,
            OpenShardRequest,
            OpenShardResponse,
            crate::ingest::ShardServiceError,
        >,
    >,
    get_shard_layer: Option<
        quickwit_common::tower::BoxLayer<
            Box<dyn ShardService>,
            GetShardRequest,
            GetShardResponse,
            crate::ingest::ShardServiceError,
        >,
    >,
    update_shard_layer: Option<
        quickwit_common::tower::BoxLayer<
            Box<dyn ShardService>,
            UpdateShardRequest,
            UpdateShardResponse,
            crate::ingest::ShardServiceError,
        >,
    >,
    close_shard_layer: Option<
        quickwit_common::tower::BoxLayer<
            Box<dyn ShardService>,
            CloseShardRequest,
            CloseShardResponse,
            crate::ingest::ShardServiceError,
        >,
    >,
    delete_shard_layer: Option<
        quickwit_common::tower::BoxLayer<
            Box<dyn ShardService>,
            DeleteShardRequest,
            DeleteShardResponse,
            crate::ingest::ShardServiceError,
        >,
    >,
    list_shards_layer: Option<
        quickwit_common::tower::BoxLayer<
            Box<dyn ShardService>,
            ListShardsRequest,
            ListShardsResponse,
            crate::ingest::ShardServiceError,
        >,
    >,
}
impl ShardServiceTowerBlockBuilder {
    pub fn open_shard_layer(
        mut self,
        layer: quickwit_common::tower::BoxLayer<
            Box<dyn ShardService>,
            OpenShardRequest,
            OpenShardResponse,
            crate::ingest::ShardServiceError,
        >,
    ) -> Self {
        self.open_shard_layer = Some(layer);
        self
    }
    pub fn get_shard_layer(
        mut self,
        layer: quickwit_common::tower::BoxLayer<
            Box<dyn ShardService>,
            GetShardRequest,
            GetShardResponse,
            crate::ingest::ShardServiceError,
        >,
    ) -> Self {
        self.get_shard_layer = Some(layer);
        self
    }
    pub fn update_shard_layer(
        mut self,
        layer: quickwit_common::tower::BoxLayer<
            Box<dyn ShardService>,
            UpdateShardRequest,
            UpdateShardResponse,
            crate::ingest::ShardServiceError,
        >,
    ) -> Self {
        self.update_shard_layer = Some(layer);
        self
    }
    pub fn close_shard_layer(
        mut self,
        layer: quickwit_common::tower::BoxLayer<
            Box<dyn ShardService>,
            CloseShardRequest,
            CloseShardResponse,
            crate::ingest::ShardServiceError,
        >,
    ) -> Self {
        self.close_shard_layer = Some(layer);
        self
    }
    pub fn delete_shard_layer(
        mut self,
        layer: quickwit_common::tower::BoxLayer<
            Box<dyn ShardService>,
            DeleteShardRequest,
            DeleteShardResponse,
            crate::ingest::ShardServiceError,
        >,
    ) -> Self {
        self.delete_shard_layer = Some(layer);
        self
    }
    pub fn list_shards_layer(
        mut self,
        layer: quickwit_common::tower::BoxLayer<
            Box<dyn ShardService>,
            ListShardsRequest,
            ListShardsResponse,
            crate::ingest::ShardServiceError,
        >,
    ) -> Self {
        self.list_shards_layer = Some(layer);
        self
    }
    pub fn service<T>(self, instance: T) -> ShardServiceClient
    where
        T: ShardService + Clone,
    {
        let boxed_instance: Box<dyn ShardService> = Box::new(instance);
        let open_shard_svc = if let Some(layer) = self.open_shard_layer {
            layer.layer(boxed_instance.clone())
        } else {
            quickwit_common::tower::BoxService::new(boxed_instance.clone())
        };
        let get_shard_svc = if let Some(layer) = self.get_shard_layer {
            layer.layer(boxed_instance.clone())
        } else {
            quickwit_common::tower::BoxService::new(boxed_instance.clone())
        };
        let update_shard_svc = if let Some(layer) = self.update_shard_layer {
            layer.layer(boxed_instance.clone())
        } else {
            quickwit_common::tower::BoxService::new(boxed_instance.clone())
        };
        let close_shard_svc = if let Some(layer) = self.close_shard_layer {
            layer.layer(boxed_instance.clone())
        } else {
            quickwit_common::tower::BoxService::new(boxed_instance.clone())
        };
        let delete_shard_svc = if let Some(layer) = self.delete_shard_layer {
            layer.layer(boxed_instance.clone())
        } else {
            quickwit_common::tower::BoxService::new(boxed_instance.clone())
        };
        let list_shards_svc = if let Some(layer) = self.list_shards_layer {
            layer.layer(boxed_instance.clone())
        } else {
            quickwit_common::tower::BoxService::new(boxed_instance.clone())
        };
        let tower_block = ShardServiceTowerBlock {
            open_shard_svc,
            get_shard_svc,
            update_shard_svc,
            close_shard_svc,
            delete_shard_svc,
            list_shards_svc,
        };
        ShardServiceClient::new(tower_block)
    }
}
#[derive(Debug, Clone)]
struct MailboxAdapter<A: quickwit_actors::Actor, E> {
    inner: quickwit_actors::Mailbox<A>,
    phantom: std::marker::PhantomData<E>,
}
impl<A, E> std::ops::Deref for MailboxAdapter<A, E>
where
    A: quickwit_actors::Actor,
{
    type Target = quickwit_actors::Mailbox<A>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
#[derive(Debug)]
pub struct ShardServiceMailbox<A: quickwit_actors::Actor> {
    inner: MailboxAdapter<A, crate::ingest::ShardServiceError>,
}
impl<A: quickwit_actors::Actor> ShardServiceMailbox<A> {
    pub fn new(instance: quickwit_actors::Mailbox<A>) -> Self {
        let inner = MailboxAdapter {
            inner: instance,
            phantom: std::marker::PhantomData,
        };
        Self { inner }
    }
}
impl<A: quickwit_actors::Actor> Clone for ShardServiceMailbox<A> {
    fn clone(&self) -> Self {
        let inner = MailboxAdapter {
            inner: self.inner.clone(),
            phantom: std::marker::PhantomData,
        };
        Self { inner }
    }
}
use tower::{Layer, Service, ServiceExt};
impl<A, M, T, E> tower::Service<M> for ShardServiceMailbox<A>
where
    A: quickwit_actors::Actor
        + quickwit_actors::DeferableReplyHandler<M, Reply = Result<T, E>> + Send + Sync
        + 'static,
    M: std::fmt::Debug + Send + Sync + 'static,
    T: Send + Sync + 'static,
    E: std::fmt::Debug + Send + Sync + 'static,
    crate::ingest::ShardServiceError: From<quickwit_actors::AskError<E>>,
{
    type Response = T;
    type Error = crate::ingest::ShardServiceError;
    type Future = BoxFuture<Self::Response, Self::Error>;
    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        //! This does not work with balance middlewares such as `tower::balance::pool::Pool` because
        //! this always returns `Poll::Ready`. The fix is to acquire a permit from the
        //! mailbox in `poll_ready` and consume it in `call`.
        std::task::Poll::Ready(Ok(()))
    }
    fn call(&mut self, message: M) -> Self::Future {
        let mailbox = self.inner.clone();
        let fut = async move {
            mailbox.ask_for_res(message).await.map_err(|error| error.into())
        };
        Box::pin(fut)
    }
}
#[async_trait::async_trait]
impl<A> ShardService for ShardServiceMailbox<A>
where
    A: quickwit_actors::Actor + std::fmt::Debug + Send + Sync + 'static,
    ShardServiceMailbox<
        A,
    >: tower::Service<
            OpenShardRequest,
            Response = OpenShardResponse,
            Error = crate::ingest::ShardServiceError,
            Future = BoxFuture<OpenShardResponse, crate::ingest::ShardServiceError>,
        >
        + tower::Service<
            GetShardRequest,
            Response = GetShardResponse,
            Error = crate::ingest::ShardServiceError,
            Future = BoxFuture<GetShardResponse, crate::ingest::ShardServiceError>,
        >
        + tower::Service<
            UpdateShardRequest,
            Response = UpdateShardResponse,
            Error = crate::ingest::ShardServiceError,
            Future = BoxFuture<UpdateShardResponse, crate::ingest::ShardServiceError>,
        >
        + tower::Service<
            CloseShardRequest,
            Response = CloseShardResponse,
            Error = crate::ingest::ShardServiceError,
            Future = BoxFuture<CloseShardResponse, crate::ingest::ShardServiceError>,
        >
        + tower::Service<
            DeleteShardRequest,
            Response = DeleteShardResponse,
            Error = crate::ingest::ShardServiceError,
            Future = BoxFuture<DeleteShardResponse, crate::ingest::ShardServiceError>,
        >
        + tower::Service<
            ListShardsRequest,
            Response = ListShardsResponse,
            Error = crate::ingest::ShardServiceError,
            Future = BoxFuture<ListShardsResponse, crate::ingest::ShardServiceError>,
        >,
{
    async fn open_shard(
        &mut self,
        request: OpenShardRequest,
    ) -> crate::ingest::Result<OpenShardResponse> {
        self.call(request).await
    }
    async fn get_shard(
        &mut self,
        request: GetShardRequest,
    ) -> crate::ingest::Result<GetShardResponse> {
        self.call(request).await
    }
    async fn update_shard(
        &mut self,
        request: UpdateShardRequest,
    ) -> crate::ingest::Result<UpdateShardResponse> {
        self.call(request).await
    }
    async fn close_shard(
        &mut self,
        request: CloseShardRequest,
    ) -> crate::ingest::Result<CloseShardResponse> {
        self.call(request).await
    }
    async fn delete_shard(
        &mut self,
        request: DeleteShardRequest,
    ) -> crate::ingest::Result<DeleteShardResponse> {
        self.call(request).await
    }
    async fn list_shards(
        &mut self,
        request: ListShardsRequest,
    ) -> crate::ingest::Result<ListShardsResponse> {
        self.call(request).await
    }
}
#[derive(Debug, Clone)]
pub struct ShardServiceGrpcClientAdapter<T> {
    inner: T,
}
impl<T> ShardServiceGrpcClientAdapter<T> {
    pub fn new(instance: T) -> Self {
        Self { inner: instance }
    }
}
#[async_trait::async_trait]
impl<T> ShardService
for ShardServiceGrpcClientAdapter<shard_service_grpc_client::ShardServiceGrpcClient<T>>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody> + std::fmt::Debug + Clone + Send
        + Sync + 'static,
    T::ResponseBody: tonic::codegen::Body<Data = tonic::codegen::Bytes> + Send + 'static,
    <T::ResponseBody as tonic::codegen::Body>::Error: Into<tonic::codegen::StdError>
        + Send,
    T::Future: Send,
{
    async fn open_shard(
        &mut self,
        request: OpenShardRequest,
    ) -> crate::ingest::Result<OpenShardResponse> {
        self.inner
            .open_shard(request)
            .await
            .map(|response| response.into_inner())
            .map_err(|error| error.into())
    }
    async fn get_shard(
        &mut self,
        request: GetShardRequest,
    ) -> crate::ingest::Result<GetShardResponse> {
        self.inner
            .get_shard(request)
            .await
            .map(|response| response.into_inner())
            .map_err(|error| error.into())
    }
    async fn update_shard(
        &mut self,
        request: UpdateShardRequest,
    ) -> crate::ingest::Result<UpdateShardResponse> {
        self.inner
            .update_shard(request)
            .await
            .map(|response| response.into_inner())
            .map_err(|error| error.into())
    }
    async fn close_shard(
        &mut self,
        request: CloseShardRequest,
    ) -> crate::ingest::Result<CloseShardResponse> {
        self.inner
            .close_shard(request)
            .await
            .map(|response| response.into_inner())
            .map_err(|error| error.into())
    }
    async fn delete_shard(
        &mut self,
        request: DeleteShardRequest,
    ) -> crate::ingest::Result<DeleteShardResponse> {
        self.inner
            .delete_shard(request)
            .await
            .map(|response| response.into_inner())
            .map_err(|error| error.into())
    }
    async fn list_shards(
        &mut self,
        request: ListShardsRequest,
    ) -> crate::ingest::Result<ListShardsResponse> {
        self.inner
            .list_shards(request)
            .await
            .map(|response| response.into_inner())
            .map_err(|error| error.into())
    }
}
#[derive(Debug)]
pub struct ShardServiceGrpcServerAdapter {
    inner: Box<dyn ShardService>,
}
impl ShardServiceGrpcServerAdapter {
    pub fn new<T>(instance: T) -> Self
    where
        T: ShardService,
    {
        Self { inner: Box::new(instance) }
    }
}
#[async_trait::async_trait]
impl shard_service_grpc_server::ShardServiceGrpc for ShardServiceGrpcServerAdapter {
    async fn open_shard(
        &self,
        request: tonic::Request<OpenShardRequest>,
    ) -> Result<tonic::Response<OpenShardResponse>, tonic::Status> {
        self.inner
            .clone()
            .open_shard(request.into_inner())
            .await
            .map(tonic::Response::new)
            .map_err(Into::into)
    }
    async fn get_shard(
        &self,
        request: tonic::Request<GetShardRequest>,
    ) -> Result<tonic::Response<GetShardResponse>, tonic::Status> {
        self.inner
            .clone()
            .get_shard(request.into_inner())
            .await
            .map(tonic::Response::new)
            .map_err(Into::into)
    }
    async fn update_shard(
        &self,
        request: tonic::Request<UpdateShardRequest>,
    ) -> Result<tonic::Response<UpdateShardResponse>, tonic::Status> {
        self.inner
            .clone()
            .update_shard(request.into_inner())
            .await
            .map(tonic::Response::new)
            .map_err(Into::into)
    }
    async fn close_shard(
        &self,
        request: tonic::Request<CloseShardRequest>,
    ) -> Result<tonic::Response<CloseShardResponse>, tonic::Status> {
        self.inner
            .clone()
            .close_shard(request.into_inner())
            .await
            .map(tonic::Response::new)
            .map_err(Into::into)
    }
    async fn delete_shard(
        &self,
        request: tonic::Request<DeleteShardRequest>,
    ) -> Result<tonic::Response<DeleteShardResponse>, tonic::Status> {
        self.inner
            .clone()
            .delete_shard(request.into_inner())
            .await
            .map(tonic::Response::new)
            .map_err(Into::into)
    }
    async fn list_shards(
        &self,
        request: tonic::Request<ListShardsRequest>,
    ) -> Result<tonic::Response<ListShardsResponse>, tonic::Status> {
        self.inner
            .clone()
            .list_shards(request.into_inner())
            .await
            .map(tonic::Response::new)
            .map_err(Into::into)
    }
}
/// Generated client implementations.
pub mod shard_service_grpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ShardServiceGrpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ShardServiceGrpcClient<tonic::transport::Channel> {
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
    impl<T> ShardServiceGrpcClient<T>
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
        ) -> ShardServiceGrpcClient<InterceptedService<T, F>>
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
            ShardServiceGrpcClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn open_shard(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenShardRequest>,
        ) -> Result<tonic::Response<super::OpenShardResponse>, tonic::Status> {
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
                "/shard_service.ShardService/OpenShard",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_shard(
            &mut self,
            request: impl tonic::IntoRequest<super::GetShardRequest>,
        ) -> Result<tonic::Response<super::GetShardResponse>, tonic::Status> {
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
                "/shard_service.ShardService/GetShard",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_shard(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateShardRequest>,
        ) -> Result<tonic::Response<super::UpdateShardResponse>, tonic::Status> {
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
                "/shard_service.ShardService/UpdateShard",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn close_shard(
            &mut self,
            request: impl tonic::IntoRequest<super::CloseShardRequest>,
        ) -> Result<tonic::Response<super::CloseShardResponse>, tonic::Status> {
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
                "/shard_service.ShardService/CloseShard",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_shard(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteShardRequest>,
        ) -> Result<tonic::Response<super::DeleteShardResponse>, tonic::Status> {
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
                "/shard_service.ShardService/DeleteShard",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_shards(
            &mut self,
            request: impl tonic::IntoRequest<super::ListShardsRequest>,
        ) -> Result<tonic::Response<super::ListShardsResponse>, tonic::Status> {
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
                "/shard_service.ShardService/ListShards",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod shard_service_grpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ShardServiceGrpcServer.
    #[async_trait]
    pub trait ShardServiceGrpc: Send + Sync + 'static {
        async fn open_shard(
            &self,
            request: tonic::Request<super::OpenShardRequest>,
        ) -> Result<tonic::Response<super::OpenShardResponse>, tonic::Status>;
        async fn get_shard(
            &self,
            request: tonic::Request<super::GetShardRequest>,
        ) -> Result<tonic::Response<super::GetShardResponse>, tonic::Status>;
        async fn update_shard(
            &self,
            request: tonic::Request<super::UpdateShardRequest>,
        ) -> Result<tonic::Response<super::UpdateShardResponse>, tonic::Status>;
        async fn close_shard(
            &self,
            request: tonic::Request<super::CloseShardRequest>,
        ) -> Result<tonic::Response<super::CloseShardResponse>, tonic::Status>;
        async fn delete_shard(
            &self,
            request: tonic::Request<super::DeleteShardRequest>,
        ) -> Result<tonic::Response<super::DeleteShardResponse>, tonic::Status>;
        async fn list_shards(
            &self,
            request: tonic::Request<super::ListShardsRequest>,
        ) -> Result<tonic::Response<super::ListShardsResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ShardServiceGrpcServer<T: ShardServiceGrpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ShardServiceGrpc> ShardServiceGrpcServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ShardServiceGrpcServer<T>
    where
        T: ShardServiceGrpc,
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
                "/shard_service.ShardService/OpenShard" => {
                    #[allow(non_camel_case_types)]
                    struct OpenShardSvc<T: ShardServiceGrpc>(pub Arc<T>);
                    impl<
                        T: ShardServiceGrpc,
                    > tonic::server::UnaryService<super::OpenShardRequest>
                    for OpenShardSvc<T> {
                        type Response = super::OpenShardResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OpenShardRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).open_shard(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OpenShardSvc(inner);
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
                "/shard_service.ShardService/GetShard" => {
                    #[allow(non_camel_case_types)]
                    struct GetShardSvc<T: ShardServiceGrpc>(pub Arc<T>);
                    impl<
                        T: ShardServiceGrpc,
                    > tonic::server::UnaryService<super::GetShardRequest>
                    for GetShardSvc<T> {
                        type Response = super::GetShardResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetShardRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_shard(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetShardSvc(inner);
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
                "/shard_service.ShardService/UpdateShard" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateShardSvc<T: ShardServiceGrpc>(pub Arc<T>);
                    impl<
                        T: ShardServiceGrpc,
                    > tonic::server::UnaryService<super::UpdateShardRequest>
                    for UpdateShardSvc<T> {
                        type Response = super::UpdateShardResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateShardRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_shard(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateShardSvc(inner);
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
                "/shard_service.ShardService/CloseShard" => {
                    #[allow(non_camel_case_types)]
                    struct CloseShardSvc<T: ShardServiceGrpc>(pub Arc<T>);
                    impl<
                        T: ShardServiceGrpc,
                    > tonic::server::UnaryService<super::CloseShardRequest>
                    for CloseShardSvc<T> {
                        type Response = super::CloseShardResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CloseShardRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).close_shard(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CloseShardSvc(inner);
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
                "/shard_service.ShardService/DeleteShard" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteShardSvc<T: ShardServiceGrpc>(pub Arc<T>);
                    impl<
                        T: ShardServiceGrpc,
                    > tonic::server::UnaryService<super::DeleteShardRequest>
                    for DeleteShardSvc<T> {
                        type Response = super::DeleteShardResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteShardRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_shard(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteShardSvc(inner);
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
                "/shard_service.ShardService/ListShards" => {
                    #[allow(non_camel_case_types)]
                    struct ListShardsSvc<T: ShardServiceGrpc>(pub Arc<T>);
                    impl<
                        T: ShardServiceGrpc,
                    > tonic::server::UnaryService<super::ListShardsRequest>
                    for ListShardsSvc<T> {
                        type Response = super::ListShardsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListShardsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_shards(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListShardsSvc(inner);
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
    impl<T: ShardServiceGrpc> Clone for ShardServiceGrpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ShardServiceGrpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ShardServiceGrpc> tonic::server::NamedService for ShardServiceGrpcServer<T> {
        const NAME: &'static str = "shard_service.ShardService";
    }
}
