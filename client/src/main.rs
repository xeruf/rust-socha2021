use argparse::{ArgumentParser, Store};
mod test_client;
mod xml_client;
mod xml_node;
//use player::simple_client::SimpleClient as Player;
use player::mcts::Mcts as Player;
//use player::neural_network::NeuralNetwork as Player;
//use player::search::Searcher as Player;
use test_client::run_test_client;
use xml_client::XmlClient;

fn main() {
    let mut host = "localhost".to_string();
    let mut port = "13050".to_string();
    let mut reservation = "".to_string();
    let mut time: i64 = 1960;
    let mut test = false;

    {
        let mut parser = ArgumentParser::new();
        parser
            .refer(&mut host)
            .add_option(&["-h", "--host"], Store, "Host");
        parser
            .refer(&mut port)
            .add_option(&["-p", "--port"], Store, "Port");
        parser
            .refer(&mut reservation)
            .add_option(&["-r", "--reservation"], Store, "Reservation");
        parser
            .refer(&mut time)
            .add_option(&["-t", "--time"], Store, "Time/Action in ms");
        parser.refer(&mut test).add_option(
            &["-c", "--test"],
            Store,
            "Run the test client instead of the xml client.",
        );
        parser.parse_args_or_exit();
    }

    println!(
        "Server: {}:{}\nReservation: \"{}\"\nTime/Action: {}ms\nTest: {}",
        host, port, reservation, time, test
    );

    //let player = Box::new(Player::new("weights").unwrap());

    let mut player = Box::new(Player::default());
    player.set_time_limit(Some(time));
    //player.set_neural_network(None);

    if test {
        run_test_client(player);
    } else {
        let mut client = XmlClient::new(host, port, reservation, player);
        client.run();
    }
}
