#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// A verbose description of the daemon's commit.
    #[prost(string, tag="1")]
    pub commit: ::prost::alloc::string::String,
    /// The SHA1 commit hash that the daemon is compiled with.
    #[prost(string, tag="2")]
    pub commit_hash: ::prost::alloc::string::String,
    /// The semantic version.
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
    /// The major application version.
    #[prost(uint32, tag="4")]
    pub app_major: u32,
    /// The minor application version.
    #[prost(uint32, tag="5")]
    pub app_minor: u32,
    /// The application patch number.
    #[prost(uint32, tag="6")]
    pub app_patch: u32,
    /// The application pre-release modifier, possibly empty.
    #[prost(string, tag="7")]
    pub app_pre_release: ::prost::alloc::string::String,
    /// The list of build tags that were supplied during compilation.
    #[prost(string, repeated, tag="8")]
    pub build_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The version of go that compiled the executable.
    #[prost(string, tag="9")]
    pub go_version: ::prost::alloc::string::String,
}
# [doc = r" Generated client implementations."] pub mod versioner_client { # ! [allow (unused_variables , dead_code , missing_docs , clippy :: let_unit_value ,)] use tonic :: codegen :: * ; # [doc = " Versioner is a service that can be used to get information about the version"] # [doc = " and build information of the running daemon."] # [derive (Debug , Clone)] pub struct VersionerClient < T > { inner : tonic :: client :: Grpc < T > , } impl VersionerClient < tonic :: transport :: Channel > { # [doc = r" Attempt to create a new client by connecting to a given endpoint."] pub async fn connect < D > (dst : D) -> Result < Self , tonic :: transport :: Error > where D : std :: convert :: TryInto < tonic :: transport :: Endpoint > , D :: Error : Into < StdError > , { let conn = tonic :: transport :: Endpoint :: new (dst) ? . connect () . await ? ; Ok (Self :: new (conn)) } } impl < T > VersionerClient < T > where T : tonic :: client :: GrpcService < tonic :: body :: BoxBody > , T :: ResponseBody : Body + Send + Sync + 'static , T :: Error : Into < StdError > , < T :: ResponseBody as Body > :: Error : Into < StdError > + Send , { pub fn new (inner : T) -> Self { let inner = tonic :: client :: Grpc :: new (inner) ; Self { inner } } pub fn with_interceptor < F > (inner : T , interceptor : F) -> VersionerClient < InterceptedService < T , F >> where F : tonic :: service :: Interceptor , T : tonic :: codegen :: Service < http :: Request < tonic :: body :: BoxBody > , Response = http :: Response << T as tonic :: client :: GrpcService < tonic :: body :: BoxBody >> :: ResponseBody > > , < T as tonic :: codegen :: Service < http :: Request < tonic :: body :: BoxBody >> > :: Error : Into < StdError > + Send + Sync , { VersionerClient :: new (InterceptedService :: new (inner , interceptor)) } # [doc = r" Compress requests with `gzip`."] # [doc = r""] # [doc = r" This requires the server to support it otherwise it might respond with an"] # [doc = r" error."] pub fn send_gzip (mut self) -> Self { self . inner = self . inner . send_gzip () ; self } # [doc = r" Enable decompressing responses with `gzip`."] pub fn accept_gzip (mut self) -> Self { self . inner = self . inner . accept_gzip () ; self } # [doc = " lncli: `version`"] # [doc = "GetVersion returns the current version and build information of the running"] # [doc = "daemon."] pub async fn get_version (& mut self , request : impl tonic :: IntoRequest < super :: VersionRequest > ,) -> Result < tonic :: Response < super :: Version > , tonic :: Status > { self . inner . ready () . await . map_err (| e | { tonic :: Status :: new (tonic :: Code :: Unknown , format ! ("Service was not ready: {}" , e . into ())) }) ? ; let codec = tonic :: codec :: ProstCodec :: default () ; let path = http :: uri :: PathAndQuery :: from_static ("/verrpc.Versioner/GetVersion") ; self . inner . unary (request . into_request () , path , codec) . await } } }