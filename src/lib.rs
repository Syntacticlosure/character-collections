use std::collections::HashMap;
use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::{
    env,
    ext_contract,
    near_bindgen,
    Promise,
};
use serde_json::json;
use serde::{Serialize,Deserialize};
use std::mem::transmute;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! require_owner_of{
    ($self:ident,$token:expr) => {
        assert!(env::signer_account_id() == $self.get_character_owner($token),
         "require_owner_of")
         }
}
#[derive(Clone,Default,BorshSerialize,BorshDeserialize,Serialize,Deserialize)]
pub struct Character {
    name: String,
    level:u32,
    attack:u32,
    defense:u32
}

#[near_bindgen]
#[derive(Default,BorshSerialize,BorshDeserialize)]
pub struct CharacterCollections{
    characters : Vec<Character>,
    characters_owner : Vec<String>,
    characters_by_owner : HashMap<String,Vec<u64>>,
    wallet : HashMap<String,u64>
}
#[near_bindgen]
impl CharacterCollections{
    pub fn get_character(&self,token:u64) -> String{
        serde_json::to_string(&self.characters[token as usize]).unwrap()
    }
    pub fn create_random_character(&mut self)->u64{
            let balance = self.wallet.entry(env::signer_account_id()).or_insert(10000);
        assert!(*balance>=10,"Not enough money to level up");
        *balance-=10;
        let level :u32 =1;
        let attack:u32 = get_random(100,&[1]) as u32;
        let defense:u32 = get_random(100,&[2]) as u32;
        let names = ["Misaka Mikoto","Donald Trump","Sizumiya Haruhi",
        "Furukawa Nagisa","Hitler","Hibino Mirai","Altria Pendragon",
        "Kaname Madoka"];
        let name = names[get_random(names.len() as u64,&[3]) as usize]
            .to_owned();
        let character = Character {name,level,attack,defense};
        let idx : u64 =(self.characters.len()) as u64;
        self.characters.push(character);
        self.characters_owner.push(env::signer_account_id());
        let mut owned_characters=self.characters_by_owner.entry(env::signer_account_id()).or_insert(Vec::new());
        owned_characters.push(idx);
        idx
    }
    pub fn get_characters_by_owner(&self,owner:String)->String{
        let characters = self.characters_by_owner.get(&owner);
        match characters {
            Some(c) => serde_json::to_string(c).unwrap(),
            None => "[]".to_owned()
        }

    }
    pub fn get_character_owner(&self,token:u64) -> String{
        self.characters_owner[token as usize].clone()
    }
    pub fn transfer(&mut self,token:u64,target:String) {
        require_owner_of!(self,token);
        let owner : String= env::signer_account_id();
        self.characters_by_owner.entry(target.clone()).or_default().push(token);
        let owner_chs = self.characters_by_owner.entry(owner).or_default();
        *owner_chs = (*owner_chs).iter().filter(|&&x| x!= token)
            .map(|x| x.clone()).collect();
        self.characters_owner[token as usize] = target;
    }

    pub fn get_balance(&self,owner:String) -> String{
        self.wallet.get(&owner).unwrap_or(&10000).to_string()
    }
    pub fn level_up_character(&mut self,token:u64) {
        require_owner_of!(self,token);
        let balance = self.wallet.entry(env::signer_account_id()).or_insert(10000);
        assert!(*balance>=10,"Not enough money to level up");
        let ch = self.characters.get_mut(token as usize).unwrap();
        ch.level+=1;
        ch.attack += get_random(10,&[1,2]) as u32;
        ch.defense += get_random(10,&[2,3]) as u32;
        *balance-=10;
    }
    pub fn get_characters_by_owner_detail(&self,owner:String) -> String{
        let chs = self.characters_by_owner.get(&owner).unwrap();
        let js : serde_json::value::Value= chs.iter()
            .map(|x|{
                let character :&Character = self.characters.get(*x as usize).unwrap();
                json!({
                "name" : character.name.clone(),
                "attack" : character.attack,
                "defense" : character.defense,
                "level" : character.level,
                "token" : *x})
            }).collect();
        js.to_string()
    }
}

fn get_random(upper_bound:u64,extra_seeds:&[u8]) -> u64 {
    let vectored : [u8;8] = unsafe {
        transmute(env::block_timestamp())
    };
    let mut seeds : Vec<u8> = Vec::new();
    seeds.extend(&vectored);
    seeds.extend(extra_seeds);
    let hashed : Vec<u8> = env::sha256(&seeds);
    let mut ret :u64=0;
    for &i in hashed.iter(){
        ret+=i as u64;
    }
    ret % upper_bound
}