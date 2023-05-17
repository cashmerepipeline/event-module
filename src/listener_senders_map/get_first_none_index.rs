use crate::listener_senders_map::get_listener_sender_map;

pub fn get_first_none_index(listener_id:&String) -> u32{
    let instance_index_sender_map = get_listener_sender_map(listener_id);
    let instance_index_sender_map = instance_index_sender_map.read();

    let mut index = 0;
    loop{
        if !instance_index_sender_map.contains_key(&index){
            break;
        }

        if instance_index_sender_map.get(&index).unwrap().is_none(){
            break;
        }

        index += 1;
    }

    index
}