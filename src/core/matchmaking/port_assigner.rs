use port_scanner::local_port_available;
use rand::Rng;

pub fn get_free_random_port_for_gameserver() -> u16 {
    let random_port: u16 = rand::thread_rng().gen_range(1026u16..65000u16);

    if local_port_available(random_port) {
        random_port
    } else {
        get_free_random_port_for_gameserver()
    }
}
