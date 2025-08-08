use anyhow::{anyhow, Result};
use std::process::Command;
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// GitHub API rate limiting and retry logic
pub struct GitHubApiClient {
    rate_limiter: RateLimiter,
    retry_config: RetryConfig,
}

#[derive(Debug)]
pub struct RateLimiter {
    requests_per_hour: u32,
    last_request: Option<Instant>,
    requests_made: u32,
    window_start: Instant,
}

#[derive(Debug)]
pub struct RetryConfig {
    max_retries: u32,
    base_delay: Duration,
    max_delay: Duration,
}

impl GitHubApiClient {
    pub fn new() -> Self {
        Self {
            rate_limiter: RateLimiter::new(5000), // GitHub's rate limit
            retry_config: RetryConfig {
                max_retries: 3,
                base_delay: Duration::from_secs(1),
                max_delay: Duration::from_secs(60),
            },
        }
    }

    pub async fn execute_gh_command(&mut self, args: &[&str]) -> Result<String> {
        let mut attempt = 0;
        
        loop {
            // Check rate limit before making request
            self.rate_limiter.wait_if_needed().await;
            
            match self.try_gh_command(args).await {
                Ok(output) => {
                    self.rate_limiter.record_request();
                    return Ok(output);
                }
                Err(e) => {
                    attempt += 1;
                    
                    if attempt >= self.retry_config.max_retries {
                        return Err(e);
                    }
                    
                    // Check if this is a rate limit error
                    if self.is_rate_limit_error(&e) {
                        let delay = self.calculate_backoff_delay(attempt);
                        eprintln!("Rate limited, waiting {}s before retry...", delay.as_secs());
                        sleep(delay).await;
                    } else if self.is_retryable_error(&e) {
                        let delay = self.calculate_backoff_delay(attempt);
                        eprintln!("Retryable error, waiting {}s before retry: {}", delay.as_secs(), e);
                        sleep(delay).await;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
    }

    async fn try_gh_command(&self, args: &[&str]) -> Result<String> {
        let output = Command::new("gh")
            .args(args)
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("GitHub CLI command failed: {}", stderr))
        }
    }

    fn is_rate_limit_error(&self, error: &anyhow::Error) -> bool {
        let error_str = error.to_string().to_lowercase();
        error_str.contains("rate limit") || 
        error_str.contains("403") ||
        error_str.contains("api rate limit exceeded")
    }

    fn is_retryable_error(&self, error: &anyhow::Error) -> bool {
        let error_str = error.to_string().to_lowercase();
        error_str.contains("timeout") ||
        error_str.contains("connection") ||
        error_str.contains("502") ||
        error_str.contains("503") ||
        error_str.contains("504")
    }

    fn calculate_backoff_delay(&self, attempt: u32) -> Duration {
        let delay = self.retry_config.base_delay * 2_u32.pow(attempt - 1);
        delay.min(self.retry_config.max_delay)
    }

    pub async fn find_existing_issue(&mut self, title: &str, repo: &str) -> Result<Option<u64>> {
        let search_query = format!("{} in:title", title);
        let args = [
            "issue", "list",
            "--repo", repo,
            "--state", "open",
            "--search", &search_query,
            "--json", "number,title",
            "-q", ".[0].number"
        ];

        let output = self.execute_gh_command(&args).await?;
        let trimmed = output.trim();
        
        if trimmed.is_empty() || trimmed == "null" {
            Ok(None)
        } else {
            Ok(trimmed.parse().ok())
        }
    }

    pub async fn create_issue(
        &mut self,
        title: &str,
        body_file: &str,
        labels: &str,
        repo: &str,
    ) -> Result<u64> {
        let args = [
            "issue", "create",
            "--repo", repo,
            "--title", title,
            "--label", labels,
            "--body-file", body_file
        ];

        let output = self.execute_gh_command(&args).await?;
        
        // Extract issue number from GitHub CLI output (usually a URL)
        if let Some(issue_url) = output.lines().last() {
            if let Some(number_str) = issue_url.split('/').last() {
                return Ok(number_str.parse().unwrap_or(0));
            }
        }
        
        Ok(0)
    }

    pub async fn update_issue(
        &mut self,
        issue_number: u64,
        body_file: &str,
        labels: &str,
        repo: &str,
    ) -> Result<()> {
        let args = [
            "issue", "edit", &issue_number.to_string(),
            "--repo", repo,
            "--body-file", body_file,
            "--add-label", labels
        ];

        self.execute_gh_command(&args).await?;
        Ok(())
    }
}

impl RateLimiter {
    fn new(requests_per_hour: u32) -> Self {
        Self {
            requests_per_hour,
            last_request: None,
            requests_made: 0,
            window_start: Instant::now(),
        }
    }

    async fn wait_if_needed(&mut self) {
        let now = Instant::now();
        
        // Reset window if an hour has passed
        if now.duration_since(self.window_start) >= Duration::from_secs(3600) {
            self.requests_made = 0;
            self.window_start = now;
        }

        // Check if we're approaching the rate limit
        if self.requests_made >= self.requests_per_hour {
            let time_until_reset = Duration::from_secs(3600) - now.duration_since(self.window_start);
            eprintln!("Rate limit reached, waiting {}s until reset", time_until_reset.as_secs());
            sleep(time_until_reset).await;
            
            // Reset after waiting
            self.requests_made = 0;
            self.window_start = Instant::now();
        }

        // Ensure minimum delay between requests (avoid burst)
        if let Some(last) = self.last_request {
            let min_interval = Duration::from_millis(100); // 10 requests per second max
            let elapsed = now.duration_since(last);
            
            if elapsed < min_interval {
                sleep(min_interval - elapsed).await;
            }
        }
    }

    fn record_request(&mut self) {
        self.requests_made += 1;
        self.last_request = Some(Instant::now());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{advance, pause, resume};

    #[tokio::test]
    async fn test_rate_limiter() {
        pause();
        
        let mut limiter = RateLimiter::new(2); // Very low limit for testing
        
        // First request should go through immediately
        let start = Instant::now();
        limiter.wait_if_needed().await;
        limiter.record_request();
        assert!(start.elapsed() < Duration::from_millis(10));
        
        // Second request should also go through
        limiter.wait_if_needed().await;
        limiter.record_request();
        
        // Third request should be rate limited
        let start = Instant::now();
        let wait_future = limiter.wait_if_needed();
        
        // Advance time to simulate waiting
        advance(Duration::from_secs(3600)).await;
        wait_future.await;
        
        resume();
    }

    #[test]
    fn test_backoff_calculation() {
        let client = GitHubApiClient::new();
        
        assert_eq!(client.calculate_backoff_delay(1), Duration::from_secs(1));
        assert_eq!(client.calculate_backoff_delay(2), Duration::from_secs(2));
        assert_eq!(client.calculate_backoff_delay(3), Duration::from_secs(4));
        
        // Should cap at max_delay
        let long_delay = client.calculate_backoff_delay(10);
        assert!(long_delay <= client.retry_config.max_delay);
    }
}