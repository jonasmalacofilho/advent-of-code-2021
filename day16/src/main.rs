use bitvec::prelude::*;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Hello, world!");

    let packet = parse_packet(INPUT);

    dbg!(sum_versions(&packet));
    dbg!(eval(&packet));
}

#[derive(Debug)]
enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug)]
enum PacketData {
    Literal(u64),
    Operator { op: Operator, args: Vec<Packet> },
}

#[derive(Debug)]
struct Packet {
    version: u8,
    data: PacketData,
}

impl Packet {
    fn new_literal(version: u8, value: u64) -> Packet {
        Packet {
            version,
            data: PacketData::Literal(value)
        }
    }

    fn new_operator(version: u8, op: Operator, args: Vec<Packet>) -> Packet {
        Packet {
            version,
            data: PacketData::Operator { op, args }
        }
    }
}

fn parse_bits(s: &str, radix: usize) -> BitVec {
    let s = s.trim_end();

    match radix {
        2 => s
            .bytes()
            .map(|b| {
                assert!((b'0'..=b'9').contains(&b));
                b - b'0' != 0
            })
            .collect(),
        16 => {
            let mut bits = bitvec![];

            for digit in s.bytes() {
                let digit = match digit {
                    b'0' => [0, 0, 0, 0],
                    b'1' => [0, 0, 0, 1],
                    b'2' => [0, 0, 1, 0],
                    b'3' => [0, 0, 1, 1],
                    b'4' => [0, 1, 0, 0],
                    b'5' => [0, 1, 0, 1],
                    b'6' => [0, 1, 1, 0],
                    b'7' => [0, 1, 1, 1],
                    b'8' => [1, 0, 0, 0],
                    b'9' => [1, 0, 0, 1],
                    b'A' => [1, 0, 1, 0],
                    b'B' => [1, 0, 1, 1],
                    b'C' => [1, 1, 0, 0],
                    b'D' => [1, 1, 0, 1],
                    b'E' => [1, 1, 1, 0],
                    b'F' => [1, 1, 1, 1],
                    _ => panic!("invalid char: {}", char::from(digit)),
                };

                for b in digit {
                    bits.push(b != 0);
                }
            }

            bits
        }
        _ => panic!("unsupported radix: {}", radix),
    }
}

fn parse_packet(s: &str) -> Packet {
    let bits = parse_bits(s, 16);

    let (root, bits) = packet(bits.as_bitslice());
    dbg!(&root);
    assert!(bits.iter().all(|b| !b));
    dbg!(bits.len());

    root
}

fn packet(bits: &BitSlice) -> (Packet, &BitSlice) {
    let (version, bits) = integer(bits, 3);
    let (type_id, bits) = integer(bits, 3);

    let (data, bits) = match type_id {
        4 => {
            let (literal, bits) = variable_integer(bits);
            (PacketData::Literal(literal), bits)
        }
        _ => {
            let op = operator(type_id as u8);
            let (with_number, bits) = (bits[0], &bits[1..]);

            if with_number {
                let (n, bits) = integer(bits, 11);
                let (args, bits) = exact_children(bits, n as usize);
                (PacketData::Operator { op, args }, bits)
            } else {
                let (len, bits) = integer(bits, 15);
                let (args, _) = children(&bits[0..(len as usize)]);
                (PacketData::Operator { op, args }, &bits[(len as usize)..])
            }
        }
    };

    let packet = Packet {
        version: version as u8,
        data,
    };

    (packet, bits)
}

fn operator(type_id: u8) -> Operator {
    match type_id {
        0 => Operator::Sum,
        1 => Operator::Product,
        2 => Operator::Minimum,
        3 => Operator::Maximum,
        5 => Operator::GreaterThan,
        6 => Operator::LessThan,
        7 => Operator::EqualTo,
        _ => panic!("invalid type_id ({}) for operator", type_id),
    }
}

fn children(mut bits: &BitSlice) -> (Vec<Packet>, &BitSlice) {
    let mut children = vec![];

    while !bits.is_empty() {
        let (packet, next) = packet(bits);
        children.push(packet);
        bits = next;
    }

    (children, bits)
}

fn exact_children(mut bits: &BitSlice, n: usize) -> (Vec<Packet>, &BitSlice) {
    let mut children = vec![];

    for _ in 0..n {
        let (packet, next) = packet(bits);
        children.push(packet);
        bits = next;
    }

    (children, bits)
}

