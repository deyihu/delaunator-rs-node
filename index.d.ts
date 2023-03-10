/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface DResult {
  triangles: Array<number>
  /**
   * A vector of adjacent halfedge indices that allows traversing the triangulation graph.
   *
   * `i`-th half-edge in the array corresponds to vertex `triangles[i]`
   * the half-edge is coming from. `halfedges[i]` is the index of a twin half-edge
   * in an adjacent triangle (or `EMPTY` for outer half-edges on the convex hull).
   */
  halfedges: Array<number>
  /**
   * A vector of indices that reference points on the convex hull of the triangulation,
   * counter-clockwise.
   */
  hull: Array<number>
}
export function delaunator(points: Array<number>): DResult
