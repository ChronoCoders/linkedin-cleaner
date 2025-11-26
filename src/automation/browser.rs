use crate::models::{Automation, FilterResult, Person};
use anyhow::Result;
use headless_chrome::{Browser, LaunchOptions};
use rand::Rng;
use std::time::Duration;
use std::thread;

pub struct BrowserAutomation {
    config: Automation,
}

impl BrowserAutomation {
    pub fn new(config: Automation) -> Self {
        Self { config }
    }

    pub fn remove_connections(&self, results: &[FilterResult], email: &str, password: &str) -> Result<()> {
        let to_remove: Vec<_> = results.iter().filter(|r| r.should_remove).collect();
        
        println!("\n=== Removal Process ===");
        println!("Total to remove: {}", to_remove.len());
        println!("Daily limit: {}", self.config.daily_limit);
        println!("Dry run: {}", self.config.dry_run);
        
        if self.config.dry_run {
            println!("\nDRY RUN MODE - No actual removals will be performed");
            return Ok(());
        }
        
        let browser = Browser::new(LaunchOptions {
            headless: false,
            ..Default::default()
        })?;
        
        let tab = browser.new_tab()?;
        
        println!("\nLogging into LinkedIn...");
        self.login(&tab, email, password)?;
        
        let mut removed = 0;
        let limit = to_remove.len().min(self.config.daily_limit);
        
        for (i, result) in to_remove.iter().take(limit).enumerate() {
            let Person::Connection(conn) = &result.person;
            
            println!("\n[{}/{}] Removing: {} {} - {}", 
                i + 1, limit, conn.first_name, conn.last_name, conn.position);
            
            match self.remove_single_connection(&tab, &conn.url) {
                Ok(_) => {
                    println!("✓ Successfully removed");
                    removed += 1;
                }
                Err(e) => {
                    println!("✗ Failed: {}", e);
                }
            }
            
            self.random_delay();
            
            if (i + 1) % self.config.batch_size == 0 && (i + 1) < limit {
                println!("\n--- Batch complete. Pausing for {} seconds ---", 
                    self.config.pause_between_batches_ms / 1000);
                thread::sleep(Duration::from_millis(self.config.pause_between_batches_ms));
            }
        }
        
        println!("\n=== Summary ===");
        println!("Removed: {}/{}", removed, limit);
        
        Ok(())
    }

    fn login(&self, tab: &headless_chrome::Tab, email: &str, password: &str) -> Result<()> {
        tab.navigate_to("https://www.linkedin.com/login")?;
        tab.wait_for_element("input#username")?;
        
        let username_elem = tab.find_element("input#username")?;
        username_elem.click()?;
        username_elem.type_into(email)?;
        
        let password_elem = tab.find_element("input#password")?;
        password_elem.click()?;
        password_elem.type_into(password)?;
        
        tab.find_element("button[type='submit']")?.click()?;
        
        thread::sleep(Duration::from_secs(5));
        
        println!("✓ Login successful");
        Ok(())
    }

    fn remove_single_connection(&self, tab: &headless_chrome::Tab, profile_url: &str) -> Result<()> {
        if profile_url.trim().is_empty() {
            return Err(anyhow::anyhow!("Empty URL - skipping"));
        }
        
        tab.navigate_to(profile_url)?;
        thread::sleep(Duration::from_secs(3));
        
        if let Ok(more_button) = tab.find_element("button[aria-label*='More actions']") {
            more_button.click()?;
            thread::sleep(Duration::from_millis(1000));
            
            if let Ok(remove_button) = tab.find_element("div[aria-label*='Remove connection']") {
                remove_button.click()?;
                thread::sleep(Duration::from_millis(1000));
                
                if let Ok(confirm_button) = tab.find_element("button[data-test-dialog-primary-btn]") {
                    confirm_button.click()?;
                    thread::sleep(Duration::from_millis(1000));
                }
            }
        }
        
        Ok(())
    }

    fn random_delay(&self) {
        let mut rng = rand::thread_rng();
        let delay = rng.gen_range(self.config.delay_min_ms..=self.config.delay_max_ms);
        thread::sleep(Duration::from_millis(delay));
    }
}