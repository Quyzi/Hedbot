extern crate irc;
extern crate rand;

use irc::client::prelude::*;

use rand::{Rng, thread_rng};

use std::io::prelude::*;
use std::str::FromStr;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::{thread, time};
use std::thread::spawn;

fn lines_from_file<P>(filename: P) -> Vec<String>
    where P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}

fn main() {
	let server = IrcServer::new("config.json").unwrap();
	server.identify().unwrap();

	let mintime: u64 = u64::from_str(server.config().get_option("minTime")).unwrap() * 60; 
	let maxtime: u64 = u64::from_str(server.config().get_option("maxTime")).unwrap() * 60; 

	let lines: Vec<String> = lines_from_file(&server.config().get_option("quoteFile"));
	let mut rng = thread_rng();

	if server.config().get_option("spawnThread") == "yes" {
		let server2 = server.clone();
		spawn(move || {
			server2.iter().map(|m| print!("{}", m.unwrap())).count();
		});
	}

	loop {
		let line = rng.gen_range(0, lines.len());
		for channel in server.config().channels() {
			println!("Sending #{} to \"{}\"", line, channel);
			server.send_privmsg(channel, &lines[line]).expect("Something wrong");
		}
		thread::sleep(time::Duration::from_secs(rng.gen_range(mintime, maxtime)));
	}
}