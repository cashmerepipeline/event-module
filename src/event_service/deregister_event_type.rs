pub async fn deregister_event_type(type_id: &String){
    crate::event_types_map::deregister_event_type(type_id);
}
