use async_trait::async_trait;
use tonic::{Code, Request, Response, Status};

use monie_rpc::monie::timeline::timeline_api_server::TimelineApi;
use monie_rpc::monie::timeline::{
    GetPlotRequest, PlotResponse, TimelinePageRequest, TimelineResponse,
};

#[derive(Debug)]
struct Timeline {}

#[async_trait]
impl TimelineApi for Timeline {
    async fn get_timeline(
        &self,
        request: Request<TimelinePageRequest>,
    ) -> Result<Response<TimelineResponse>, Status> {
        
        return Ok(Response::new(TimelineResponse { plots: vec![] }));
    }

    async fn get_plot_by_id(
        &self,
        request: Request<GetPlotRequest>,
    ) -> Result<Response<PlotResponse>, Status> {
        let request = request.into_inner();
        Err(Status::not_found(format!(
            "Plot with id {} not found",
            request.id
        )))
    }
}
