/// Fetch input data from the Advent of Code server, with cache.
/// Copyright 2023 by Alex Utter
///
/// A simple API inspired by the Python "advent-of-code-data" package:
///     https://pypi.org/project/advent-of-code-data/
/// Uses the same "AOC_SESSION" environment variable for authentication,
/// so please follow their instructions for how to retrieve that token.

extern crate reqwest;
use reqwest::blocking::Client;
use std::env;
use std::fs;
use std::io::Result;
use std::path::Path;

/// Cache filename for a given year/day.
fn cache_filename(year: &usize, day: &usize) -> String
{
    return format!("input/input_{}_{:02}.txt", year, day);
}

/// Fetch input for a given year/day from the local cache.
fn read_from_cache(year: &usize, day: &usize) -> Option<String>
{
    let filename = cache_filename(year, day);
    return fs::read_to_string(filename).ok();
}

/// Write input data to the local cache.
fn save_to_cache(year: &usize, day: &usize, data: &str) -> Result<()>
{
    let filename = cache_filename(year, day);
    let parent = Path::new(&filename).parent().unwrap();
    fs::create_dir_all(&parent)?;
    fs::write(&filename, data)?;
    Ok(())
}

/// Fetch input for a given year/day from the Advent of Code server.
/// Requires environment variable "AOC_SESSION" for authentication.
fn read_from_web(year: &usize, day: &usize) -> Option<String>
{
    if let Ok(session) = env::var("AOC_SESSION") {
        // Initialize HTTPS client.
        let client = Client::builder()
            .user_agent("ooterness_aocd_knockoff")
            .build().unwrap();

        // Manually build header so we don't need reqwest::cookie.
        let tok = format!("session={}", session);
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

        // Attempt to fetch the input data.
        return client.get(url)
            .header(reqwest::header::COOKIE, &tok)
            .send().ok()
            .and_then( |x| x.text().ok() );
    } else { None }
}

/// Fetch input for a given year/day from cache if available.
/// Otherwise, download from server and update local cache.
pub fn get_data(year: usize, day: usize) -> Option<String>
{
    if let Some(data) = read_from_cache(&year, &day) {
        // Use local cache.
        return Some(data);
    } else if let Some(data) = read_from_web(&year, &day) {
        // Fetch from server and update cache.
        save_to_cache(&year, &day, &data)
            .unwrap_or_else(|err| println!("{}", err));
        return Some(data);
    } else {
        // No cache and unable to contact server.
        return None;
    }
}
