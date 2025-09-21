use std::collections::{BTreeSet, HashMap};

fn main() {}

/**
 * Your MovieRentingSystem object will be instantiated and called as such:
 * let obj = MovieRentingSystem::new(n, entries);
 * let ret_1: Vec<i32> = obj.search(movie);
 * obj.rent(shop, movie);
 * obj.drop(shop, movie);
 * let ret_4: Vec<Vec<i32>> = obj.report();
 */
struct MovieRentingSystem {
    shop_movie_to_price: HashMap<(i32, i32), i32>, // (shop,movie) -> price
    unrent_movie_to_price_shop: HashMap<i32, BTreeSet<(i32, i32)>>, // movie -> (price,shop)
    rent_movie: BTreeSet<(i32, i32, i32)>,         // (price,shop,movie)
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovieRentingSystem {
    fn new(_n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut shop_movie_to_price: HashMap<(i32, i32), i32> = HashMap::new();
        let mut unrent_movie_to_price_shop: HashMap<i32, BTreeSet<(i32, i32)>> = HashMap::new();
        for v in entries {
            let shop = v[0];
            let movie = v[1];
            let price = v[2];
            shop_movie_to_price.insert((shop, movie), price);
            unrent_movie_to_price_shop
                .entry(movie)
                .or_default()
                .insert((price, shop));
        }
        MovieRentingSystem {
            shop_movie_to_price,
            unrent_movie_to_price_shop,
            rent_movie: BTreeSet::new(),
        }
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        self.unrent_movie_to_price_shop
            .get(&movie)
            .map_or_else(Vec::new, |s| {
                s.iter().take(5).map(|&(_, shop)| shop).collect()
            })
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        let price = self.shop_movie_to_price[&(shop, movie)];
        self.rent_movie.insert((price, shop, movie));
        self.unrent_movie_to_price_shop
            .get_mut(&movie)
            .unwrap()
            .remove(&(price, shop));
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        let price = self.shop_movie_to_price[&(shop, movie)];
        self.rent_movie.remove(&(price, shop, movie));
        self.unrent_movie_to_price_shop
            .get_mut(&movie)
            .unwrap()
            .insert((price, shop));
    }

    fn report(&self) -> Vec<Vec<i32>> {
        self.rent_movie
            .iter()
            .take(5)
            .map(|&(_, shop, movie)| vec![shop, movie])
            .collect()
    }
}
