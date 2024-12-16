mod card;
mod deck;

use deck::Deck;
use card::Card;
use rand::Rng;
use std::io;
use colored::Colorize;

/*
 * This is an enumeration type (Sebesta, 6.4)
 * It enumerates the possible hands in poker
 */
#[derive(Clone, Debug)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

/*
 * The struct type in Rust is a record type (Sebesta, 6.7)
 * It is an aggregate data type with two elements named cards and chips.
 */
#[derive(Clone)]
struct Player {    
    cards: Vec<Card>,
    chips: i32,
}

/*
 * The struct type in Rust is a record type (Sebesta, 6.7)
 * It is an aggregate data type with three elements named hand, rank_score, and suit_score.
 */
#[derive(Debug)]
struct HandScore {
    hand: Hand,
    rank_score: usize,
    suit_score: usize,
}

impl HandScore {
    fn new(hand: Hand, rank_score: usize, suit_score: usize) -> HandScore {
        HandScore { hand, rank_score, suit_score }
    }
}

impl Player {
    fn new(starting_chips: i32) -> Player {
        Player { cards: Vec::new(), chips: starting_chips }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn update_chips(&mut self, amount: i32) {
        self.chips += amount;
    }
}

/*
 * The parameter players is a reference to a vector of Player structs. It is a reference type (Sebesta, 6.11).
 */
fn find_winning_hand(players: &Vec<Player>, community_cards: &Vec<Card>, bets: &Vec<i32>) -> usize {
    /*
        This function is used to find the score of a player's hand. It takes a vector of cards as input and returns a HandScore struct.
     */
    fn hand_score(hand: &Vec<Card>) -> HandScore {
        let mut current_hand = Hand::HighCard;

        let mut ranks = vec![0; 13];
        let mut suits = vec![0; 4];

        // Marks the ranks and suits of the cards
        for card in hand.iter() {
            ranks[card.rank.clone() as usize] += 1;
            suits[card.suit.clone() as usize] += 1;
        }

        // Checks for 5 or more of one suit
        let is_flush = suits.iter().any(|&count| count >= 5);
        // Checks for 5 or more consecutive ranks
        let is_straight = ranks.windows(5).any(|window| window.iter().all(|&count| count > 0));

        if is_flush {
            current_hand = Hand::Flush; 
        }

        if is_straight {
            current_hand = Hand::Straight; 
        }

        // Checks for a straight flush
        if is_flush && is_straight {
            current_hand = Hand::StraightFlush;
        }

        // Checks for four of a kind
        if ranks.iter().any(|&count| count == 4) {
            current_hand = Hand::FourOfAKind;
        }

        // Checks for a full house

        // Finds the rank of the triple
        let mut triple_rank = ranks.iter().position(|&count| count == 3);
        if ranks.iter().filter(|&&count| count == 3).count() == 2 {
            triple_rank = ranks.iter().skip(triple_rank.unwrap() + 1).position(|&count| count == 3);
        }
        if triple_rank.is_some() {
            // first pair
            let mut pair_rank = ranks.iter().position(|&count| count == 2);
            // Deals with case of two pairs
            if ranks.iter().filter(|&&count| count == 2).count() == 2 {
                if ranks.iter().skip(pair_rank.unwrap() + 1).position(|&count| count == 2) != triple_rank {
                    pair_rank = ranks.iter().skip(pair_rank.unwrap() + 1).position(|&count| count == 2);
                }
            }
            // Deals with case of three pairs
            if ranks.iter().filter(|&&count| count == 2).count() == 3 {
                // second pair
                let mut next_pair_rank = ranks.iter().skip(pair_rank.unwrap() + 1).position(|&count| count == 2);
                // second highest pair is no the same as the triple
                if next_pair_rank != triple_rank {
                    pair_rank = next_pair_rank;
                    
                } 
                // third pair
                next_pair_rank = ranks.iter().skip(next_pair_rank.unwrap() + 1).position(|&count| count == 2);
                // third pair is not the same as the triple
                if next_pair_rank != triple_rank {
                    pair_rank = next_pair_rank;
                }
                
            }
            if pair_rank.is_some() {
                current_hand = Hand::FullHouse;
            }
        }

        
        // Checks for three of a kind
        if triple_rank.is_some() {
            current_hand = Hand::ThreeOfAKind;
        }
        //  Checks for two pair 
        if ranks.iter().filter(|&&count| count == 2).count() == 2 {
            current_hand = Hand::TwoPair;
        }

        if ranks.iter().any(|&count| count == 2) {
            current_hand = Hand::OnePair;
        }
        let mut rank_score = 0;
        // Adds points for the ranks of the cards (high cards)
        for (i, rank) in ranks.iter().enumerate() {
            rank_score += i * rank;
        }
        // Adds points for the suits of the cards
        let mut suit_score = 0;
        for (i, suit) in suits.iter().enumerate() {
            suit_score += i * suit;
        }
        HandScore::new(current_hand, rank_score, suit_score)
    }

    // Finds the score of each player's hand
    let mut hand_scores = Vec::new();
    for player in players.iter() {
        let mut all_cards = player.cards.clone();
        all_cards.extend_from_slice(community_cards);
        hand_scores.push(hand_score(&all_cards));
    }

    let mut tie = true;
    // sets initial score type to check
    // 0: check score by poker hand
    // 1: check score by card ranks (high card)
    // 2: check score by card suits (highest suit)

    let mut current_score_type = 0;
    let mut winning_players = Vec::new();
    let mut current_players = Vec::new();
        for (i, _player) in players.iter().enumerate() {
            // Skip players who have folded
            if bets[i] != -1 {
                current_players.push(i);
            }
        }

    // Repeat until single winner found
    while tie {
        let mut best_hand = 0;
        
        // Finds the best hand and the winning player for a score type
        for player in current_players.iter() {
            let mut score = hand_scores[*player].hand.clone() as usize;
            if current_score_type == 1 {
                score = hand_scores[*player].rank_score;
            } else if current_score_type == 2 {
                score = hand_scores[*player].suit_score;
            }
            if score > best_hand {
                best_hand = score;
                winning_players = vec![*player];
                tie = false;
            } else if score == best_hand {
                tie = true;
                winning_players.push(*player);
            }
        }
        // If multiple players have the same score, move to the next score type to check
        if tie {
            current_score_type += 1;
            current_players = winning_players.clone();
        }
    }
    if winning_players.len() > 1 {
        println!("Error, tie between players:");
        for player in winning_players.iter() {
            println!("{}", player);
        }
    }

    winning_players[0]
}

/*
    This function is used to get the bets from the players. It cycles through the players and asks them to bet or fold. The function returns a vector of the bets made by each player.
*/
fn get_bets(players: &mut Vec<Player>, starting_better: usize, bets: &mut Vec<i32>, pot: &mut i32, community_cards: & Vec<Card>) -> Vec<i32> {
    let mut current_better = starting_better;
    let mut current_bet = 0;
    let mut all_bets_in = false;
    let mut players_visited = 0;
    while !all_bets_in || players_visited < players.len() {
        
        // Skip players who have folded
        if bets[current_better] == -1 {
            // Move to the next player
            current_better = (current_better + 1) % players.len();
            // Check if all bets are in
            for bet in bets.iter() {
                if *bet != -1 && *bet != current_bet {
                    all_bets_in = false;
                    break;
                }
                all_bets_in = true;
            }
            players_visited += 1;
            continue;
        }

        // User's turn to bet
        if current_better == 0 {
            
            // Print the current bets of all players
            if starting_better != 0 || players_visited > 0 {
                println!("\nCurrent bets:");
                println!("{}", "----------------------------".blue());
                for (i, bet) in bets.iter().enumerate() {
                    if *bet == -1 { // Player has folded
                        println!("{:<27}{}", format!("Player {} has folded", i + 1), "|".blue());
                    } else if *bet == -2 { // Player has not bet yet
                        if i == 0 {
                            println!("{:<27}{}", format!("You have not bet yet"), "|".blue());
                        } else {
                            println!("{:<27}{}", format!("Player {} has not bet yet", i + 1), "|".blue());
                        }
                    } else {
                        if i == 0 {
                            println!("{:<27}{}", format!("Your current bet: {}", bet), "|".blue());
                        } else {
                            println!("{:<27}{}", format!("Player {}'s current bet: {}", i + 1, bet), "|".blue());
                        }
                    }
                }
                println!("{}", "----------------------------".blue());
                println!("It's your turn to bet");
            } else { // User is the first to bet, no need to show current bets
                println!("\nYou are betting first");
            }
            println!("{}", "\n+==================+".bold().red());
            println!("{}{:^18}{}", "|".bold().red(), "Your Cards:", "|".bold().red());
            println!("{}{}{}", "+".red().bold(), "------------------".white(), "+".red().bold());
            for card in players[0].cards.iter() {
                println!("{}{:^18}{}", "|".bold().red(), format!("{:?} of {:?}", card.rank, card.suit).bold(), "|".bold().red());
            }
            if community_cards.len() > 0 {
                println!("{}", "+==================+".bold().red());
                println!("{}{:^18}{}", "|".bold().red(), "Community Cards:", "|".bold().red());
                println!("{}{}{}", "+".red().bold(), "------------------".white(), "+".red().bold());
                for card in community_cards.iter() {
                    println!("{}{:^18}{}", "|".bold().red(), format!("{:?} of {:?}", card.rank, card.suit).bold(), "|".bold().red());
                }
            }
            println!("{}", "+==================+".bold().red());
            println!("{}{:^18}{}", "|".bold().red(), format!("Your chips: {}", players[current_better].chips), "|".bold().red());
            println!("{}", "+==================+\n".bold().red());
            println!("{}", "--------------------------------------------------".bold().white());
            let mut valid_bet = false;
            while !valid_bet {
                println!("The current bet is: {}", format!("{}", current_bet).bold());
                println!("Enter your bet (-1 to fold): ");

                // Read the bet from the user   
                let mut bet = String::new();
                io::stdin().read_line(&mut bet).expect("Failed to read line");
                let bet: i32 = bet.trim().parse().expect("Please enter a number");

                // check if the bet is valid
                if bet == -1 {
                    println!("You fold");
                    bets[current_better] = -1;
                    valid_bet = true;
                } else if bet < -1 {
                    println!("Please enter a valid bet");
                } else if bet > players[current_better].chips {
                    println!("You don't have enough chips to bet that amount, please enter a valid bet");
                } else if bet < current_bet {
                    println!("You must bet at least the current bet {}, please enter a valid bet", current_bet);
                } else {
                    // Update the current bet, the player's chips, the pot, and the bets
                    current_bet = bet;
                    players[current_better].update_chips(-bet);
                    *pot += bet;
                    bets[current_better] = bet;
                    valid_bet = true;
                
                }
            }
        } else {
            // Computer's turn to bet
            let mut rng = rand::thread_rng();
            let bet_or_fold = rng.gen_range(1..=4);
            if bet_or_fold == 1{
                // Random bet between current bet and max chips, favors lower bets
                let bet = (rng.gen_range(0..=25) as f64 / 100.0).powi(2) * (players[current_better].chips - current_bet) as f64 + current_bet as f64;
                let bet = bet as i32;
                players[current_better].update_chips(-bet);
                *pot += bet;
                bets[current_better] = bet;
                current_bet = bet;
                println!("\nPlayer {} bets {}", current_better + 1, bet);
            } else if bet_or_fold <= 3 {
                // Call the current bet
                players[current_better].update_chips(-current_bet);
                *pot += current_bet;
                bets[current_better] = current_bet;
                println!("\nPlayer {} bets {}", current_better + 1, current_bet);
            } else {
                println!("\nPlayer {} folds", current_better + 1);
                bets[current_better] = -1;
            }
        }
        // Move to the next player
        current_better = (current_better + 1) % players.len();
        players_visited += 1;
        // Check if all bets are in
        for bet in bets.iter() {
            if *bet != -1 && *bet != current_bet {
                all_bets_in = false;
                break;
            }
            all_bets_in = true;
        }
    }

    // Reset the bets, except for the players who have folded
    // -2 is used to indicate that the player has not yet bet
    for bet in bets.iter_mut() {
        if *bet != -1 {
            *bet = -2;
        }
    }

    bets.to_vec()
}

fn game(players: &mut Vec<Player>, starting_better: usize) {
    // Initialize the deck
    let mut deck = Deck::new();
    deck.shuffle();

    // Reset the players' cards
    for player in players.iter_mut() {
        player.cards = Vec::new();
    }

    // Deal  first two cards to each player
    for _ in 0..2 {
        for player in players.iter_mut() {
            player.add(deck.deal().unwrap());
        }
    }

    println!("{}", "--------------------------------------------------\n".bold().white());

    // Print the cards of the user
    println!("{}", "=================".red());
    println!("{}", "Your cards:".white());
    for card in players[0].cards.iter() {
        println!("{}", format!("{:?} of {:?}", card.rank, card.suit).white());
    }
    println!("{}", "=================\n".red());


    // Wait for user to view cards and continue
    println!("Press {} to begin betting", "Enter".bold());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("{}", "--------------------------------------------------".bold().white());

    let mut bets = vec![-2; players.len()];
    let mut pot = 0;
    let mut community_cards = Vec::new();

    // Betting round 1
    bets = get_bets(players, starting_better, &mut bets, &mut pot, &community_cards);
    println!("\nAll bets are in, the pot is now {}", pot);


    // Wait for user to view bets and continue
    println!("\nPress {} to deal the flop", "Enter".bold());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Burn a card
    deck.deal();

    println!("{}", "--------------------------------------------------".bold().white());

    // Deal the flop
    for _ in 0..3 {
        community_cards.push(deck.deal().unwrap());
    }
    // Print the flop
    println!("{}", "\nCards turned:".green());
    println!("{}", "+----------------+".green());
    for card in community_cards.iter() {
        println!("{:^18}", format!("{:?} of {:?}", card.rank, card.suit));
    }
    println!("{}", "+----------------+".green());


    // Wait for user to view cards and continue
    println!("Press {} to begin betting", "Enter".bold());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("{}", "--------------------------------------------------".bold().white());

    // Betting round 2
    bets = get_bets(players, starting_better, &mut bets, &mut pot, &community_cards);
    println!("\nAll bets are in, the pot is now {}", pot);


    // Wait for user to view bets and continue
    println!("\nPress {} to deal the turn", "Enter".bold());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("{}", "--------------------------------------------------".bold().white());

    // Burn a card
    deck.deal();

    // Deal the turn
    community_cards.push(deck.deal().unwrap());
    // Print the community cards
    println!("{}", "\nCards turned:".green());
    println!("{}", "+----------------+".green());
    for card in community_cards.iter() {
        println!("{:^18}", format!("{:?} of {:?}", card.rank, card.suit));
    }
    println!("{}", "+----------------+".green());


    // Wait for user to view cards and continue
    println!("Press {} to begin betting", "Enter".bold());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("{}", "--------------------------------------------------".bold().white());

    // Betting round 3
    bets = get_bets(players, starting_better, &mut bets, &mut pot, &community_cards);
    println!("\nAll bets are in, the pot is now {}", pot);


    // Wait for user to view bets and continue
    println!("\nPress {} to deal the river", "Enter".bold());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("{}", "--------------------------------------------------".bold().white());

    // Burn a card
    deck.deal();
    // Deal the turn
    community_cards.push(deck.deal().unwrap());
    // Print the community cards
    println!("{}", "\nCards turned:".green());
    println!("{}", "+----------------+".green());
    for card in community_cards.iter() {
        println!("{:^18}", format!("{:?} of {:?}", card.rank, card.suit));
    }
    println!("{}", "+----------------+".green());


    // Wait for user to view cards and continue
    println!("Press {} to begin betting", "Enter".bold());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("{}", "--------------------------------------------------".bold().white());

    // Betting round 4
    bets = get_bets(players, starting_better, &mut bets, &mut pot, &community_cards);
    println!("\nAll bets are in, the pot is now {}", pot);


    // Wait for user to view bets and continue
    println!("\nPress {} to reveal the winning hand", "Enter".bold());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("{}", "--------------------------------------------------".bold().white());

    // Find the winning player
    let winning_player = find_winning_hand(players, &community_cards, &bets);
    players[winning_player].update_chips(pot);

    // Print the winning player
    if winning_player == 0 {
        println!("You have the best hand");
    } else {
        println!("{} {} has the best hand\n", "Player", format!("{}", winning_player + 1).bold().red().on_yellow());
        println!("Player {}'s hand:", winning_player + 1);
        println!("{}", "+====================+".bold().yellow());
        for card in players[winning_player].cards.iter() {
            println!("{}{:<20}{}", "|".bold().yellow(), format!("{:?} of {:?}", card.rank, card.suit), "|".bold().yellow());
        }
        println!("{}", "+====================+\n".bold().yellow());
    }

    // Print the chips of each player
    println!("\nEnd of round, each player has the following chips:");
    println!("{}", "+====================+".bold().cyan());
    for (i, player) in players.iter().enumerate() {
        if i == 0 {
            println!("{}{:<20}{}", "|".bold().cyan(), format!("You: {}", player.chips).bold().red(), "|".bold().cyan());
        } else {
            println!("{}{:<20}{}", "|".bold().cyan(), format!("Player {}: {}", i + 1, player.chips), "|".bold().cyan());
        }
    }
    println!("{}", "+====================+\n".bold().cyan());

    println!("{}", "--------------------------------------------------".bold().white());

}

fn main() {
    /*
     * This is a primitive data type (Sebesta, 6.2)
     * In Rust, the type of num_of_players is inferred from the context. In this case the type will be set to i32, a 32-bit signed integer (Sebesta, 6.2.1.1).
     */
    let mut num_of_players = 5;
    // Starting chips for each player
    let mut starting_chips = 1000;

    println!("{}", "===================================".bold().dimmed().yellow());
    println!("{} {} {}", "|".bold().dimmed().yellow(), "Welcome to Texas Hold'em Poker!".bold().yellow(), "|".bold().dimmed().yellow());
    println!("{}", "===================================\n\n".bold().dimmed().yellow());

    // Let user choose the number of players
    println!("How many players are playing? [>2] (default is 5)");

    /*
     * The String type in Rust is a sequence of Unicode characters (Sebesta, 6.3). It is not a primitve data type and has dynamic length (Sebesta, 6.3.1).
     */
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    if !input.trim().is_empty() {
        num_of_players = input.trim().parse::<usize>().expect("Please enter a number");
        if num_of_players < 2 {
            println!("Invalid number of players, defaulting to 5 players");
        }
    } else {
        println!("Setting the number of players to 5\n");
    }

    // Let user choose the starting chips for each player
    println!("How many chips does each player start with? [>10] (default is 1000)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    if !input.trim().is_empty() {
        starting_chips = input.trim().parse::<i32>().expect("Please enter a number");
        if starting_chips < 10 {
            println!("Invalid number of chips, defaulting to 1000 chips");
        }
    } else {
        println!("Setting the number of chips to 1000\n");
    }

    /*
     * players is a vector type in Rust. Rust vectors
     * are Lists (Sebesta, 6.9) that can grow and shrink in size.
     * Rust vectors have several methods. The vec! macro is used to * create a new vector with the specified elements.
     */
    let mut players = vec![Player::new(starting_chips); num_of_players];

    let mut round = 1;

    /*
     * This is a primitive data type (Sebesta, 6.2)
     * play_again is a boolean type in Rust (Sebesta, 6.2.2)
     * Booleans are used to represent true or 
     * false values.
     */
    let mut play_again = true;
    // Game loop
    while play_again {
        println!("{:^35}", "+---------+");
        println!("{:^35}", format!("| Round {} |", round));
        println!("{:^37}", "+---------+\n\n");
        game(&mut players, (round - 1) % num_of_players);
        if players[0].chips <= 0 {
            println!("You have run out of chips, game over!");
            break;
        }
        println!("\nDo you want to play another round? ({}/{})", "y".bold().green(), "n".bold().red());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == "n" {
            play_again = false;
        }
        round += 1;
    }


}