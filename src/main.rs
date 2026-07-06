use std::collections::HashMap; // Если планируешь вводить данные с клавиатуры
#[derive(Clone, Copy)]
enum Side {
    Buy,
    Sell,
}
struct User {
    id: u64,
    name: String,
    solana: f64,
    usdt: f64,
}

enum Asset {
    Sol,
    Usdt,
}

struct Exchange {
    users: HashMap<u64, User>,
    buy_orders: Vec<Order>,
    sell_orders: Vec<Order>,
    next_user_id: u64,
    next_order_id: u64,
}
#[derive(Debug)]
enum ExchangeErr {
    UserNotFound,
    InsufficientUSDT,
    Insufficientsolana,
}
impl Exchange {
    fn new() -> Exchange {
        Exchange {
            users: HashMap::new(),
            buy_orders: vec![],
            sell_orders: vec![],
            next_user_id: 1,
            next_order_id: 1,
        }
    }
    fn register_us(&mut self, name: String) -> u64 {
        let id = self.next_user_id;
        let new_user = User::new(id, name);
        self.users.insert(id, new_user);
        self.next_user_id += 1;
        id
    }
    fn deposit(&mut self, id: u64, asset: Asset, amount: f64) -> Result<(), ExchangeErr> {
        match self.users.get_mut(&id) {
            Some(user) => {
                match asset {
                    Asset::Sol => user.solana += amount,
                    Asset::Usdt => user.usdt += amount,
                }
                Ok(())
            }
            None => Err(ExchangeErr::UserNotFound),
        }
    }
    fn add_order(
        &mut self,
        user_id: u64,
        side: Side,
        price: f64,
        amount: f64,
    ) -> Result<u64, ExchangeErr> {
        // ШАГ 1: Только лезем к юзеру и списываем бабки
        match self.users.get_mut(&user_id) {
            Some(user) => match side {
                Side::Buy => {
                    let total = price * amount;
                    if user.usdt < total {
                        return Err(ExchangeErr::InsufficientUSDT);
                    }
                    user.usdt -= total;
                }
                Side::Sell => {
                    if user.solana < amount {
                        return Err(ExchangeErr::Insufficientsolana);
                    }
                    user.solana -= amount;
                }
            },
            None => return Err(ExchangeErr::UserNotFound),
        }

        // ШАГ 2: Код создания ордера пишется ВСЕГО ОДИН РАЗ на всю функцию!
        let order_id = self.next_order_id;
        let new_order = Order {
            id: order_id,
            user_id,
            side, // Мы просто прокидываем side из аргументов функции!
            price,
            amount,
        };

        // ШАГ 3: Быстро решаем, в какую стопку положить
        match side {
            Side::Buy => self.buy_orders.push(new_order),
            Side::Sell => self.sell_orders.push(new_order),
        }

        // Двигаем счетчик и возвращаем ID
        self.next_order_id += 1;
        Ok(order_id)
    }
}

impl User {
    fn new(id: u64, name: String) -> User {
        User {
            id,
            name,
            solana: 0.0,
            usdt: 0.0,
        }
    }
}

struct Order {
    id: u64,
    user_id: u64,
    side: Side,
    price: f64,
    amount: f64,
}

fn main() {
    fn main() {
        // 1. Создаем биржу
        let mut exchange = Exchange::new();
        println!("Биржа успешно запущена!");

        // 2. Регистрируем тебя
        let my_id = exchange.register_us("Илья".to_string());
        println!("Пользователь зарегистрирован. Твой ID: {}", my_id);

        // 3. Закидываем тебе 1000 баксов на счет
        let _ = exchange.deposit(my_id, Asset::Usdt, 1000.0);
        println!("Депозит зачислен. Баланс проверен.");

        // 4. Пробуем создать ордер на покупку 5 Solana по цене 140$
        println!("Пробуем купить 5 SOL по 140$ (всего надо 700$)...");

        match exchange.add_order(my_id, Side::Buy, 140.0, 5.0) {
            Ok(order_id) => {
                println!("Успех! Ордер №{} добавлен в стакан покупок.", order_id);
            }
            Err(error) => {
                println!("Что-то пошло не так: {:?}", error);
            }
        }
    }
}
