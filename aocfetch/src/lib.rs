/// Fetch input data from the Advent of Code server, with cache.
/// Copyright 2023 by Alex Utter
///
/// A simple API inspired by the Python "advent-of-code-data" package:
///     https://pypi.org/project/advent-of-code-data/
/// Uses the same "AOC_SESSION" environment variable for authentication,
/// so please follow their instructions for how to retrieve that token.

extern crate reqwest;
use reqwest::blocking::Client;
type ErrMsg = Box<dyn std::error::Error>;

/// Cache filename for a given year/day.
fn cache_filename(year: &usize, day: &usize) -> String
{
    format!("input/input_{}_{:02}.txt", year, day)
}

/// Fetch input for a given year/day from the local cache.
fn read_from_cache(year: &usize, day: &usize) -> Option<String>
{
    std::fs::read_to_string(&cache_filename(year, day)).ok()
}

/// Write input data to the local cache.
fn save_to_cache(year: &usize, day: &usize, data: &str) -> Result<(), ErrMsg>
{
    let filename = cache_filename(year, day);
    let parent = std::path::Path::new(&filename).parent().unwrap();
    std::fs::create_dir_all(&parent)?;
    std::fs::write(&filename, data)?;
    Ok(())
}

/// Fetch input for a given year/day from the Advent of Code server.
/// Requires environment variable "AOC_SESSION" for authentication.
fn read_from_web(year: &usize, day: &usize) -> Result<String, ErrMsg>
{
    if let Ok(session) = std::env::var("AOC_SESSION") {
        // Initialize HTTPS client.
        let client = Client::builder()
            .user_agent("ooterness_aocd_knockoff")
            .build().unwrap();

        // Manually build header so we don't need reqwest::cookie.
        let tok = format!("session={}", session.trim());
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

        // Attempt to fetch the input data from the AoC server.
        let response = client.get(url)
            .header(reqwest::header::COOKIE, &tok)
            .send()?;
        let status = response.status().as_u16();
        let data = response.text()?;

        // Filter by HTTP response code.
        // We could use "error_from_response()", but custom error messages
        // from the AoC server are much easier for end-users to understand.
        if status == 200 {
            // 200 OK indicates the message is the user's input data.
            return Ok(data);
        } else {
            // Any other response contains a human-readable error message.
            // (e.g., Bad login token, problem not yet posted, etc.)
            return Err(data.into());
        }
    } else {
        return Err("Missing AOC_SESSION environment variable. Instructions here:\n\
                    https://pypi.org/project/advent-of-code-data/".into());
    }
}

/// Fetch input for a given year/day from cache if available.
/// Otherwise, download from server and update local cache.
pub fn get_data(year: usize, day: usize) -> Result<String, ErrMsg>
{
    // Use local cache if possible.
    if let Some(data) = read_from_cache(&year, &day) {return Ok(data);}

    // Attempt to fetch from server...
    let data = read_from_web(&year, &day)?;

    // If successful, update cache before returning.
    save_to_cache(&year, &day, &data)?;
    return Ok(data);
}
