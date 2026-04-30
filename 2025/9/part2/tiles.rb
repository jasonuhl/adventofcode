#!/usr/bin/env ruby

def main
  f = open("input.txt")
  tiles = []
  f.each_line do |line|
    col, row = line.split(",").map { |x| x.to_i }
    tiles.push([row, col])
  end

  n = tiles.size
  vertical_edges = []
  n.times do |i|
    a = tiles[i]
    b = tiles[(i + 1) % n]
    fail "non-axis-aligned edge between #{a} and #{b}" unless a[0] == b[0] || a[1] == b[1]
    if a[1] == b[1]
      y1, y2 = [a[0], b[0]].sort
      vertical_edges.push([y1, y2, a[1]])
    end
  end

  rs = tiles.map { |t| t[0] }.uniq.sort
  cs = tiles.map { |t| t[1] }.uniq.sort
  nr = rs.size
  nc = cs.size

  inside = Array.new(nr - 1) { Array.new(nc - 1, false) }
  (nr - 1).times do |i|
    y_mid = (rs[i] + rs[i + 1]) / 2.0
    crossing_xs = vertical_edges
      .select { |y1, y2, _| y1 < y_mid && y_mid < y2 }
      .map { |_, _, x| x }
      .sort
    remaining = crossing_xs.size
    k = 0
    (nc - 1).times do |j|
      x_mid = (cs[j] + cs[j + 1]) / 2.0
      while k < crossing_xs.size && crossing_xs[k] < x_mid
        k += 1
        remaining -= 1
      end
      inside[i][j] = remaining.odd?
    end
  end

  prefix = Array.new(nr) { Array.new(nc, 0) }
  (1...nr).each do |i|
    (1...nc).each do |j|
      cell_area = inside[i - 1][j - 1] ? (rs[i] - rs[i - 1]) * (cs[j] - cs[j - 1]) : 0
      prefix[i][j] = cell_area + prefix[i - 1][j] + prefix[i][j - 1] - prefix[i - 1][j - 1]
    end
  end

  row_to_idx = {}
  rs.each_with_index { |r, i| row_to_idx[r] = i }
  col_to_idx = {}
  cs.each_with_index { |c, j| col_to_idx[c] = j }

  max_area = 0
  rect_corners = []
  tiles.each_with_index do |c1, idx1|
    tiles.each_with_index do |c2, idx2|
      next if idx2 <= idx1
      r_lo, r_hi = [c1[0], c2[0]].minmax
      c_lo, c_hi = [c1[1], c2[1]].minmax
      next if r_lo == r_hi || c_lo == c_hi

      i_lo = row_to_idx[r_lo]
      i_hi = row_to_idx[r_hi]
      j_lo = col_to_idx[c_lo]
      j_hi = col_to_idx[c_hi]

      inside_area = prefix[i_hi][j_hi] - prefix[i_lo][j_hi] - prefix[i_hi][j_lo] + prefix[i_lo][j_lo]
      geom_area = (r_hi - r_lo) * (c_hi - c_lo)
      next unless inside_area == geom_area

      tile_area = (r_hi - r_lo + 1) * (c_hi - c_lo + 1)
      if tile_area > max_area
        max_area = tile_area
        rect_corners = [c1, c2]
      end
    end
  end

  p max_area
  p rect_corners
end

main()
