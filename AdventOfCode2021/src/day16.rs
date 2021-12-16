/// Day 16: https://adventofcode.com/2021/day/16
/// Copyright 2021 by Alex Utter

#[path = "bits.rs"] mod bits;
#[path = "common.rs"] mod common;

pub fn solve() {
    // Total version for various example packets.
    assert_eq!(bits::Packet::from("D2FE28").ver_total(), 6);
    assert_eq!(bits::Packet::from("8A004A801A8002F478").ver_total(), 16);
    assert_eq!(bits::Packet::from("620080001611562C8802118E34").ver_total(), 12);
    assert_eq!(bits::Packet::from("C0015000016115A2E0802F182340").ver_total(), 23);
    assert_eq!(bits::Packet::from("A0016C880162017C3686B18A3D4780").ver_total(), 31);

    // Expression value for various example packets.
    assert_eq!(bits::Packet::from("D2FE28").evaluate(), 2021);
    assert_eq!(bits::Packet::from("C200B40A82").evaluate(), 3);
    assert_eq!(bits::Packet::from("04005AC33890").evaluate(), 54);
    assert_eq!(bits::Packet::from("880086C3E88112").evaluate(), 7);
    assert_eq!(bits::Packet::from("CE00C43D881120").evaluate(), 9);
    assert_eq!(bits::Packet::from("D8005AC2A8F0").evaluate(), 1);
    assert_eq!(bits::Packet::from("F600BC2D8F").evaluate(), 0);
    assert_eq!(bits::Packet::from("9C005AC2F8F0").evaluate(), 0);
    assert_eq!(bits::Packet::from("9C0141080250320F1802104A08").evaluate(), 1);

    // Parse the main data packet.
    let data = common::read_lines("input/input16.txt");
    let data0 = bits::Packet::from(&data[0]);
    println!("Part1: {}", data0.ver_total());
    println!("Part2: {}", data0.evaluate());
}
