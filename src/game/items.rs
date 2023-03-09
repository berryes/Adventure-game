use super::enemy::Faction;

#[derive(Debug,Clone,PartialEq)]
pub enum Bonus {
    Zero,
    Has{
        faction: Faction,
        amount: f64
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum Effect {
    Heal,
    Protection,
    Attack
}

#[derive(Debug,Clone,PartialEq)]

pub struct Potion {
    pub effect: Effect, // attack
    pub amount: f64, //3
    pub one_use: bool // true
}
#[derive(Debug,Clone,PartialEq)]

pub struct Sword {
    
    pub name: String,

    pub normal: f64,
    
    pub bonus: Bonus,

}

#[derive(Debug,Clone,PartialEq)]
pub struct Armour {
    
    pub name: String,

    pub normal: f64, // protection

    pub bonus: Bonus, // aditional protection

}



