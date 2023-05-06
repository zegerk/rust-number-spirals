use std::f64::consts::PI;
use primal::Sieve;

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

          //println!("{} {:?} {}", i, prime_factors, factors[i]);
      }
  }

  factors
}

fn draw_circle_and_text(document: svg::Document, res: usize, n:usize, i: usize, sieve: &Vec<usize>) -> svg::Document {

  let pi2 = 2.0 * PI;
  let half_size = res as f64 / 2.0;
  let base_scale = (res as f64 / 2.2) / ((n as f64).sqrt() + 1.0);
  let divisors = sieve[i];

  let r = (i as f64).sqrt();
  let theta = r * pi2;
  let x = theta.cos() * r * base_scale + half_size;
  let y = (-theta.sin()) * r * base_scale + half_size;

  let mut mersenne = 0;
  let mut twin = 0;
  let mut germain = 0;
  let mut safe = 0;

  let color = if divisors == 1 {
    
    mersenne = (((i - 1) & ((i - 1) - 1)) == 0) as u8;
    twin = (sieve.len() > i + 2 && sieve[i+2] == 1) as u8;
    germain = (sieve.len() > 2 * i + 1 && sieve[2 * i + 1] == 1) as u8;
    safe = ((i - 1) / 2 > 0 && sieve[(i - 1) / 2] == 1) as u8;

    let red = 128u8.wrapping_add((germain << 5) + (safe << 5));
    let blue = mersenne << 7;
    let green = twin << 7;


    //println!("{} {} {} {}", i, red, green, blue);

    format!("#{:0>2x}{:0>2x}{:0>2x}{:0>2x}", red, green, blue, 255)
  } else {
    format!("#{:0>2x}{:0>2x}{:0>2x}{:0>2x}", 128, 128, 128, (32 * divisors - 1) as u8)
  };

  // Compute the area of the whole circle of dots, divide by the amount of numbers and
  // compute the radius of a single circle
  let radius = ((res as f64).powf(2.0) / (n as f64)).sqrt() / 2.0;
  
  let circle = svg::node::element::Circle::new()
      .set("cx", x)
      .set("cy", y)
      .set("r", radius)
      .set("fill", color);

  let text = svg::node::element::Text::new()
      .set("x", x)
      .set("y", y + radius / 4.0) // Adjusted to vertically center the text
      .set("font-size", radius)
      .set("text-anchor", "middle")
      .set("fill", "white")
      .add(svg::node::Text::new(i.to_string()));

  let title = svg::node::element::Title::new()
      .add(svg::node::Text::new(format!(
        "{}, divisors: {}\nIs mersenne: {}\nIs twin: {}\nIs germain: {}\nIs safe: {}", 
        i.to_string(),
        divisors, mersenne, twin, germain, safe))
      );

  let group = svg::node::element::Group::new()
      .add(circle);
      //.add(text);
      //.add(title);

  document.add(group)
}


fn render(res: usize, n: usize) -> svg::Document {
  let mut document = svg::Document::new().set("viewBox", (0, 0, res, res));

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
          document = draw_circle_and_text(document, res, n, j + 1, &sieve);
      }
  }

  // Draw prime numbers
  for j in 0..n {
      let strength = sieve[j + 1] as u32;

      if strength == 1 {
          document = draw_circle_and_text(document, res, n, j + 1, &sieve);
      }
  }

  document
}

fn main() {
    let res = 4096;
    let n = 10000;
    let document = render(res, n);

    svg::save("output.svg", &document).unwrap();
}
