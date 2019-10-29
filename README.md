# cabo.rs
> A fun card game all about mind-games and outwitting your opponent, coded in rust for some reason

## How to play
General information of the game [here](https://en.wikipedia.org/wiki/Cabo_(game)). Our version probably differs ALOT

### Goal
The smaller the better! Win by having the smallest sum

### Number of players
Recommended for only 4 players, no more no less despite what Leslie Moore says

### Tools Needed
1. Standard card deck (52 cards)
2. Friends

### Difficulty
Easy to learn, fun for all ages (common sense dictates 3 and up)

### Gameplay
1. Each player is dealt 3 cards face down, the rest of the pile goes in the middle
2. Players can look at their *own* left and right card, the middle card remains face down
3. First player draws one card from the top of the pile and decides
    - Discard the card
    - Swap it with an existing
    - If its a power card, play it (or choose to save it for later by swapping *sneaky sneaky*)
4. Repeat step 3 until the pile is depleted and the game ends
5. Tally up the numbers in your remaining cards, lowest score wins!

#### Fun spins
1. Add the joker into the game! It counts as 0
2. Speedrun the game! Game ends in 3 minutes for a lightning round

### Cards
Cards are the value it is (Ace is 1, 2 is 2 etc) with the exception of **power cards**:
- Jack - swap card, when played you can swap two cards of any players with each other **face down** (aka you dont see what you swap)
- Queen - look card, look :eyes: at one card from any player (with the exception of the draw pile)
- King - kill card, discard any one card from any player (add it to the discard pile face up), the player fills that spot with the top card from the draw pile *face down* (aka no one sees what it is)

At the end of the game, power cards count as ten (10) value each.
