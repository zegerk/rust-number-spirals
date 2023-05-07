#![cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use std::f64::consts::PI;
use primal::Sieve;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SieveNumber {
    number: usize,
    divisors: usize,
    is_prime: bool,
    is_mersenne: bool,
    is_twin: bool,
    is_germain: bool,
    is_safe: bool,
}

/*
 * Return the count of unique factors
 */ 
fn factor_count_sieve(n: usize) -> Vec<usize> {
  let sieve = Sieve::new(n);
  let mut factors = vec![0; n + 1];

  for i in 2..=n {
      if factors[i] == 0 {
          let prime_factors = sieve.factor(i).unwrap();
          factors[i] = prime_factors.len();

          /*
           * Take powers of primes into account
           */
          if factors[i] == 1 {
            factors[i] = prime_factors[0].1;
          }
      }
  }

  factors
}

fn draw_number(document: svg::Document, res: usize, n:usize, i: usize, sieve: &Vec<usize>) -> svg::Document {

  let pi2 = 2.0 * PI;
  let half_size = res as f64 / 2.0;
  let base_scale = (res as f64 / 2.2) / ((n as f64).sqrt() + 1.0);

  let r = (i as f64).sqrt();
  let theta = r * pi2;
  let x = theta.cos() * r * base_scale + half_size;
  let y = (-theta.sin()) * r * base_scale + half_size;

  /*
   * Number properties
   */
  let divisors = sieve[i];
  let mut mersenne = false;
  let mut twin = false;
  let mut germain = false;
  let mut safe = false;

  // Compute the area of the whole circle of dots, divide by the amount of numbers and
  // compute the radius of a single circle
  let radius = ((res as f64).powf(2.0) / (n as f64)).sqrt() / 2.0;

  /*
   * The primes have color scheme, the non primes are gray
   */
  let color = if divisors == 1 {
    
    mersenne = ((i - 1) & ((i - 1) - 1)) == 0;
    twin = sieve.len() > i + 2 && sieve[i+2] == 1;
    germain = sieve.len() > 2 * i + 1 && sieve[2 * i + 1] == 1;
    safe = (i - 1) / 2 > 0 && sieve[(i - 1) / 2] == 1;

    let red = 128u8.wrapping_add(((germain as u8) << 5) + ((safe as u8) << 5));
    let blue = (mersenne as u8) << 7;
    let green = (twin as u8) << 7;
 
    format!("#{:0>2x}{:0>2x}{:0>2x}{:0>2x}", red, green, blue, 255)
  } else {
    format!("#{:0>2x}{:0>2x}{:0>2x}{:0>2x}", 128, 128, 128, (32 * divisors - 1) as u8)
  };

  let sieve_number = SieveNumber {
    number: i,
    divisors: divisors,
    is_prime: divisors == 1,
    is_mersenne: mersenne,
    is_twin: twin,
    is_germain: germain,
    is_safe: safe,
  };  

  let circle_class = 
    vec![
      "spiral-number",
      if divisors == 1 { "prime" } else { "composite" },
      if mersenne { "mersenne" } else { "" },
      if twin { "twin" } else { "" },
      if germain { "germain" } else { "" },
      if safe { "safe" } else { "" },
    ].join(" ");


  let sieve_number_json = match serde_json::to_string(&sieve_number) {
    Ok(json) => json,
    Err(_) => "".to_string()
  };

  let circle = svg::node::element::Circle::new()
      .set("cx", x)
      .set("cy", y)
      .set("r", radius)
      .set("fill", color)
      // Store the details for the number in a json array so we can use it on the frontend
      .set("number-data", sieve_number_json)
      .set("class", circle_class)
      .set("onclick", "handleCircleClick(evt)")
      ;

  let text = svg::node::element::Text::new()
      .set("x", x)
      .set("y", y + radius / 6.0) // Adjusted to vertically center the text
      .set("font-size", radius / 3.0)
      .set("text-anchor", "middle")
      .set("fill", "white")
      .set("class", "spiral-text")
      .add(svg::node::Text::new(i.to_string()))
      ;

  /* 
   * Add a number to the node make it really slow to visualize 
   */
  let group = svg::node::element::Group::new()
      .add(circle)
      //.add(text)
      ;

  document.add(group)
}


fn render(res: usize, n: usize) -> svg::Document {
  let mut document = svg::Document::new()
    .set("viewBox", (0, 0, res, res))
    .set("overflow", "auto")
    .set("id", "prime-numbers-svg");

  /*
   * We check for germain primes, so we need a bigger sieve 
   */ 
  let sieve = factor_count_sieve(2 * n + 1);

  /*
   * First draw the none prime numbers, SVG does not have Z index, if 
   * the prime numbers have to overlap the other numbers they have to be
   * drawn last
   */
  for j in 0..n {
      let strength = sieve[j + 1] as u32;

      if strength > 1 {
          document = draw_number(document, res, n, j + 1, &sieve);
      }
  }

  // Draw prime numbers
  for j in 0..n {
      let strength = sieve[j + 1] as u32;

      if strength == 1 {
          document = draw_number(document, res, n, j + 1, &sieve);
      }
  }

  document
}

#[wasm_bindgen]
pub fn generate_svg(n: u32) -> String {

    let n = n as usize;

    // This is the "resolution", but this has no actual impact on vector drawings :)
    let res = 4096; 
    let document = render(res, n);

    format!("{}", document)
}