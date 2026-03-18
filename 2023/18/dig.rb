#!/usr/bin/env ruby

def main
  f = open("input.txt")
  a = [[0, 0]]
  vectors = [[0,  1], # R
             [1,  0], # D
             [0, -1], # L
             [-1, 0]] # U
  f.each_line do |line|
    (_, _, hex) = line.chomp.split(' ')
    distance = Integer("0x#{hex[2..6]}")
    vector = vectors[hex[7].to_i]

    newpoint = vector.map { |n| distance * n }.zip(a[-1]).map { |n| n.inject(:+) }
    a.push(newpoint)
  end

  # Shoelace formula
  area = 0
  (0...a.size).each do |i|
    j = (i + 1) % a.size

    # perimeter
    area += (a[i][0] - a[j][0]).abs
    area += (a[i][1] - a[j][1]).abs

    # inner area
    area += a[i][1] * a[j][0]
    area -= a[j][1] * a[i][0]
  end
  area /= 2
  area += 1	# origin square

  p area
end

main()
