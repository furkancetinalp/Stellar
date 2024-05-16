#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

//Bu projeyi oluşturmak için önceki proje olan hello_word dizininden şu komutu çalıştırdık:
// soroban contract init ./ --with-example increment
//yukarıdaki kod bloğu bir üst dizine çıkarak paralel yeni bir proje oluşturuyor


//Symbol türü max 32 karaktere sahip bir string'i temsil ediyor fakat kısıtı (a-zA-Z),numbers (0-9) ve _ sembolünden ibarettir
const COUNTER: Symbol = symbol_short!("COUNTER"); //symbol_short bir makrodur ve bu durumu 9 karakterle sınırlandırıyor

#[contract]
pub struct IncrementContract;//yine benzer şekilde struct oluşturuldu

#[contractimpl]
impl IncrementContract {
    pub fn increment(env: Env) -> u32 {
        // env.storage() => kontrattaki veriye ulaşmak ve değişiklik yapmak için kullanılıyor
        //get(&COUNTER) => ilgili anahtara ait değeri getirir (burada anahtar &COUNTER yani key-value ilişkisi)
        //unwrap_or(0) => eğer değer yoksa varsayılan olarak 0 getir komutu
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // Bu değişken veri erişimi için kullanıldı (Get)
        log!(&env, "count: {}", count);

        // count değerini artırma işlemi
        count += 1;

        // yukarıdaki get fonksiyonundan farklı olarak, ilgili yeni veriyi güncellemek için set metodu kullanıldı.
        env.storage().instance().set(&COUNTER, &count); //Bu fonksiyon veri manipülasyonu için kullanıldı (Update)

        // The contract instance will be bumped to have a lifetime of at least 100 ledgers if the current expiration lifetime at most 50.
        // If the lifetime is already more than 100 ledgers, this is a no-op. Otherwise,
        // the lifetime is extended to 100 ledgers. This lifetime bump includes the contract
        // instance itself and all entries in storage().instance(), i.e, COUNTER.

        //ttl kavramı kontrat veri depolama ile alakalı
        //Time To Live (TTL) kavramı, ilgili kontratın genişlemesi için kulanılan parametrelerdir. Bunlar üç tanedir:
        //Temporary (tek seferlik veya geçici bilgiler için) , 
        //Instance (paylaşımlı işlemler için admin accounts, metadata), 
        //Persistent (kalıcı veriler için örneğin balances)

        env.storage().instance().extend_ttl(50, 100);

        // count değişkeninin fonksiyondan dönmesi
        count
    }
}

mod test;
