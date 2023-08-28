use petri_net::IPetriNet;

mod petri_net;

fn main() {
    let mut petri_net = petri_net::create_petri_net::<Element, Binder>();
    let node = petri_net.add_node(Element{name: "test".to_string()});
}

struct Element{
    name: String,
}
struct Binder{

}