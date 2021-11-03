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
    let mut player1 = vec![1, 2, 3];
    let mut player2 = vec![1, 2, 5];

    loop {
        let card1 = match get_first_card(&mut player1) {
            Some(value) => value,
            None => break,
        };

        let card2 = match get_first_card(&mut player2) {
            Some(value) => value,
            None => break,
        };

        println!("{}", card1);
    }

    println!("Endgame")
}

fn get_first_card(deck: &mut Vec<i32>) -> Option<i32> {
    if deck.len() == 0 {
        return None;
    }
    let card = deck.remove(0);
    Some(card)
}
