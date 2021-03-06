use grpc::{ServerHandlerContext, ServerRequestSingle, ServerResponseUnarySink};

use mizer_api::handlers::MediaHandler;
use mizer_api::models::*;

use crate::protos::MediaApi;

impl MediaApi for MediaHandler {
    fn create_tag(
        &self,
        o: ServerHandlerContext,
        req: ServerRequestSingle<CreateMediaTag>,
        resp: ServerResponseUnarySink<MediaTag>,
    ) -> grpc::Result<()> {
        let handler = self.clone();
        o.spawn(async move {
            let tag = handler.create_tag(req.message.name).await.unwrap();

            resp.finish(tag)
        });

        Ok(())
    }

    fn get_tags_with_media(
        &self,
        o: ServerHandlerContext,
        _: ServerRequestSingle<GetMediaTags>,
        resp: ServerResponseUnarySink<GroupedMediaFiles>,
    ) -> grpc::Result<()> {
        let handler = self.clone();
        o.spawn(async move {
            let files = handler.get_tags_with_media().await.unwrap();

            resp.finish(files)
        });

        Ok(())
    }

    fn get_media(
        &self,
        o: ServerHandlerContext,
        _: ServerRequestSingle<GetMediaRequest>,
        resp: ServerResponseUnarySink<MediaFiles>,
    ) -> grpc::Result<()> {
        let handler = self.clone();
        o.spawn(async move {
            let files = handler.get_media().await.unwrap();

            resp.finish(files)
        });

        Ok(())
    }
}
