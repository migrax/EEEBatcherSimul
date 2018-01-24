extern crate eee_hyst;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::io;
use std::io::BufRead;
use structopt::StructOpt;
use eee_hyst::{simulator, Time};
use eee_hyst::switch::Packet;


#[derive(StructOpt, Debug)]
#[structopt(name = "Packet buncher", about = "Creates batches of Ethernet frames.")]
struct Opt {
    #[structopt(short = "c", long = "capacity", help = "Sets the output link capacity in Gb/s",
                default_value = "10")]
    capacity: u32,

    #[structopt(short = "d", long = "delay",
                help = "Delay for the first packet in the burst in ns.", default_value = "0")]
    delay: u32,
}

struct PacketsFromRead<'a, R: 'a + BufRead + ?Sized> {
    is: &'a mut R,
}

impl<'a, R: BufRead + ?Sized> PacketsFromRead<'a, R> {
    pub fn new(buf: &'a mut R) -> PacketsFromRead<'a, R> {
        PacketsFromRead { is: buf }
    }
}

impl<'a, R: BufRead + ?Sized> Iterator for PacketsFromRead<'a, R> {
    type Item = Packet;

    fn next(&mut self) -> Option<Packet> {
        let line = &mut String::new();

        match self.is.read_line(line) {
            Err(_) => None,
            _ => {
                let values: Vec<&str> = line.split_whitespace().collect();

                match values.len() {
                    0 => None, // Just an empty line
                    2 => Some(Packet::new(
                        Time::from_secs(
                            values[0]
                                .parse()
                                .expect(&format!("{} is not a valid arrival time.", values[0])),
                        ),
                        values[1]
                            .parse()
                            .expect(&format!("{} is not a valid size.", values[1])),
                    )),
                    _ => {
                        eprintln!("Malformed line \"{}\"", line);
                        ::std::process::exit(1)
                    }
                }
            }
        }
    }
}


fn main() {
    let opt = Opt::from_args();

    let stdin = io::stdin();
    let input_read = &mut stdin.lock();
    let input_packets = PacketsFromRead::new(input_read);

    let simul = simulator::Simulator::new_explicit(
        Time(0),
        Time(opt.delay as u64),
        input_packets,
        Time(0),
        Time(0),
        1e9 * opt.capacity as f64,
    );

    for event in simul {
        if let (time, Some(packet), _) = event {
            println!("{:e}\t{}", time.as_secs(), packet.size());
        }
    }
}
