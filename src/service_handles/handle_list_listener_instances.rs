use dependencies_sync::tonic::async_trait;

use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::{Request, Response, Status};

use crate::listener_instances_map::get_listener_instance_map;
use crate::protocols::*;

#[async_trait]
pub trait HandleListListenerInstances {
    async fn handle_list_listener_instances(
        &self,
        request: Request<ListListenerInstancesRequest>,
    ) -> Result<Response<ListListenerInstancesResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_list_listener_instances)
            .await
    }
}

async fn validate_view_rules(
    request: Request<ListListenerInstancesRequest>,
) -> Result<Request<ListListenerInstancesRequest>, Status> {
    #[cfg(feature = "validate_view_rules")]
    {
        use crate::ids_codes::manage_ids::EVENT_LISTENERS_MANAGE_ID;
        let manage_id = EVENT_LISTENERS_MANAGE_ID;
        let (account_id, _groups, role_group) = request_account_context(request.metadata())?;
        if let Err(e) = view::validates::validate_collection_can_read(&manage_id, &role_group).await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<ListListenerInstancesRequest>,
) -> Result<Request<ListListenerInstancesRequest>, Status> {
    Ok(request)
}

async fn handle_list_listener_instances(
    request: Request<ListListenerInstancesRequest>,
) -> Result<Response<ListListenerInstancesResponse>, Status> {
    let listener_id = &request.get_ref().listener_id;

    let listener_sender_map_arc = get_listener_instance_map(listener_id);
    let listener_sender_map = listener_sender_map_arc.read();
    let listener_instances = listener_sender_map
        .iter()
        .filter(|(_, instance_inner)| instance_inner.is_some())
        .map(|(_instance_id, instance_inner)| {
            let instance_inner = instance_inner.as_ref().unwrap();
            let name = instance_inner.name.clone();
            let index = instance_inner.index.clone();

            ListenerInstance { name, index }
        })
        .collect();

    Ok(Response::new(ListListenerInstancesResponse {
        instances: listener_instances,
    }))
}
