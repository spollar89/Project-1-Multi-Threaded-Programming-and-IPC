use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use rand::Rng; // For random transaction in stress testing

/// Represents a bank account with a mutex-protected balance
struct BankAccount {
    balance: Mutex<i32>,
}

impl BankAccount {
    /// Creates a new bank account with an initial balance
    fn new(initial_balance: i32) -> Self {
        BankAccount {
            balance: Mutex::new(initial_balance),
        }
    }

    /// Deposits money into the account (protected by a mutex)
    fn deposit(&self, amount: i32) {
        let mut balance = self.balance.lock().unwrap(); // Lock the balance
        *balance += amount;
        println!("Deposited: {}, New Balance: {}", amount, *balance);
    }

    /// Withdraws money from the account (protected by a mutex)
    fn withdraw(&self, amount: i32) {
        let mut balance = self.balance.lock().unwrap(); // Lock the balance
        if *balance >= amount {
            *balance -= amount;
            println!("Withdrawn: {}, New Balance: {}", amount, *balance);
        } else {
            println!("Withdrawal failed: Insufficient funds.");
        }
    }

    /// Safe transfers between two accounts with timeout to prevent deadlock
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

/// Phase 1: Basic Thread Operations
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

/// Phase 2: Resource Protection (Testing Mutex Locks)
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

/// Phase 3: Deadlock Creation
ffn phase_3(account1: Arc<BankAccount>, account2: Arc<BankAccount>) {
    println!("=== Phase 3: Deadlock Creation ===");
    let handles = vec![
        thread::spawn({
            let account1 = Arc::clone(&account1);
            let account2 = Arc::clone(&account2);
            move || {
                let _lock1 = account1.balance.lock().unwrap();
                thread::sleep(Duration::from_millis(100)); // Simulate delay
                let _lock2 = account2.balance.lock().unwrap();
                println!("Locked both accounts successfully (No deadlock due to ordering).\n");
            }
        }),
        thread::spawn({
            let account1 = Arc::clone(&account1);
            let account2 = Arc::clone(&account2);
            move || {
                let _lock2 = account2.balance.lock().unwrap();
                thread::sleep(Duration::from_millis(100));
                let _lock1 = account1.balance.lock().unwrap();
                println!("Locked both accounts successfully.\n");
            }
        }),
    ];
    for handle in handles {
        handle.join().unwrap();
    }
    println!();
}

/// Phase 4: Deadlock Resolution
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

/// **Stress Testing: Simulate high-load scenario with multiple transactions**
fn stress_test(account: Arc<BankAccount>){
    println!("+++ Stress Testing: High Load Simulation ===");
    let mut rng = rand::thread_rng();
    let handles: Vec<_> = (0..50).map(|_| {
        let account = Arc::clone(&account);
        thread::spawn(move||{
            let amount = rng.gen_range(10..100);
            if rng.gen_bool(0.5){
                account.deposit(amount);
            } else{
                account.withdraw(amount);
            }
        })
    }) .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Stress Test Completed.\n");
}

fn main() {
    let account1 = Arc::new(BankAccount::new(100));
    let account2 = Arc::new(BankAccount::new(100));

    phase_1(Arc::clone(&account1)); // Run Phase 1
    phase_2(Arc::clone(&account1)); // Run Phase 2
    phase_3(Arc::clone(&account1), Arc::clone(&account2)); // Run Phase 3
    phase_4(Arc::clone(&account1), Arc::clone(&account2)); // Run Phase 4
}