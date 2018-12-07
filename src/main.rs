/*
 * Copyright (C) 2018 Miguel Rodríguez Pérez <miguel@det.uvigo.gal>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use structopt;

use eee_hyst::switch::Packet;
use eee_hyst::{simulator, Time};
use std::io;
use std::io::BufRead;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Packet buncher", about = "Creates batches of Ethernet frames.")]
struct Opt {
    #[structopt(
        short = "c",
        long = "capacity",
        help = "Sets the output link capacity in Gb/s",
        default_value = "10"
    )]
    capacity: u32,

    #[structopt(
        short = "d",
        long = "delay",
        help = "Delay for the first packet in the burst in ns.",
        default_value = "0"
    )]
    delay: u32,
}

struct PacketsFromRead<'a, R: BufRead + ?Sized> {
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
                        Time::from_secs(values[0].parse().unwrap_or_else(|_| {
                            panic!("{} is not a valid arrival time.", values[0])
                        })),
                        values[1]
                            .parse()
                            .unwrap_or_else(|_| panic!("{} is not a valid size.", values[1])),
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
        Time(u64::from(opt.delay)),
        input_packets,
        Time(0),
        Time(0),
        1e9 * f64::from(opt.capacity),
    );

    for event in simul {
        if let (time, Some(packet), _) = event {
            println!("{:e}\t{}", time.as_secs(), packet.size());
        }
    }
}
