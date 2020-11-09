use std::process::{ChildStdin, ChildStdout, Command, Stdio};

pub struct Client {
    pub input: ChildStdin,
    pub output: ChildStdout,
    pub path: String,
    pub wins_when_team1: u32,
    pub draws_when_team1: u32,
    pub losses_when_team1: u32,
    pub wins_when_team2: u32,
    pub draws_when_team2: u32,
    pub losses_when_team2: u32,
}

impl Client {
    pub fn from_path(path: String, time: u64) -> Client {
        let mut process = Command::new(path.clone())
            .args(&["--time", &time.to_string()])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap_or_else(|_| panic!("Can't start the client: {}", path));
        let input = process.stdin.take().unwrap();
        let output = process.stdout.take().unwrap();

        Client {
            input,
            output,
            path,
            wins_when_team1: 0,
            draws_when_team1: 0,
            losses_when_team1: 0,
            wins_when_team2: 0,
            draws_when_team2: 0,
            losses_when_team2: 0,
        }
    }
}

pub fn print_stats(client1: &Client, client2: &Client) {
    let mut line = String::new();
    let wins = client1.wins_when_team1 + client1.wins_when_team2;
    let draws = client1.draws_when_team1 + client1.draws_when_team2;
    let losses = client1.losses_when_team1 + client1.losses_when_team2;
    let games_played = wins + draws + losses;
    line.push_str(&format!("{:6} ", games_played));
    line.push_str(&format!("{:27}", client1.path));
    line.push_str(&format!("{:6}", wins));
    line.push_str(&format!("{:6}", draws));
    line.push_str(&format!("{:6} ", losses));
    line.push_str(&format!("{:27}", client2.path));
    println!("{}", line);
}
