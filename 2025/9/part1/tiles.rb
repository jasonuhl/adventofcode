#!/usr/bin/env ruby

# 2D cross product of OA and OB vectors, i.e. z-component of their 3D cross product.
# Returns a positive value, if OAB makes a counter-clockwise turn,
# negative for clockwise turn, and zero if the points are collinear.
# (From: https://en.wikibooks.org/wiki/Algorithm_Implementation/Geometry/Convex_hull/Monotone_chain)
def cross(o, a, b)
  (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
end

def convex_hull(tiles)
  tiles.sort!

  lower = []
  tiles.each do |tile|
    while lower.size >= 2 && cross(lower[-2], lower[-1], tile) <= 0
      lower.pop
    end
    lower.push(tile)
  end

  upper = []
  tiles.reverse_each do |tile|
    while upper.size >= 2 && cross(upper[-2], upper[-1], tile) <= 0
      upper.pop
    end
    upper.push(tile)
  end

  lower.pop
  upper.pop
  convex_points = lower + upper
  convex_points
end
fail unless convex_hull((0..9).to_a.repeated_permutation(2).to_a) == [[0, 0], [9, 0], [9, 9], [0, 9]]

def main
  f = open("input.txt")
  tiles = []
  f.each_line do |line|
    col, row = line.split(",").map { |x| x.to_i }
    tiles.push([row, col])
  end

  points = convex_hull(tiles)

  max_area = 0
  rect_corners = []
  points.each do |corner1|
    points.each do |corner2|
      area = ((corner1[0] - corner2[0]).abs + 1) * ((corner1[1] - corner2[1]).abs + 1)
      if area > max_area
        max_area = area
        rect_corners = [corner1, corner2]
      end
    end
  end

  p max_area
  p rect_corners
end

main()
