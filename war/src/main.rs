use std::cmp::Ordering;

// input
// 3
// AD
// KC
// QC
// 3
// KH
// QS
// JC

// Expected output
// 1 3
// meaning
// player 1 won in 3 rounds

fn main() {
    // convert cards into numbers - colors of cards doesn't matter

    // define players decks
    let mut player1 = vec![
        6, 7, 6, 12, 7, 8, 6, 5, 6, 12, 4, 3, 7, 3, 4, 5, 12, 5, 3, 3, 8, 4, 4, 12, 5, 7,
    ];
    let mut player2 = vec![
        11, 14, 13, 14, 9, 2, 2, 11, 10, 13, 10, 11, 11, 9, 9, 13, 14, 13, 10, 8, 2, 10, 8, 14, 2,
        9,
    ];
    // let mut player1 = vec![14, 13, 12];
    // let mut player2 = vec![13, 12, 11];

    let mut war1 = Vec::new();
    let mut war2 = Vec::new();

    let mut is_war = false;

    let winner;
    let mut counter = 0;

    loop {
        eprintln!("~~~");
        eprintln!("p1\t{:?}", player1);
        eprintln!("p2\t{:?}", player2);
        let card1 = match get_first_card(&mut player1) {
            Some(value) => value,
            None => {
                winner = match is_war {
                    true => "PAT",
                    false => "2",
                };
                break;
            }
        };

        let card2 = match get_first_card(&mut player2) {
            Some(value) => value,
            None => {
                winner = match is_war {
                    true => "PAT",
                    false => "1",
                };
                break;
            }
        };
        eprintln!("---\n{}", counter);
        eprintln!("{} vs {}", card1, card2);

        if !is_war {
            counter += 1;
        }

        war1.push(card1);
        war2.push(card2);

        match card1.cmp(&card2) {
            Ordering::Greater => {
                eprintln!("p1 won");
                player1 = [player1, war1.clone(), war2.clone()].concat();

                war1.clear();
                war2.clear();
                is_war = false;
            }
            Ordering::Less => {
                eprintln!("p2 won");
                // player2 = [player2, war2.clone(), war1.clone()].concat();
                player2 = [player2, war1.clone(), war2.clone()].concat();

                war1.clear();
                war2.clear();
                is_war = false;
            }
            Ordering::Equal => {
                //war
                eprintln!("war");
                is_war = true;
                for _ in [0..3] {
                    let p1 = get_first_card(&mut player1);
                    let p2 = get_first_card(&mut player2);
                    // war2.push(get_first_card(&mut &mut player2).unwrap_or_else(|| winner = "PAT"));

                    match (p1, p2) {
                        (Some(x), Some(y)) => {
                            war1.push(x);
                            war2.push(y);
                        }
                        _ => continue,
                    }
                }
            }
        }
    }

    if winner == "PAT" {
        println!("{}", winner);
    } else {
        println!("{} {}", winner, counter);
    }
}

fn _convert_card_to_val(name: String) -> i32 {
    let (str_value, _) = name.split_at(name.len() - 1);
    // println!("{:?}", str_value);

    match str_value {
        "J" => 11,
        "Q" => 12,
        "K" => 13,
        "A" => 14,
        _ => str_value.parse().unwrap(),
    }
}

fn get_first_card(deck: &mut Vec<i32>) -> Option<i32> {
    if deck.len() == 0 {
        return None;
    }
    let card = deck.remove(0);
    Some(card)
}
