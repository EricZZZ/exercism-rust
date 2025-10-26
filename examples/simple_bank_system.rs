fn main() {}
struct Bank {
    balance: Vec<i64>,
}
impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Bank { balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if account1 < 1 || account2 < 1 {
            return false;
        }
        let i1 = (account1 - 1) as usize;
        let i2 = (account2 - 1) as usize;
        if i1 >= self.balance.len() || i2 >= self.balance.len() {
            return false;
        }
        if self.balance[i1] < money {
            return false;
        }
        self.balance[i1] -= money;
        self.balance[i2] += money;
        true
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if account < 1 {
            return false;
        }
        let idx = (account - 1) as usize;
        if idx >= self.balance.len() {
            return false;
        }
        self.balance[idx] += money;
        true
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if account < 1 {
            return false;
        }
        let idx = (account - 1) as usize;
        if idx >= self.balance.len() {
            return false;
        }
        if self.balance[idx] >= money {
            self.balance[idx] -= money;
            true
        } else {
            false
        }
    }
}
