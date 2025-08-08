use regex::Regex;
use std::collections::HashSet;

/// Security-by-default: redact common secret patterns
pub fn redact_secrets(text: &str) -> String {
    lazy_static::lazy_static! {
        static ref SECRET_PATTERNS: Vec<Regex> = vec![
            Regex::new(r"(?i)(token|secret|password|bearer|api[_-]?key)\s*[:=]\s*['\"]?([a-zA-Z0-9_\-]{8,})['\"]?").unwrap(),
            Regex::new(r"(?i)authorization\s*:\s*bearer\s+([a-zA-Z0-9_\-\.]{20,})").unwrap(),
            Regex::new(r"(?i)(aws_access_key_id|aws_secret_access_key)\s*[:=]\s*['\"]?([A-Z0-9]{16,})['\"]?").unwrap(),
        ];
    }

    let mut redacted = text.to_string();
    
    for pattern in SECRET_PATTERNS.iter() {
        redacted = pattern.replace_all(&redacted, |caps: &regex::Captures| {
            if caps.len() >= 3 {
                format!("{}=***REDACTED***", &caps[1])
            } else {
                "***REDACTED***".to_string()
            }
        }).to_string();
    }
    
    redacted
}

/// Check if a path should be followed (security: no symlinks by default)
pub fn should_follow_path(path: &std::path::Path, follow_symlinks: bool) -> bool {
    if !follow_symlinks && path.is_symlink() {
        return false;
    }
    
    // Security: skip common sensitive directories
    let sensitive_dirs: HashSet<&str> = [
        ".git", ".svn", ".hg",
        "node_modules", "target", "dist", "build",
        ".env", ".secrets",
    ].iter().cloned().collect();
    
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        if sensitive_dirs.contains(name) {
            return false;
        }
    }
    
    true
}

/// Canonicalize path safely
pub fn canonicalize_path_safe(path: &std::path::Path) -> std::path::PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}