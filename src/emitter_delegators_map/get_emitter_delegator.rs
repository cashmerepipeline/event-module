
/// 取得发射者
pub fn get_emitter_delegator(event_type_id: &String, emitter_id: &String) -> Option<Emitter> {
    let emitters_arc = get_event_type_emitters_map(event_type_id);
    let emitters = emitters_arc.read();

    emitters.get(emitter_id).cloned()
}
