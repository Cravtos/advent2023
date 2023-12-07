use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Copy, Clone)]
enum CardType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPairs = 3,
    OnePair = 2,
    HighCard = 1,
}

fn card_rank(card: char) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        n if n.is_ascii_digit() => n.to_digit(10).unwrap() as u8,
        _ => panic!("unknown card {card}"),
    }
}

fn get_type(hand: &str) -> u8 {
    let mut jokers = 0;

    let mut cards: HashMap<char, u8> = HashMap::new();
    for card in hand.chars() {
        if card == 'J' {
            jokers += 1;
            continue;
        }
        *cards.entry(card).or_insert(0) += 1
    }

    let counts = cards.values();
    let max = *counts.clone().max().unwrap_or(&0);
    let min = *counts.clone().min().unwrap_or(&0);
    let uniq = counts.count();

    match max {
        max if max + jokers == 5 => CardType::FiveOfAKind as u8,
        max if max + jokers == 4 => CardType::FourOfAKind as u8,
        max if max + jokers == 3 && uniq == 2 => CardType::FullHouse as u8,
        max if max + jokers == 3 && min == 1 => CardType::ThreeOfAKind as u8,
        2 if uniq == 3 && jokers == 0 => CardType::TwoPairs as u8,
        max if max == 2 && jokers == 0 && uniq == 4 || max == 1 && jokers == 1 => {
            CardType::OnePair as u8
        }
        1 if jokers == 0 => CardType::HighCard as u8,
        _ => panic!("couldn't get hand type"),
    }
}

fn cmp_hands(a: &str, b: &str) -> Ordering {
    let a_type = get_type(a);
    let b_type = get_type(b);

    let ordering = a_type.cmp(&b_type);
    if ordering != Ordering::Equal {
        return ordering;
    }

    for (c1, c2) in a.chars().zip(b.chars()) {
        let ordering = card_rank(c1).cmp(&card_rank(c2));
        if ordering != Ordering::Equal {
            return ordering;
        }
    }

    Ordering::Equal
}

fn main() {
    let mut hands = include_str!("../input.txt")
        .lines()
        .map(|l| l.split(' '))
        .map(|mut split| {
            (
                split.next().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(&str, usize)>>();

    hands.sort_by(|a, b| cmp_hands(a.0, b.0));

    let score: usize = hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1))
        .sum();
    println!("{score}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_type() {
        assert_eq!(get_type("AAAAA"), CardType::FiveOfAKind as u8);
        assert_eq!(get_type("AAAAK"), CardType::FourOfAKind as u8);
        assert_eq!(get_type("AAAKK"), CardType::FullHouse as u8);
        assert_eq!(get_type("AAAK3"), CardType::ThreeOfAKind as u8);
        assert_eq!(get_type("AAKKC"), CardType::TwoPairs as u8);
        assert_eq!(get_type("AAK32"), CardType::OnePair as u8);
        assert_eq!(get_type("23456"), CardType::HighCard as u8);
    }

    #[test]
    fn test_cmp_hands() {
        assert_eq!(cmp_hands("KK677", "KTJJT"), Ordering::Less);
    }
}
