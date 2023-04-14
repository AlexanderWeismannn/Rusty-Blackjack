use std::cmp::Ordering;

pub struct Player{
    pub name:String,
    pub hand:Vec<i32>,
    pub balance: i32,
    pub sum: i32
}

impl Player{

    //draws a card from the deck and puts it in the players hand
    pub fn draw_card(&mut self,card_deck: &mut Vec<i32>) -> i32{
        let val: i32 = card_deck.pop().unwrap();
        self.hand.push(val);
        return self.sum_value();
    }

    //takes in player bet amount and reduces the balance if able to
    pub fn bet(&mut self, amount: i32){
        match self.balance.cmp(&amount) {
            Ordering::Less => println!("You're too poor! Bet <= {}",self.balance),
            Ordering::Greater => self.balance -= &amount,
            Ordering::Equal => self.balance -= &amount,  
        };
    }

    //prints the players full hand at the moment
    pub fn print_hand(&self){
        println!("{:?}",self.hand)
    }

    //get the total sum of the player hand, accounts for aces/1 conversion and returns the value
    pub fn sum_value(&mut self) -> i32{
        let sum: i32 = self.hand.iter().sum();
        //look for the first occurance of 11 in the vector
        if let Some(index) = self.hand.iter().position(|&n| n == 11){
            //an 11 exists, does it make the user go over?
            if sum > 21{
                //if so change the vector value to 1 and recalculate the sum
                self.hand[index] = 1;
                let sum: i32 = self.hand.iter().sum();
            } 
        }
        return sum 
    }

}





