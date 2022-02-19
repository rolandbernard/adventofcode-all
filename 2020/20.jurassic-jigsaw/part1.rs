
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn main() {
    let stdin = io::stdin();
    let mut tiles = Vec::new();
    let mut tile = Vec::new();
    let mut tile_id = 0;
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        if line == "" {
            tiles.push((tile_id, tile));
            tile = Vec::new();
        } else {
            let splt = line.trim_matches(':').split(" ").collect::<Vec<_>>();
            if splt.len() == 2 {
                tile_id = splt[1].parse::<usize>().unwrap();
            } else {
                tile.push(line.chars().map(|c| if c == '#' { 1 } else { 0 }).collect::<Vec<_>>());
            }
        }
    }
    if tile.len() != 0 {
        tiles.push((tile_id, tile));
    }
    let mut tile_edg = vec![Vec::new(); tiles.len()];
    let mut edges = HashMap::new();
    for (i, (_, t)) in tiles.iter().enumerate() {
        for k in [0, t.len() - 1] {
            let mut edge = 0;
            let mut pow = 1;
            for j in 0..t.len() {
                edge += pow * t[j][k];
                pow *= 2;
            }
            let mut edge2 = 0;
            pow = 1;
            for j in (0..t.len()).rev() {
                edge2 += pow * t[j][k];
                pow *= 2;
            }
            edge = edge.min(edge2);
            tile_edg[i].push(edge);
            edges.entry(edge).or_insert(Vec::new());
            edges.get_mut(&edge).unwrap().push(i);
        }
        for k in [0, t.len() - 1] {
            let mut edge = 0;
            let mut pow = 1;
            for j in 0..t[k].len() {
                edge += pow * t[k][j];
                pow *= 2;
            }
            let mut edge2 = 0;
            pow = 1;
            for j in (0..t[k].len()).rev() {
                edge2 += pow * t[k][j];
                pow *= 2;
            }
            edge = edge.min(edge2);
            tile_edg[i].push(edge);
            edges.entry(edge).or_insert(Vec::new());
            edges.get_mut(&edge).unwrap().push(i);
        }
    }
    let mut res = 1;
    for (i, (t, _)) in tiles.iter().enumerate() {
        let mut edge = 0;
        for e in &tile_edg[i] {
            if edges[e].len() == 1 {
                edge += 1;
            }
        }
        if edge == 2 {
            res *= t;
        }
    }
    println!("Result: {}", res);
}

