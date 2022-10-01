// https://github.com/hyperium/tonic/blob/master/examples/src/hyper_warp/server.rs

use crate::machine::store::Store;
use crate::server::core::{ServerEvent, ServerState};
use futures::Stream;
use paperclip_proto::service::designer::designer_server::Designer;
use paperclip_proto::service::designer::{
    file_response, Empty, FileRequest, FileResponse, PaperclipData, UpdateFileRequest,
};
use parking_lot::Mutex;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status};

type OpenFileResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<FileResponse, Status>> + Send>>;

#[derive(Clone)]
pub struct DesignerService {
    state: Arc<Mutex<Store<ServerState, ServerEvent>>>,
}

impl DesignerService {
    pub fn new(state: Arc<Mutex<Store<ServerState, ServerEvent>>>) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl Designer for DesignerService {
    type OpenFileStream = ResponseStream;
    async fn open_file(
        &self,
        request: Request<FileRequest>,
    ) -> OpenFileResult<Self::OpenFileStream> {
        Err(Status::unimplemented("not implemented"))
    }
    async fn update_file(
        &self,
        request: Request<UpdateFileRequest>,
    ) -> Result<Response<Empty>, Status> {
        Err(Status::unimplemented("not implemented"))
    }
}

// fn evaluate_pc_data(
//     path: &str,
//     state: Arc<Mutex<ServerState>>,
// ) -> file_response::Data {

//     let mut runtime = Runtime::new();

//     // block_on(project.load_file(path)).expect("Coudn't load file");

//     let graph = state.graph.lock().unwrap();
//     let (css, html) = block_on(runtime.evaluate(path, &graph, &project.io)).unwrap();
//     file_response::Data::Paperclip(PaperclipData {
//         css: Some(css),
//         html: Some(html),
//     })
// }