fn integer(bits: &BitSlice, size: usize) -> (u16, &BitSlice) {
    assert!(size <= 16);
    let int = bits[0..size]
        .iter()
        .fold(0, |acc, b| (acc << 1) | *b as u16);
    (int, &bits[size..])
}

fn variable_integer(mut bits: &BitSlice) -> (u64, &BitSlice) {
    let mut int = 0u64;

    while bits[0] {
        let (group, next) = integer(&bits[1..], 4);
        int = (int << 4) | group as u64;
        bits = next;
    }

    let (group, bits) = integer(&bits[1..], 4);
    int = (int << 4) | group as u64;

    (int, bits)
}

fn fold_packet<B, F>(root: &Packet, init: B, mut f: F) -> B
where
    F: FnMut(B, &Packet) -> B + Copy, // FIXME
{
    let mut acc = f(init, root);

    if let PacketData::Operator { args, .. } = &root.data {
        for child in args {
            acc = fold_packet(child, acc, f);
        }
    }

    acc
}

fn sum_versions(root: &Packet) -> usize {
    fold_packet(root, 0, |acc, p| acc + p.version as usize)
}

fn eval(root: &Packet) -> u64 {
    match &root.data {
        PacketData::Literal(v) => *v as _,
        PacketData::Operator { op, args } => {
            let mut args = args.iter().map(eval);

            match op {
                Operator::Sum => args.sum(),
                Operator::Product => args.product(),
                Operator::Minimum => args.min().unwrap(),
                Operator::Maximum => args.max().unwrap(),
                Operator::GreaterThan => (args.next().unwrap() > args.next().unwrap()) as _,
                Operator::LessThan => (args.next().unwrap() < args.next().unwrap()) as _,
                Operator::EqualTo => (args.next().unwrap() == args.next().unwrap()) as _,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bits() {
        assert_eq!(parse_bits("1101", 2), bitvec![1, 1, 0, 1]);
    }

    #[test]
    fn parses_bits_from_hex() {
        assert_eq!(
            parse_bits("D2FE28", 16),
            parse_bits("110100101111111000101000", 2)
        );
    }

    #[test]
    fn parses_integers() {
        assert_eq!(
            integer(bitvec![1, 1, 0].as_bitslice(), 3),
            (6, bitvec![].as_bitslice())
        );
    }

    #[test]
    fn parses_variable_integers() {
        assert_eq!(
            variable_integer(parse_bits("101111111000101", 2).as_bitslice()),
            (2021, bitvec![].as_bitslice())
        );
    }

    #[test]
    fn sample_version_sums_match() {
        assert_eq!(sum_versions(&parse_packet("8A004A801A8002F478")), 16);
        assert_eq!(sum_versions(&parse_packet("620080001611562C8802118E34")), 12);
        assert_eq!(sum_versions(&parse_packet("C0015000016115A2E0802F182340")), 23);
        assert_eq!(sum_versions(&parse_packet("A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    fn sample_eval_results_match() {
        assert_eq!(eval(&parse_packet("C200B40A82")), 3);
        assert_eq!(eval(&parse_packet("04005AC33890")), 54);
        assert_eq!(eval(&parse_packet("880086C3E88112")), 7);
        assert_eq!(eval(&parse_packet("CE00C43D881120")), 9);
        assert_eq!(eval(&parse_packet("D8005AC2A8F0")), 1);
        assert_eq!(eval(&parse_packet("F600BC2D8F")), 0);
        assert_eq!(eval(&parse_packet("9C005AC2F8F0")),0);
        assert_eq!(eval(&parse_packet("9C0141080250320F1802104A08")), 1);
    }

    #[test]
    fn sums_a_single_packet() {
        let val = Packet::new_literal(0, 42);
        let sum = Packet::new_operator(0, Operator::Sum, vec![val]);
        assert_eq!(eval(&sum), 42);
    }

    #[test]
    fn multiplies_a_single_packet() {
        let val = Packet::new_literal(0, 42);
        let prod = Packet::new_operator(0, Operator::Product, vec![val]);
        assert_eq!(eval(&prod), 42);
    }

    #[test]
    fn does_not_regress() {
        let packet = parse_packet(INPUT);
        assert_eq!(sum_versions(&packet), 947);
        assert_eq!(eval(&packet), 660797830937);
    }
}
