use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};

use rand::prelude::*;

//COMMANDS
static HELP: &str = "!help";
static ROLL: &str = "!roll";

//REPLIES
static HELP_REPLY: &str = "
Current commands:
 => !help - Sends this message into the room where this was called
 => !roll - You provide a series of dice, and the roll with be done for you
    -> Example: !roll 2d6 + 1d8 - 2

Details:
 => This bot is even in the personal chat between you and the DM. This means that if you need to roll for something hidden you can!
 => You should always receive feedback regarding the roll. This means either it succeeds and you see the result, or it fails and you get told to try again.
 => If you encounter what you believe is a bug, dm Elijah.
";

static INVALID_REPLY: &str = "Invalid input. Please try again";

//Handler
struct Handler;

impl Handler {
    fn compute_result(msg : String) -> Result<String, &'static str> {
            let parsed_message = msg
                .strip_prefix(ROLL)
                .unwrap()
                .trim()
                .split(' ')
                .collect::<Vec<_>>();

            let mut dice_rolls = vec![];
            let mut ops = vec![];

            for comp in parsed_message {
                if comp == "+" || comp == "-" {
                    ops.push(comp);
                }
                else {
                    let die_res = Handler::roll_dice(comp)?;
                    dice_rolls.push(die_res);
                }
            }

            Ok(Handler::formulate_result(&dice_rolls, &ops))
    }
    
    fn roll_dice(message : &str) -> Result<Vec<usize>, &'static str> {
        let nums = message
            .split('d')
            .map(|num| num.parse::<usize>())
            .collect::<Vec<_>>();
        
        if nums.iter().any(|x| x.is_err()) { 
            return Err(INVALID_REPLY); 
        }

        match nums.len() {
            2 => {
                let mut ans = vec![];
                for _ in 0..*nums[0].as_ref().unwrap() {
                    ans.push(rand::thread_rng().gen_range(1..=*nums[1].as_ref().unwrap()));
                }
                Ok(ans)
            }, 
            1 => Ok(vec![*nums[0].as_ref().unwrap()]),
            _ => Err(INVALID_REPLY),
        }
    }

    fn formulate_result(rolls : &[Vec<usize>], ops : &[&str]) -> String {
        let mut ans = String::new();
        let mut sum : i32 = 0;

        for (i, vec) in rolls.iter().enumerate() {
            let sign = match ops.get(i.overflowing_sub(1).0) {
                Some(&"+") => { ans.push_str(" + "); "+" },
                Some(&"-") => { ans.push_str(" - "); "-" },
                _ => "+",
            };

            ans.push('(');
            for (j, num) in vec.iter().enumerate() {
                ans.push_str(&num.to_string());
                if j != vec.len() - 1 { ans.push_str(" + "); }
                sum = if sign == "-" { sum - *num as i32 } else { sum + *num as i32 };
            }
            ans.push(')');
        }

        ans.push_str(&format!(" = {}", sum));
        ans
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        if msg.content.starts_with(HELP) {
            if let Err(why) = msg.channel_id.say(&context.http, HELP_REPLY).await {
                println!("Unable to send message: {}", why);
            }
        }
        else if msg.content.starts_with(ROLL) {
            let response = match Handler::compute_result(msg.content) {
                Ok(result) => {
                    MessageBuilder::new()
                        .push_bold_safe(&msg.author.name)
                        .push(" rolled: \n")
                        .push(&result)
                        .build()
                },
                Err(e) => {
                    e.to_string()
                },
            };

            if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                println!("Unable to send message: {}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
