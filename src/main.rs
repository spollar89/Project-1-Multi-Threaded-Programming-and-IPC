use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};


fn main() {
   // Create two shared resources protected by Mutexes
   let resource1 = Arc::new(Mutex::new(0));
   let resource2 = Arc::new(Mutex::new(0));


   // Clone the Arcs to share ownership with the threads
   let resource1_clone = Arc::clone(&resource1);
   let resource2_clone = Arc::clone(&resource2);


   // Thread 1: Locks resources in a specific order
   let handle1 = thread::spawn(move || {
       let start = Instant::now();
       let max_retries = 3; // Maximum number of retries
       let mut retries = 0;


       loop {
           // Try to acquire resource1 with a timeout
           let timeout = Duration::from_secs(2);
           let mut lock1_acquired = false;
           while start.elapsed() < timeout {
               if let Ok(_lock1) = resource1_clone.try_lock() {
                   println!("Thread 1 acquired resource1");
                   lock1_acquired = true;
                   break;
               }
               thread::sleep(Duration::from_millis(100)); // Sleep for a short duration
           }


           if lock1_acquired {
               // Try to acquire resource2 with a timeout
               let mut lock2_acquired = false;
               while start.elapsed() < timeout {
                   if let Ok(_lock2) = resource2_clone.try_lock() {
                       println!("Thread 1 acquired resource2");
                       lock2_acquired = true;
                       break;
                   }
                   thread::sleep(Duration::from_millis(100)); // Sleep for a short duration
               }


               if lock2_acquired {
                   // Simulate some work
                   thread::sleep(Duration::from_secs(1));
                   println!("Thread 1 finished");
                   break; // Exit the loop after successful completion
               } else {
                   println!("Thread 1 failed to acquire resource2, releasing resource1 and retrying...");
               }
           } else {
               println!("Thread 1 failed to acquire resource1, retrying...");
           }


           retries += 1;
           if retries >= max_retries {
               println!("Thread 1 reached maximum retries, exiting...");
               break;
           }


           // Wait before retrying
           thread::sleep(Duration::from_secs(1));
       }
   });


   // Thread 2: Locks resources in the same order as Thread 1
   let handle2 = thread::spawn(move || {
       let start = Instant::now();
       let max_retries = 3; // Maximum number of retries
       let mut retries = 0;


       loop {
           // Try to acquire resource1 with a timeout
           let timeout = Duration::from_secs(2);
           let mut lock1_acquired = false;
           while start.elapsed() < timeout {
               if let Ok(_lock1) = resource1.try_lock() {
                   println!("Thread 2 acquired resource1");
                   lock1_acquired = true;
                   break;
               }
               thread::sleep(Duration::from_millis(100)); // Sleep for a short duration
           }


           if lock1_acquired {
               // Try to acquire resource2 with a timeout
               let mut lock2_acquired = false;
               while start.elapsed() < timeout {
                   if let Ok(_lock2) = resource2.try_lock() {
                       println!("Thread 2 acquired resource2");
                       lock2_acquired = true;
                       break;
                   }
                   thread::sleep(Duration::from_millis(100)); // Sleep for a short duration
               }


               if lock2_acquired {
                   // Simulate some work
                   thread::sleep(Duration::from_secs(1));
                   println!("Thread 2 finished");
                   break; // Exit the loop after successful completion
               } else {
                   println!("Thread 2 failed to acquire resource2, releasing resource1 and retrying...");
               }
           } else {
               println!("Thread 2 failed to acquire resource1, retrying...");
           }


           retries += 1;
           if retries >= max_retries {
               println!("Thread 2 reached maximum retries, exiting...");
               break;
           }


           // Wait before retrying
           thread::sleep(Duration::from_secs(1));
       }
   });


   // Wait for both threads to finish
   handle1.join().unwrap();
   handle2.join().unwrap();


   println!("All threads finished");
}