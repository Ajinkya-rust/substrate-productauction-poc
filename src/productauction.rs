use support::{decl_storage, decl_module,dispatch::Result, StorageValue};
use system::ensure_signed;
use runtime_primitives::traits::{As, Hash};
use parity_codec::{Encode, Decode};



#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Product<Hash, Balance> {
    // structure for products.
             id:Hash,
             name:Hash,
             price:Balance,
             gen:u64,
             //quantity:u32,
}
pub trait Trait: balances::Trait {}

 decl_storage!{
    trait Store for Module<T: Trait> as ProductStorage 
    {
        Products get(new_product): map T::Hash => Product <T::Hash, T::Balance>;

        Nonce: u64;
    }
}
 decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin 
    {

        // This is function for product creation
        fn create_product(origin)->Result{
            let sender = ensure_signed(origin)?;
            
            let nonce = <Nonce<T>>::get();
            let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
                .using_encoded(<T as system::Trait>::Hashing::hash);

           
            let new_product = Product {
                id: random_hash,
                name: random_hash,
                price: <T::Balance as As<u64>>::sa(0),
                gen: 0,
            };

            <Products<T>>::insert(random_hash, new_product);
           
            <Nonce<T>>::mutate(|n| *n += 1);


            Ok(())
        }
    }
}
