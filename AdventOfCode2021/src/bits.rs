/// Buoyancy Interchange Transmission System (BITS)
/// Copyright 2021 by Alex Utter
/// See also: https://adventofcode.com/2021/day/16

pub struct BitStream {
    dat: Vec<bool>,
}

impl BitStream {
    // Create an empty BitStream.
    pub fn new() -> BitStream {
        BitStream { dat: Vec::new() }
    }

    // Length of remaining bits.
    pub fn len(&self) -> usize {
        self.dat.len()
    }

    // Consume the next N bits, if available.
    pub fn consume(&mut self, n: usize) -> Option<BitStream> {
        if self.len() >= n {
            Some( BitStream {dat: self.dat.drain(0..n).collect()} )
        } else {None}
    }

    // Read the next N bits as an integer, if available.
    pub fn read(&mut self, n: usize) -> Option<u64> {
        if let Some(tmp) = self.consume(n) {
            Some(tmp.value())
        } else {None}
    }

    // Evaluate the entire BitStream as an integer.
    pub fn value(&self) -> u64 {
        let mut x = 0u64;
        for b in self.dat.iter() {
            x = 2*x + if *b {1} else {0};
        }
        return x
    }

    // Add a bitstream onto the end of this one.
    pub fn append(&mut self, mut x: BitStream) {
        self.dat.append(&mut x.dat);
    }
}

impl From<&str> for BitStream {
    fn from(x: &str) -> Self {
        let mut dat = Vec::new();
        for ch in x.chars() {
            // Each valid hexadecimal character is four bits.
            if let Some(n) = ch.to_digit(16) {
                dat.push((n & 8) > 0);  // MSB-first
                dat.push((n & 4) > 0);
                dat.push((n & 2) > 0);
                dat.push((n & 1) > 0);
            }
        }
        BitStream { dat:dat }
    }
}

impl From<&String> for BitStream {
    fn from(x: &String) -> Self {
        BitStream::from(x as &str)
    }
}

pub enum PacketContents {
    Value(BitStream),
    Packets(Vec<Packet>),
}

pub struct Packet {
    pub ver: u8,
    pub typ: u8,
    pub dat: PacketContents,
}

impl Packet {
    // Read a single packet from a BitStream.
    pub fn read(src: &mut BitStream) -> Self {
        let ver = src.read(3).unwrap() as u8;
        let typ = src.read(3).unwrap() as u8;
        let dat = Packet::read_contents(src, typ);
        Packet { ver:ver, typ:typ, dat:dat }
    }

    // Find total version of this packet and all children.
    // (i.e., Day 16 Part 1 solution.)
    pub fn ver_total(&self) -> u64 {
        self.ver as u64 + match &self.dat {
            PacketContents::Value(_) =>
                0u64,
            PacketContents::Packets(inner) =>
                inner.iter().map(|p| p.ver_total()).sum(),
        }
    }

    // Evaluate the expression contained in this packet.
    // (i.e., Day 16 Part 2 solution.)
    pub fn evaluate(&self) -> u64 {
        match (self.typ, &self.dat) {
            (_, PacketContents::Value(literal)) =>
                literal.value(),
            (0, PacketContents::Packets(inner)) =>
                inner.iter().map(|p| p.evaluate()).sum(),
            (1, PacketContents::Packets(inner)) =>
                inner.iter().map(|p| p.evaluate()).product(),
            (2, PacketContents::Packets(inner)) =>
                inner.iter().map(|p| p.evaluate()).min().unwrap(),
            (3, PacketContents::Packets(inner)) =>
                inner.iter().map(|p| p.evaluate()).max().unwrap(),
            (5, PacketContents::Packets(inner)) =>
                if inner[0].evaluate() > inner[1].evaluate() {1} else {0}
            (6, PacketContents::Packets(inner)) =>
                if inner[0].evaluate() < inner[1].evaluate() {1} else {0}
            (7, PacketContents::Packets(inner)) =>
                if inner[0].evaluate() == inner[1].evaluate() {1} else {0}
            _ => 0u64,
        }
    }

    fn read_contents(src: &mut BitStream, typ: u8) -> PacketContents {
        if typ == 4 {
            PacketContents::Value(Packet::read_literal(src))
        } else {
            let ltype = src.read(1).unwrap();
            if ltype == 0 {
                PacketContents::Packets(Packet::read_sublen(src))
            } else {
                PacketContents::Packets(Packet::read_subpkt(src))
            }
        }
    }

    fn read_literal(src: &mut BitStream) -> BitStream {
        let mut dst = BitStream::new();
        while let Some(mut x) = src.consume(5) {
            let cont = x.read(1).unwrap();
            dst.append(x);
            if cont == 0 {break;}
        }
        return dst
    }

    fn read_sublen(src: &mut BitStream) -> Vec<Packet> {
        let len = src.read(15).unwrap() as usize;
        let mut sub = src.consume(len).unwrap();
        let mut pkts = Vec::new();
        while sub.len() > 0 {
            pkts.push(Packet::read(&mut sub));
        }
        return pkts
    }

    fn read_subpkt(src: &mut BitStream) -> Vec<Packet> {
        let len = src.read(11).unwrap() as usize;
        (0..len).map(|_| Packet::read(src)).collect()
    }
}

impl From<&str> for Packet {
    fn from(src: &str) -> Self {
        let mut strm = BitStream::from(src);
        Packet::read(&mut strm)
    }
}

impl From<&String> for Packet {
    fn from(src: &String) -> Self {
        let mut strm = BitStream::from(src);
        Packet::read(&mut strm)
    }
}
