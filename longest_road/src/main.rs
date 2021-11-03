/*
    * Goal *
    Determine which player in a rectangular "Settlers of Catan" themed game board has the longest road.

    Given an ascii game board:

        aa##b
        #A##B
        #aa#b
        ##a##

    The lower case letters "a" denote a road belonging to player A.
    Uppercase letters denote a settlement.

    If a player has at least 5 consecutive (non-repeating) roads then they can be awarded the "longest road" victory points.
    Roads connected diagonally are not considered consecutive. Roads can be linked together by settlements, but the settlements do not count towards the total length of the player's roads. In the above example player A would have the longest road with a length of 5.

    The input will never include the case where multiple players are tied for longest road.

    Loops and branches
    A road may form a loop or branch out in multiple directions. In all cases the longest possible consecutive link of roads is used to determine the players' longest roads.

    Inspired by the board game "Settlers of Catan" by Klaus Teuber.
    Image by dograapps from Pixabay.

    *Input
    Line 1: Integer n the length of the square board.
    Next n lines: String of characters representing the game board. # represents an open space. Lowercase letters represent roads, and uppercase letters represent settlements.
    *Output
    Output the capital letter of the player with the longest road, and the length of the road, e.g. A 6.
    If no player has the longest road then output the number 0.
    *Constraints
    5 ≤ n ≤ 10

    Learned skills: Depth first search, Ascii art
*/

fn main() {
    let n = 5;
    let mut lines = Vec::new();

    // * Level 1
    // 5
    // ["#a###", "#a###", "#a###", "#aa##", "##a##"]
    // A 6

    // * Level 2
    // 5
    // ["aa#a#", "#A#aA", "#a##a", "#aa##", "##a##"]
    // A 6

    // * Level 3
    // 5
    // ["#####", "#A###", "#a###", "#aA##", "##aa#"]
    // 0

    // * Level 4
    // 5
    // ["#A#bb", "#a##b", "####B", "#Aa#b", "##abb"]
    // B 6

    // * Level 5
    // 10
    // ["Bb###aA###", "b#Cc#a####", "###c#a####", "###ccCccc#", "#####a####", "#####aAaaa", "######d###", "#dBbb#D###", "#d####d###", "#D####d###"]
    // C 7

    // * Level 6
    // 5
    // ["aaaaa", "A#a#a", "aaaa#", "#a###", "aaaaa"]
    // A 13

    lines.push("aaaaa".to_string());
    lines.push("A#a#a".to_string());
    lines.push("aaaa#".to_string());
    lines.push("#a###".to_string());
    lines.push("aaaaa".to_string());

    // Initiate empty scoreboard
    let mut score_board: Vec<(u8, i16)> = Vec::new();

    // Go through all possible positions
    for y in 0..n {
        for x in 0..n {
            // pathfind recursively
            pathfind((x, y), &lines, &mut score_board, None, None, None);
        }
    }

    // Print best score
    if score_board.len() > 0 {
        score_board.sort_by_key(|k| k.1);
        let best_score = score_board.pop().unwrap();
        println!("{} {}", best_score.0 as char, best_score.1);
    } else {
        println!("0")
    }
}

fn pathfind(
    pos: (i16, i16),
    lines: &Vec<String>,
    score_board: &mut Vec<(u8, i16)>,
    player: Option<u8>,
    history: Option<Vec<(i16, i16)>>,
    score: Option<i16>,
) {
    // Get default values for score, history and player
    let mut score = score.unwrap_or(0);
    let mut history = history.unwrap_or(Vec::new());

    let char_at_pos = lines[pos.1 as usize].as_bytes()[pos.0 as usize];
    let mut player = player.unwrap_or(char_at_pos);
    if player >= b'a' {
        player -= 32;
    }

    // Check if value at position is already in history or if emtpy
    if history.contains(&pos) || char_at_pos == b'#' {
        add_score(score_board, player, score);
        return;
    }

    // Check if player's tile, increase score if road (a-road, A-city), otherwise cancel road
    if char_at_pos == player {
        history.push(pos);
    } else if char_at_pos >= b'a' && char_at_pos - 32 == player {
        history.push(pos);
        score += 1;
    } else {
        add_score(score_board, player, score);
        return;
    }

    // Check neighbours
    let world_size = lines.len();
    let diffs: [(i16, i16); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    for (dx, dy) in &diffs {
        let neighbour_pos = (pos.0 + dx, pos.1 + dy);

        if (neighbour_pos.0 < 0 || neighbour_pos.0 + 1 > world_size as i16)
            || (neighbour_pos.1 < 0 || neighbour_pos.1 + 1 > world_size as i16)
        {
            continue;
        } else {
            pathfind(
                neighbour_pos,
                &lines,
                score_board,
                Some(player),
                Some(Vec::clone(&history)),
                Some(score),
            );
        }
    }
}

fn add_score(score_board: &mut Vec<(u8, i16)>, player: u8, score: i16) {
    if score >= 5 && !score_board.contains(&(player, score)) {
        score_board.push((player, score));
    }
}
