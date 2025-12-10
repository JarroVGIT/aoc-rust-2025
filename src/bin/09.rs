use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(9);

#[allow(unused_variables)]
pub fn part_one(input: &str) -> Option<usize> {
    let tiles: Vec<(usize, usize)> = input.lines().map(|l| {
        let (x,y) = l.split_once(',').unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    }).collect_vec();

    let mut result = 0;
    for i in 0..tiles.len() {
        for j in i+1..tiles.len() {
            let dx = tiles[i].0.abs_diff(tiles[j].0) + 1;
            let dy = tiles[i].1.abs_diff(tiles[j].1) + 1;
            result = result.max(dx * dy)
        }
    }
    Some(result)
}

fn inside_polygon(corner: (usize, usize), vertices: &[((usize, usize), (usize,usize))], cache: &mut HashMap<(usize, usize), bool>) -> bool {
    if let Some(r) = cache.get(&corner) {
        return *r
    }
    let mut count = 0;
    let (cx, cy) = corner;

    for &((x1, y1), (x2,y2)) in vertices {
        let (_, x2) = (x1.min(x2), x1.max(x2));
        let (y1, y2) = (y1.min(y2), y1.max(y2));
        if  cy >= y1 && cy <= y2 && cx <= x2 {
            count += 1
        }
    }
    let r = count % 2 != 0;
    cache.insert(corner, r);
    r
}

pub fn crosses_verticals(h_side: ((usize, usize), (usize,usize)), verticals: &[((usize, usize), (usize,usize))]) -> bool {
    // the verticals are stored such that y2 > y1 and x1==x2.
    // the y's of the h-side  are the same.
    let ((hx1, hy), (hx2, _)) = h_side;
    let (hx1, hx2) = (hx1.min(hx2), hx1.max(hx2));
    verticals.iter().any(|&((x1, y1), (_, y2))| {
        hx1 < x1 && hx2 > x1 && hy < y2 && hy > y1
    })
}

pub fn crosses_horizontal(v_side: ((usize, usize), (usize,usize)), horizontals: &[((usize, usize), (usize,usize))]) -> bool {
    // the horizontals are stored such that x2 > x1 and y1==y2.
    // the x's of the v-side  are the same.
    let ((vx, vy1), (_, vy2)) = v_side;
    let (vy1, vy2) = (vy1.min(vy2), vy1.max(vy2));
    horizontals.iter().any(|&((x1, y1), (x2, _))| {
        vx > x1 && vx < x2 && vy1 < y1 && vy2 > y1
    })
}

#[allow(unused_variables)]
pub fn part_two(input: &str) -> Option<usize> {
    // https://stackoverflow.com/questions/217578/how-can-i-determine-whether-a-2d-point-is-within-a-polygon
    // create vec with vertices and determine max x + 1.
    // for each potential square, determie if the other corners are in the polygon by checking how often the vertex
    // cx,cy -> maxx, cy intersects with another vertices. If odd, then inside polygon.
    // no it's harder than that; need to check if any of the horizontal borders does not cross vertical vertices
    // and if any of the vertical borders does not cross any horizontal vertices.

    let mut tiles: Vec<(usize, usize)> = input.lines().map(|l| {
        let (x,y) = l.split_once(',').unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    }).collect_vec();
    let corners = tiles.clone();
    let start = tiles[0];
    tiles.push(start);

    // sorted from small to large point.
    let vertices = tiles.iter().copied().zip(tiles.iter().skip(1).copied()).map(|((x1,y1),(x2,y2))| {
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (y1, y2) = (y1.min(y2), y1.max(y2));
        ((x1, y1), (x2, y2))
    }).collect_vec();

    let even = vertices.iter().step_by(2).copied().collect_vec();
    let odds = vertices.iter().skip(1).step_by(2).copied().collect_vec();
    let (vertical, horizontal) = if vertices[0].0.0 == vertices[0].0.1 { // first is vertical
        (even, odds)
    } else {
        (odds, even)
    };

    let mut cache = HashMap::new();
    let mut result = 0;
    for i in 0..tiles.len() {
        for j in i+1..tiles.len() {
            let (h1, h2, v1, v2) = {
                (
                    (tiles[i], (tiles[j].0, tiles[i].1)),
                    (tiles[j], (tiles[i].0, tiles[j].1)),
                    (tiles[i], (tiles[i].0, tiles[j].1)),
                    (tiles[j], (tiles[j].0, tiles[i].1)),
                )
            };
            let c1 = (tiles[i].0, tiles[j].1);
            let c2 = (tiles[j].0, tiles[i].1);
            if !crosses_horizontal(v1, &horizontal)
                && !crosses_horizontal(v2, &horizontal)
                && !crosses_verticals(h1, &vertical)
                && !crosses_verticals(h2, &vertical)
                && inside_polygon(c1, &vertices, &mut cache)
                && inside_polygon(c2, &vertices, &mut cache)
                {
                    let dx = tiles[i].0.abs_diff(tiles[j].0) + 1;
                    let dy = tiles[i].1.abs_diff(tiles[j].1) + 1;
                    result = result.max(dx * dy)
                }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
