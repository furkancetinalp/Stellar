#![no_std] //tüm kontratların bu nitelikle başlaması tavsiye ediliyor
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec}; //ilgili type ve macro'ların importu

//Rusttaki birçok tür soroban'da kabul edilmiyor. 
//Kabul edilen bazı türler:
//Vec, Map, Bytes, BytesN, Symbol
//Primitive types => u128, i128, u64, i64, u32, i32 , bool

// !! Float soroban'da kabul edilmiyor

#[contract]//bu nitelik ile hangi struct'ın implemente edileceği bildirilir
pub struct HelloContract;

#[contractimpl] //ilgili kontratın implementasyonu için bu nitelik eklenir
impl HelloContract { //Fonksiyonlar maksimum 32 karakter uzunluğunda olmalı

    //ilk parametrenin Env ile başlaması tavsiye ediliyor. Bu, soroban ortamında bu bileşenlerin kopyasına erişiminin sağlanmasına olanak tanıyor
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> { 
        vec![&env, symbol_short!("Hello"), to]
    }
}

mod test; //test.rs dosyasındaki modülü çağırıyor. Orada yukarıdaki işlemlerin testi yapılıyor

//unit testin çalıştırılması => cargo test

//kontratın derlenmesi (build) => soroban contract build



//Deploy süreci:

/*
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
  --source identity_ismi \
  --network testnet
*/

//identity_isminden kasıt, soroban yüklerken aynı zamanda bir identity oluşturuluyordu. Örnek komut:
//soroban keys generate --global furkan --network testnet


//Deploy kodunu çalıştırdıktan sonra ilgili contract'ın id'si geliyor: Örnek => CACDYF3CYMJEJTIVFESQYZTN67GO2R5D5IUABTCUG3HXQSRXCSOROBAN

//Daha sonra bu contract id'si ile deploy sürecini tekrar tetikliyoruz:
/*
soroban contract invoke \
  --id CACDYF3CYMJEJTIVFESQYZTN67GO2R5D5IUABTCUG3HXQSRXCSOROBAN \
  --source furkan \
  --network testnet \
  -- \
  hello \
  --to RPC
*/

//Program çıktı olarak şu yanıtı veriyor=> ["Hello","RPC"]