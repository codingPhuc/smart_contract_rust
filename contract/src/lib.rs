use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env , near_bindgen , AccountId , Balance}; 
use near_sdk::PanicOnDefault  ; 

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Product {
  pub id  : String ,  
  pub name  : String  , 
  pub desc : String  ,  
  pub price : Balance  , 
  pub owner  : AccountId  , 
}


#[near_bindgen] 
#[derive(PanicOnDefault, BorshDeserialize  , BorshSerialize)]
pub struct Contract  
{
  pub owner_id : AccountId  , 
  pub total-products : u32 , 
  pub products : UnorderedMap<u32 , Product >  , 
  pub product_for-owner  : UnorderedMap<AccountId , Product >  , 
}
// Implement the contract structure
#[near_bindgen]
impl Contract {
  #[init]
  pub fn new() -> Self  
  {
    Self{owner_id : env::signer_account_id() , products : UnorderedMap::new(b"products".to_vec())}
  }
  pub fn add_product(&mut self , id : String  , name :  String  , desc  : String , price : Balance )
   {
    let product = Product{id, name  , desc  , price  ,owner:env::signer_account_id()} ; 
    self.products.insert(&product.id , &product )  ; 

   } 
   pub fn get_all_products(&self) -> Vec<Product>  
   {
    let mut all_product  = Vec::new()  ; 
    for i in self.products.inter() 
    {
      all_products
    }

   }
}


// near call dev-1689438118495-87455530988477 add_product '{"id":"id me" , "name" : "name me"  , "desc"  : "no to me"  , "price" : 1 } ' --account-id konodioda2411.testnet