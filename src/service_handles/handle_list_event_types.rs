use std::ops::Deref;

use dependencies_sync::tonic::async_trait;

use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};

use request_utils::request_account_context;

use crate::event_types_map::get_event_types;
use crate::ids_codes::manage_ids::EVENT_TYPES_MANAGE_ID;
use crate::protocols::*;

#[async_trait]
pub trait HandleListEventTypes {
    async fn handle_list_event_types(
        &self,
        request: Request<ListEventTypesRequest>,
    ) -> Result<Response<ListEventTypesResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_list_event_types)
            .await
    }
}

async fn validate_view_rules(
    request: Request<ListEventTypesRequest>,
) -> Result<Request<ListEventTypesRequest>, Status> {
    #[cfg(feature = "validate_view_rules")]
    {
        let manage_id = EVENT_TYPES_MANAGE_ID;
        let (account_id, _groups, role_group) = request_account_context(request.metadata())?;
        if let Err(e) = view::validates::validate_collection_can_read(&manage_id, &role_group).await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<ListEventTypesRequest>,
) -> Result<Request<ListEventTypesRequest>, Status> {
    Ok(request)
}

async fn handle_list_event_types(
    request: Request<ListEventTypesRequest>,
) -> Result<Response<ListEventTypesResponse>, Status> {
    let (_account_id, _groups, _role_group) = request_account_context(request.metadata())?;

    let majordomo_arc = get_majordomo();
    let _manager = majordomo_arc
        .get_manager_by_id(EVENT_TYPES_MANAGE_ID)
        .unwrap();

    let event_type_arcs = get_event_types();
    let event_types = event_type_arcs
        .iter()
        .map(|event_type_arc| event_type_arc.deref().clone())
        .collect();

    Ok(Response::new(ListEventTypesResponse { event_types }))
}
