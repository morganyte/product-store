#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data untuk menyimpan produk
#[contracttype]
#[derive(Clone, Debug)]
pub struct Product {
    id: u64,
    name: String,
    price: String,
}

// Storage key untuk data produk
const PRODUCT_DATA: Symbol = symbol_short!("PROD_DATA");

#[contract]
pub struct ProductContract;

#[contractimpl]
impl ProductContract {

    pub fn get_products(env: Env) -> Vec<Product> {
        // 1. ambil data produk dari storage
        return env.storage().instance().get(&PRODUCT_DATA).unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk menambahkan produk baru
    pub fn create_product(env: Env, name: String, price: String) -> String {
        // 1. ambil data produk dari storage
        let mut products: Vec<Product> = env.storage().instance().get(&PRODUCT_DATA).unwrap_or(Vec::new(&env));
        
        // 2. Buat object produk baru
        let product = Product {
            id: env.prng().gen::<u64>(),
            name: name,
            price: price,
        };
        
        // 3. tambahkan produk ke list
        products.push_back(product);
        
        // 4. simpan ke storage
        env.storage().instance().set(&PRODUCT_DATA, &products);
        
        return String::from_str(&env, "Produk berhasil ditambahkan");
    }

    // Fungsi untuk menghapus produk berdasarkan id
    pub fn delete_product(env: Env, id: u64) -> String {
        // 1. ambil data produk dari storage 
        let mut products: Vec<Product> = env.storage().instance().get(&PRODUCT_DATA).unwrap_or(Vec::new(&env));

        // 2. cari index produk yang akan dihapus
        for i in 0..products.len() {
            if products.get(i).unwrap().id == id {
                products.remove(i);

                env.storage().instance().set(&PRODUCT_DATA, &products);
                return String::from_str(&env, "Berhasil hapus produk");
            }
        }

        return String::from_str(&env, "Produk tidak ditemukan")
    }
}

mod test;