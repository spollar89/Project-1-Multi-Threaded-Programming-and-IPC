use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct BankAccount {
    balance: Mutex<i32>,
}

impl BankAccount {
    fn new(initial_balance: i32) -> Self {
        BankAccount {
            balance: Mutex::new(initial_balance),
        }
    }

    fn deposit(&self, amount: i32) {
        let mut balance = self.balance.lock().unwrap();
        *balance += amount;
        println!("Deposited: {}, New Balance: {}", amount, *balance);
    }

    fn withdraw(&self, amount: i32) {
        let mut balance = self.balance.lock().unwrap();
        if *balance >= amount {
            *balance -= amount;
            println!("Withdrawn: {}, New Balance: {}", amount, *balance);
        } else {
            println!("Withdrawal failed: Insufficient funds.");
        }
    }

    fn transfer_safe(&self, target: &BankAccount, amount: i32) {
        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(500) { // 500ms timeout
            if let Ok(mut first) = self.balance.try_lock() {
                if let Ok(mut second) = target.balance.try_lock() {
                    if *first >= amount {
                        *first -= amount;
                        *second += amount;
                        println!("Transferred: {}", amount);
                    } else {
                        println!("Transfer failed: Insufficient funds.");
                    }
                    return;
                }
            }
            thread::sleep(Duration::from_millis(10)); // Avoid busy waiting
        }
        println!("Transfer failed due to timeout.");
    }
}

fn phase_1(account: Arc<BankAccount>) {
    println!("=== Phase 1: Basic Thread Operations ===");
    let handles: Vec<_> = (0..5).map(|i| {
        let account = Arc::clone(&account);
        thread::spawn(move || {
            if i % 2 == 0 {
                account.deposit(50);
            } else {
                account.withdraw(30);
            }
        })
    }).collect();
    for handle in handles {
        handle.join().unwrap();
    }
    println!();
}

fn phase_2(account: Arc<BankAccount>) {
    println!("=== Phase 2: Resource Protection ===");

    let handles: Vec<_> = (0..5).map(|i| {
        let account = Arc::clone(&account);
        thread::spawn(move || {
            let mut balance = account.balance.lock().unwrap();
            if i % 2 == 0 {
                *balance += 50;
                println!("(Protected) Deposited: 50, New Balance: {}", *balance);
            } else {
                if *balance >= 30 {
                    *balance -= 30;
                    println!("(Protected) Withdrawn: 30, New Balance: {}", *balance);
                } else {
                    println!("(Protected) Withdrawal failed: Insufficient funds.");
                }
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
    println!();
}


fn phase_3(account1: Arc<BankAccount>, account2: Arc<BankAccount>) {
    println!("=== Phase 3: Deadlock Creation ===");
    
    let handles = vec![
        thread::spawn({
            let account1 = Arc::clone(&account1);
            let account2 = Arc::clone(&account2);
            move || {
                let start = Instant::now();
                while start.elapsed() < Duration::from_millis(500) { // Timeout
                    if let Ok(_self_balance) = account1.balance.try_lock() {
                        println!("Locked source account 1");
                        thread::sleep(Duration::from_millis(100));
                        if let Ok(_target_balance) = account2.balance.try_lock() {
                            println!("Locked target account 2");
                            return;
                        }
                    }
                    thread::sleep(Duration::from_millis(10)); // Avoid busy waiting
                }
                println!("Thread 1: Deadlock detected, exiting...");
            }
        }),
        thread::spawn({
            let account1 = Arc::clone(&account1);
            let account2 = Arc::clone(&account2);
            move || {
                let start = Instant::now();
                while start.elapsed() < Duration::from_millis(500) { // Timeout
                    if let Ok(_self_balance) = account2.balance.try_lock() {
                        println!("Locked source account 2");
                        thread::sleep(Duration::from_millis(100));
                        if let Ok(_target_balance) = account1.balance.try_lock() {
                            println!("Locked target account 1");
                            return;
                        }
                    }
                    thread::sleep(Duration::from_millis(10)); // Avoid busy waiting
                }
                println!("Thread 2: Deadlock detected, exiting...");
            }
        }),
    ];

    for handle in handles {
        handle.join().unwrap();
    }
    println!();
}


fn phase_4(account1: Arc<BankAccount>, account2: Arc<BankAccount>) {
    println!("=== Phase 4: Deadlock Resolution ===");
    let handles = vec![
        thread::spawn({
            let account1 = Arc::clone(&account1);
            let account2 = Arc::clone(&account2);
            move || {
                account1.transfer_safe(&account2, 50);
            }
        }),
        thread::spawn({
            let account1 = Arc::clone(&account1);
            let account2 = Arc::clone(&account2);
            move || {
                account2.transfer_safe(&account1, 30);
            }
        }),
    ];
    for handle in handles {
        handle.join().unwrap();
    }
    println!();
}

fn main() {
    let account1 = Arc::new(BankAccount::new(100));
    let account2 = Arc::new(BankAccount::new(100));

    phase_1(Arc::clone(&account1)); // Run Phase 1
    phase_2(Arc::clone(&account1)); // Run Phase 2
    phase_3(Arc::clone(&account1), Arc::clone(&account2)); // Run Phase 3
    phase_4(Arc::clone(&account1), Arc::clone(&account2)); // Run Phase 4
}