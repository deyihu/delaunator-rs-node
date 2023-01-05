#![deny(clippy::all)]

use delaunator::{triangulate, Point, Triangulation};
// use napi::bindgen_prelude::Object;
use napi_derive::napi;

// https://github.com/mourner/delaunator-rs/blob/81e58c6f63b40fa34e98feaca69ac5811fe5ca26/src/lib.rs#L146
#[napi(object)]
pub struct DResult {
  pub triangles: Vec<u32>,

  /// A vector of adjacent halfedge indices that allows traversing the triangulation graph.
  ///
  /// `i`-th half-edge in the array corresponds to vertex `triangles[i]`
  /// the half-edge is coming from. `halfedges[i]` is the index of a twin half-edge
  /// in an adjacent triangle (or `EMPTY` for outer half-edges on the convex hull).
  pub halfedges: Vec<u32>,

  /// A vector of indices that reference points on the convex hull of the triangulation,
  /// counter-clockwise.
  pub hull: Vec<u32>,
}

#[napi]
pub fn delaunator(points: Vec<f64>) -> DResult {
  let result: Triangulation = triangulate(&array_to_points(points));
  return DResult {
    triangles: usize_tou32(result.triangles),
    halfedges: usize_tou32(result.halfedges),
    hull: usize_tou32(result.hull),
  };
}

fn array_to_points(v: Vec<f64>) -> Vec<Point> {
  let mut points: Vec<Point> = Vec::new();
  let mut i = 0;
  let mut x: f64 = 0.0;
  let mut y: f64 = 0.0;

  for p in v {
    if i == 0 {
      x = p;
      i += 1;
      continue;
    }
    if i == 1 {
      y = p;
      let point = Point { x, y };
      points.push(point);
      i = 0;
    }
  }
  return points;
}

fn usize_tou32(v: Vec<usize>) -> Vec<u32> {
  let mut arr: Vec<u32> = Vec::new();
  for i in v {
    arr.push(i as u32);
  }
  return arr;
}
