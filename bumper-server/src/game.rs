use bumper_core::{Car, CarView};
use serde::{Serialize, Deserialize};

use std::{net::SocketAddr, ops::Deref};
use core::hash::Hash;
use std::sync::{Arc, Mutex};


#[cfg(not(feature = "hashbrown"))]
use std::collections::HashMap;
#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;

pub trait Id: Hash + Eq + Clone + Send + Sync + std::fmt::Debug + Serialize {}

impl Id for SocketAddr {}


#[derive(Default, Debug, Clone)]
pub struct BumperCars<I> 
where
    I: Id
{
    pub players: Arc<Mutex<HashMap<I, Player<I>>>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player<I> 
where
    I: Id
{
    pub id: I,
    pub car: Car
}

impl<I> Player<I> 
where
    I: Id
{
    pub fn new(id: I, car: Car) -> Self {
        Player { id, car }
    }
}

impl<I> Deref for Player<I> 
where
    I: Id
{
    type Target = Car;

    fn deref(&self) -> &Self::Target {
        &self.car
    }
}

impl<I> BumperCars<I> 
where
    I: Id
{
    pub fn new() -> Self {
        BumperCars {
            players: Arc::new(Mutex::new(HashMap::new())),
        }
    }

}

pub trait Game<I> 
where
    I: Id
{
    type Player;
    type PlayerMutation;
    fn get_player(&self, id: I) -> Self::Player;
    fn add_player(&self, id: I, player: Self::Player);
    fn remove_player(&self, id: I) -> Option<Self::Player>;
    fn update_player(&self, id: I, changed_state: Self::PlayerMutation);
    fn send_game_state_to(&self, id: I) -> String;
    fn send_player_state_to(&self, id: I) -> Option<String>;
    fn create_player(&self, id: I) -> Self::Player;
}


impl<I> Game<I> for BumperCars<I> 
where
    I: Id
{
    type Player = Player<I>;
    type PlayerMutation = CarView;
    fn get_player(&self, id: I) -> Self::Player {
        let players = self.players.lock().expect("Couldn't lock players.");

        players
        .get(&id)
        .unwrap_or_else(|| panic!("Couldn't get player: {:#?} from game state.", id))
        .clone()
        
    }
    fn add_player(&self, id: I, player: Self::Player) {
        let mut players = self.players.lock().expect("Couldn't lock players.");
        players.insert(id, player);

    }
    fn remove_player(&self, id: I) -> Option<Self::Player> {
        let mut players = self.players.lock().expect("Couldn't lock players.");
        players.remove(&id)
    }

    fn update_player(&self, id: I, player: Self::PlayerMutation) {
        let mut players = self.players.lock().expect("Couldn't lock players.");
        players
        .entry(id)
        .and_modify(|mut v| {
            v.car.x = player.x;
            v.car.y = player.y;
            v.car.control.forward = player.forward;
            v.car.control.left = player.left;
            v.car.control.right = player.right;
            v.car.control.reverse = player.reverse;
            v.car.config = player.config;
        });
    }

    fn send_game_state_to(&self, id: I) -> String {
        let players = self.players.lock().expect("Couldn't lock players to send state.");
        let data = players
        .iter()
        .filter(|(player_id, _)| player_id != &&id)
        .map(|(_, player)| player)
        .collect::<Vec<_>>();
        serde_json::to_string(&data).expect("Couldn't serialize players.")
    }

    fn send_player_state_to(&self, id: I) -> Option<String> {
        let players = self.players.lock().expect("Couldn't lock players to send state.");
        Some(
            players
            .get(&id)
            .unwrap()
            .json()
        )
    }

    fn create_player(&self, id: I) -> Self::Player {
        let car = Car::new(100., 100., 60., 80.);
        let player = Player::new(id.clone(), car);
        self.add_player(id, player.clone());
        player
    }
}
