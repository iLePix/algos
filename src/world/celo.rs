
pub struct Celo {
    pos: Vec2,
    size: u8,
    food: f32,
    sensors: u8,
}

impl Celo {
    pub fn new(pos: Vec2, size: u8, sensors: u8) -> Self {
        Self {pos, size, food: u8::MAX as f32, sensors}
    }
    
    pub fn update(mut self) {
        self.food =- self.sensors * 0.1;
    }

    pub fn eat(mut self, food_amount: f32) {
        self.food += food_amount;
    }
}


//creating a HHMM -> Hierarchial Hidden Markov Model 

//Neural Network


//https://towardsdatascience.com/hidden-markov-model-implemented-from-scratch-72865bda430e





