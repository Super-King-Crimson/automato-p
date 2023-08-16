use automato_p::app; 

fn main() {
    app::startup();
    
    loop {
        app::run();
    }
}