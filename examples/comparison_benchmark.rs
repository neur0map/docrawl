use std::fs;
use std::path::Path;
use std::time::Instant;

// Test HTML samples from various sources
const TEST_HTML_SAMPLES: &[(&str, &str)] = &[
    (
        "hackerhub",
        r#"
<!DOCTYPE html>
<html>
<head><title>HackerHub.me</title></head>
<body>
<nav><a href="/home">Home</a></nav>
<main>
<h1>The cybersecurity student hub</h1>
<p>Everything cybersecurity students and enthusiasts need in one place—no more scattered googling.</p>
<h2>Features</h2>
<ul>
<li>Learning resources</li>
<li>Tools and utilities</li>
<li>Community forums</li>
</ul>
<table>
<tr><th>Tool</th><th>Description</th></tr>
<tr><td>Burp Suite</td><td>Web application testing</td></tr>
<tr><td>Metasploit</td><td>Penetration testing framework</td></tr>
</table>
<blockquote>
"The only person you are destined to become is the person you decide to be." — Ralph Waldo Emerson
</blockquote>
<pre><code>#!/bin/bash
echo "Hello, Security World!"
</code></pre>
</main>
</body>
</html>
"#,
    ),
    (
        "documentation",
        r#"
<!DOCTYPE html>
<html>
<head><title>API Documentation</title></head>
<body>
<article>
<h1>Getting Started</h1>
<p>This guide will help you get started with our API.</p>
<h2>Authentication</h2>
<p>Include your API key in the header:</p>
<pre><code>Authorization: Bearer YOUR_API_KEY</code></pre>
<h2>Endpoints</h2>
<table>
<tr><th>Method</th><th>Endpoint</th><th>Description</th></tr>
<tr><td>GET</td><td>/api/users</td><td>List all users</td></tr>
<tr><td>POST</td><td>/api/users</td><td>Create a new user</td></tr>
</table>
<h3>Example Request</h3>
<pre><code class="language-json">{
  "name": "John Doe",
  "email": "john@example.com"
}</code></pre>
</article>
</body>
</html>
"#,
    ),
    (
        "blog_post",
        r#"
<!DOCTYPE html>
<html>
<head><title>My Blog Post</title></head>
<body>
<article>
<h1>Understanding Rust's Ownership System</h1>
<p><em>Published on October 15, 2025</em></p>
<p>Rust's ownership system is one of its most unique features...</p>
<h2>What is Ownership?</h2>
<p>Ownership is a set of rules that govern how a Rust program manages memory.</p>
<h3>The Three Rules of Ownership</h3>
<ol>
<li>Each value in Rust has a variable that's called its owner.</li>
<li>There can only be one owner at a time.</li>
<li>When the owner goes out of scope, the value will be dropped.</li>
</ol>
<h2>Code Example</h2>
<pre><code class="language-rust">fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    println!("{}", s2); // This works
    // println!("{}", s1); // This would fail!
}</code></pre>
<blockquote>
<p><strong>Tip:</strong> Think of ownership like a book - only one person can own it at a time!</p>
</blockquote>
</article>
</body>
</html>
"#,
    ),
];

fn main() {
    println!("=== HTML to Markdown Comparison Benchmark ===\n");

    let mut total_fast_time = std::time::Duration::new(0, 0);
    let mut total_fast_chars = 0;

    for (name, html) in TEST_HTML_SAMPLES {
        println!("Testing sample: {}", name);

        // Test fast_html2md
        let start = Instant::now();
        let result_fast = html2md::rewrite_html(html, false);
        let duration_fast = start.elapsed();

        total_fast_time += duration_fast;
        total_fast_chars += result_fast.len();

        println!("  fast_html2md conversion time: {:?}", duration_fast);
        println!("  fast_html2md output length: {} chars", result_fast.len());
        println!(
            "  fast_html2md output preview:\n{}",
            &result_fast[..result_fast.len().min(300)]
        );
        if result_fast.len() > 300 {
            println!("  ...");
        }
        println!();

        // Save output for comparison
        let output_dir = Path::new("comparison_outputs");
        fs::create_dir_all(output_dir).ok();
        let output_file = output_dir.join(format!("{}_fast_html2md.md", name));
        fs::write(&output_file, &result_fast).ok();
        println!("  Saved to: {:?}", output_file);
        println!();
    }

    println!("=== Summary ===");
    println!("Total fast_html2md time: {:?}", total_fast_time);
    println!("Total fast_html2md chars: {}", total_fast_chars);
    println!(
        "Average fast_html2md time per sample: {:?}",
        total_fast_time / TEST_HTML_SAMPLES.len() as u32
    );
    println!();

    println!("Comparison completed. Check comparison_outputs/ directory for results.");
}
