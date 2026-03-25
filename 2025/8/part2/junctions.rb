#!/usr/bin/env ruby

# Prim's MST algorithm — O(N²) time, O(N) memory.

def distance_sq(box, other)
  x = box[0] - other[0]
  y = box[1] - other[1]
  z = box[2] - other[2]
  x * x + y * y + z * z
end

def main
  f = open("input.txt")
  boxes = []
  f.each_line do |line|
    boxes.push(line.split(",").map { |val| val.to_i })
  end

  n = boxes.size
  in_mst = Array.new(n, false)
  min_dist = Array.new(n, Float::INFINITY)
  min_from = Array.new(n, -1)

  # Start from vertex 0.
  min_dist[0] = 0
  last_u = nil
  last_from = nil

  n.times do
    # Find the unvisited vertex with smallest min_dist.
    u = nil
    best = Float::INFINITY
    n.times do |v|
      if !in_mst[v] && min_dist[v] < best
        best = min_dist[v]
        u = v
      end
    end

    in_mst[u] = true
    last_u = u
    last_from = min_from[u]

    # Update distances to remaining vertices.
    n.times do |v|
      next if in_mst[v]
      d = distance_sq(boxes[u], boxes[v])
      if d < min_dist[v]
        min_dist[v] = d
        min_from[v] = u
      end
    end
  end

  p "All connected, x product is #{boxes[last_from][0] * boxes[last_u][0]}."
end

main()
