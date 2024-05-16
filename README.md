# Soroban Project

## Project Structure


// PROJE YÜKLENMESİ VE AYARLAMALAR
1. Soroban CLI indirme komutu => cargo install --locked soroban-cli

2. İndirildikten sonra şu komutla sorgu yapılabilir => soroban

3. Testnet ortamı oluşturmak için aşağıdaki kod bloğunu terminalde çalıştırmak gerekiyor:
soroban network add \
  --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"

4. Test ortamında bir kimlik oluşturmak için aşağıdaki komut kullanılıyor => :
soroban keys generate --global herhangi_bir_ornek_isim --network testnet

5. Oluşturulan kimliğin bilgisini sorgulama aşağıdaki kod ile yapılıyor:
soroban keys address herhangi_bir_ornek_isim



// YENİ PROJE OLUŞTURMA
1. Yeni bir proje oluşturmak için şu komut kullanılır:
soroban contract init soroban-hello-world


