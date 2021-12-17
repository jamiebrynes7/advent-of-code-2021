use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input = get_input()?;
    println!("Part 1 result: {}", part1(&input));
    println!("Part 2 result: {}", part2(&input));
    Ok(())
}

fn part1(input: &Packet) -> u64 {
    input.sum_version()
}

fn part2(input: &Packet) -> u64 {
    input.eval()
}

fn get_input() -> Result<Packet> {
    const PATH: &str = "src/day16/input.txt";

    let mut file = File::open(PATH)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let bytes = hex::decode(data)?;
    let mut reader = BitReader::new(&bytes);

    Ok(Packet::from_bytes(&mut reader))
}

#[derive(Debug)]
enum Packet {
    Literal(LiteralPacket),
    Operator(Operator),
}

impl Packet {
    pub fn from_bytes(reader: &mut BitReader) -> Packet {
        let version = reader.read::<3>();
        let type_id = reader.read::<3>();

        match type_id {
            4 => Packet::Literal(LiteralPacket::parse(version, reader)),
            _ => Packet::Operator(Operator::parse(version, type_id, reader)),
        }
    }

    pub fn sum_version(&self) -> u64 {
        match self {
            Packet::Literal(lit) => lit.version,
            Packet::Operator(operator) => {
                operator.version
                    + operator
                        .sub_packets
                        .iter()
                        .map(|p| p.sum_version())
                        .sum::<u64>()
            }
        }
    }

    pub fn eval(&self) -> u64 {
        match self {
            Packet::Literal(lit) => lit.value,
            Packet::Operator(operator) => operator.eval(),
        }
    }
}

#[derive(Debug)]
struct LiteralPacket {
    pub version: u64,
    pub value: u64,
}

impl LiteralPacket {
    pub fn parse(version: u64, reader: &mut BitReader) -> Self {
        let mut value = 0;

        let mut more_bytes = true;

        while more_bytes {
            more_bytes = reader.read::<1>() == 1;
            value = (value << 4) | reader.read::<4>();
        }

        LiteralPacket { version, value }
    }
}

#[derive(Debug)]
struct Operator {
    pub version: u64,
    pub type_id: u64,
    pub sub_packets: Vec<Packet>,
}

impl Operator {
    pub fn parse(version: u64, type_id: u64, reader: &mut BitReader) -> Self {
        let length_type_id = reader.read::<1>();

        let sub_packets = match length_type_id {
            0 => {
                let size_bits = reader.read::<15>() as usize;

                let before = reader.bytes_read();
                let mut packets = vec![];
                while before + size_bits != reader.bytes_read() {
                    packets.push(Packet::from_bytes(reader));
                }

                packets
            }
            1 => {
                let num_packets = reader.read::<11>();

                (0..num_packets)
                    .map(|_| Packet::from_bytes(reader))
                    .collect()
            }
            _ => panic!("Unknown length_type_id"),
        };

        Operator {
            version,
            type_id,
            sub_packets,
        }
    }

    pub fn eval(&self) -> u64 {
        let mut sub_packet_values = self.sub_packets.iter().map(|p| p.eval());
        match self.type_id {
            0 => sub_packet_values.sum(),
            1 => sub_packet_values.product(),
            2 => sub_packet_values.min().unwrap(),
            3 => sub_packet_values.max().unwrap(),
            5 => {
                if sub_packet_values.next().unwrap() > sub_packet_values.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            6 => {
                if sub_packet_values.next().unwrap() < sub_packet_values.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            7 => {
                if sub_packet_values.next().unwrap() == sub_packet_values.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Unknown type id {}", self.type_id),
        }
    }
}

#[derive(Debug)]
struct BitReader<'a> {
    data: &'a Vec<u8>,
    position: usize,
    offset: u8,
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a Vec<u8>) -> Self {
        BitReader {
            data,
            position: 0,
            offset: 0,
        }
    }

    pub fn bytes_read(&self) -> usize {
        self.position * 8 + self.offset as usize
    }

    pub fn read<const N: usize>(&mut self) -> u64 {
        assert!(N <= 64);
        let mut value: u64 = 0;

        for _ in 0..N {
            value = value << 1;
            value |= self.read_bit() as u64;
        }

        value
    }

    fn read_bit(&mut self) -> u8 {
        let value = (self.data[self.position] >> (7 - self.offset)) & (0b0000001);

        self.offset = (self.offset + 1) % 8;
        if self.offset == 0 {
            self.position += 1;
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::BitReader;

    #[test]
    pub fn test_bit_reader_read() {
        let bin = hex::decode("D2FE28").unwrap();
        let mut reader = BitReader::new(&bin);
        assert_eq!(reader.read::<24>(), 0b110100101111111000101000);
    }

    #[test]
    pub fn test_bit_reader_read_other() {
        let bin = hex::decode("38006F45291200").unwrap();
        let mut reader = BitReader::new(&bin);
        assert_eq!(
            reader.read::<56>(),
            0b00111000000000000110111101000101001010010001001000000000
        );
    }
}
