function cc(cards = []) {
  const sum = cards.reduce((previousValue, currentValue) => {
    if ([2, 3, 4, 5, 6].includes(currentValue)) {
      return previousValue + 1
    } else if ([10, "J", "Q", "K", "A"].includes(currentValue)) {
      return previousValue - 1
    } else {
      return previousValue
    }
  }, 0)
  return `${sum} ` + (sum <= 0 ? "Hold" : "Bet")
}

console.log(cc([2, 3, 4, 5, 6]), "5 Bet")
console.log(cc([2, 7, 4, "Q"]), "1 Bet")
console.log(cc([2, "K", 4, "Q", "A"]), "-1 Hold")

// In the casino game Blackjack, a player can gain an advantage over the house by keeping track of the relative number of high and low cards remaining in the deck. This is called Card Counting.

// Having more high cards remaining in the deck favors the player. Each card is assigned a value according to the table below. When the count is positive, the player should bet high. When the count is zero or negative, the player should bet low.

// Count Change: Cards:

//  +1: 2, 3, 4, 5, 6
//   0: 7, 8, 9
//  -1: 10, 'J', 'Q', 'K', 'A'
// You will write a card counting function. It will receive a card parameter and increment or decrement the local count variable according to the card's value (see table). The function will then return a string with the current count and the string "Bet" if the count is positive, or "Hold" if the count is zero or negative. The current count and the player's decision ("Bet" or "Hold") should be separated by a single space.

// Example Output :

// "-3 Hold"
// "5 Bet"
