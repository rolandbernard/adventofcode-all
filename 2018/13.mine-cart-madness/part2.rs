
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut map = stdin.lock().lines()
        .map(|x| x.unwrap().chars().collect())
        .collect::<Vec<Vec<_>>>();
    let mut carts = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            match map[i][j] {
                '^' => carts.push((i as i64, j as i64, -1, 0, 0, true)),
                'v' => carts.push((i as i64, j as i64, 1, 0, 0, true)),
                '<' => carts.push((i as i64, j as i64, 0, -1, 0, true)),
                '>' => carts.push((i as i64, j as i64, 0, 1, 0, true)),
                _ => {},
            }
            match map[i][j] {
                '^' | 'v' => map[i][j] = '|',
                '<' | '>' => map[i][j] = '-',
                _ => {},
            }
        }
    }
    while carts.len() > 1 {
        for i in 0..carts.len() {
            {
                let cart = &mut carts[i];
                match map[cart.0 as usize][cart.1 as usize] {
                    '+' => {
                        if cart.4 == 0 {
                            (cart.2, cart.3) = (-cart.3, cart.2);
                        } else if cart.4 == 2 {
                            (cart.2, cart.3) = (cart.3, -cart.2);
                        }
                        cart.4 = (cart.4 + 1) % 3;
                    },
                    '/' => {
                        if cart.2 == 0 {
                            (cart.2, cart.3) = (-cart.3, 0);
                        } else {
                            (cart.2, cart.3) = (0, -cart.2);
                        }
                    },
                    '\\' => {
                        if cart.2 == 0 {
                            (cart.2, cart.3) = (cart.3, 0);
                        } else {
                            (cart.2, cart.3) = (0, cart.2);
                        }
                    },
                    _ => {},
                }
                cart.0 += cart.2;
                cart.1 += cart.3;
            }
            for j in 0..carts.len() {
                if i != j {
                    if (carts[i].0, carts[i].1) == (carts[j].0, carts[j].1) {
                        carts[i].5 = false;
                        carts[j].5 = false;
                    }
                }
            }
        }
        carts = carts.into_iter().filter(|x| x.5).collect();
    }
    println!("Result: {},{}", carts[0].1, carts[0].0);
}

