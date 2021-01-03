# Dice-Master

The DM for a campaign I play in asked if I could create or find a bot that rolled dice on the server as a way to allow those without dice to play, and to keep people honest. I started learning Rust a couple months ago, and took it as a challenge to learn how to get a bot going in a mostly unfamiliar language. At this point, here we are. 

I have used serenity and tokio in order to get access to the Discord API as well as work asynchronously. If you would like to understand them, I highly recommend checking out their respective pages which can be found:

[Serenity](https://github.com/serenity-rs/serenity)

[Tokio](https://tokio.rs)

## Functionality:

The bot supports the following commands:
  
**!roll**
- The player provides a string of dice like: 1d6 + 2d8 + 4 and the result is displayed with each set of dice in parenthesis, and the final result tallied in the end.
  
**!help**
- Explains the commands and what to do in the event of a bug.

## Notes:
  - The bot has a limit of 492 single-digit sided dice. I have not tested with double sided because if someone has to roll more than 50 d20s, then they have a problem.
  - This bot does not currently support leaving the 1 off of a roll like: 1d6. "!roll d6 + 2" will result in an invalid input error and the player will be asked to provide valid input.
  - The player should always receive feedback after providing a command, whether this be in the form of "Invalid Input" or the actual expected response. Should a message not be sent, the server will print a line to console with the reasoning.
 
## Feedback:
  - If anyone has some improvements, feel free to make a pull request and I will give it a look!
