/// Day 16: https://adventofcode.com/2021/day/16
/// Copyright 2021 by Alex Utter

#[path = "bits.rs"] mod bits;
#[path = "common.rs"] mod common;

// Part 1 adds all version numbers from a packet tree.
fn ver_total(pkt: &bits::Packet) -> u64 {
    pkt.ver as u64 + match &pkt.dat {
        bits::PacketContents::Value(_) =>
            0u64,
        bits::PacketContents::Packets(inner) =>
            inner.iter().map(|p| ver_total(p)).sum(),
    }
}

pub fn solve() {
    let test = common::read_lines("input/test16.txt");
    let data = common::read_lines("input/input16.txt");

    // Parse the five test packets.
    assert_eq!(test.len(), 5);
    let test0 = bits::Packet::from(&test[0]);
    let test1 = bits::Packet::from(&test[1]);
    let test2 = bits::Packet::from(&test[2]);
    let test3 = bits::Packet::from(&test[3]);
    let test4 = bits::Packet::from(&test[4]);
    assert_eq!(ver_total(&test0), 6);
    assert_eq!(ver_total(&test1), 16);
    assert_eq!(ver_total(&test2), 12);
    assert_eq!(ver_total(&test3), 23);
    assert_eq!(ver_total(&test4), 31);

    // Parse the main data packet.
    let data0 = bits::Packet::from(&data[0]);
    println!("Part1: {}", ver_total(&data0));
}
