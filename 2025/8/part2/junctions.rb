#!/usr/bin/env ruby

def distance(box, other)
  x = box[0] - other[0]
  y = box[1] - other[1]
  z = box[2] - other[2]

  Math.sqrt(x**2 + y**2 + z**2)
end

def main
  a=[]
  f = open("input.txt")
  boxes = []
  f.each_line do |line|
    boxes.push(line.split(",").map { |val| val.to_i })
  end

  distances = []
  boxes.each_with_index do |box, i|
    boxes[i+1..].each do |other|
      distances.push([distance(box, other), box, other])
    end
  end

  distances.sort!
  #distances.each { |d| p d }

  circuits = {}
  circuit_count = 0
  box_circuits = {}

  distances.each do |d|
    if box_circuits[d[1]].nil? && box_circuits[d[2]].nil?
      circid = circuits.size
      circuits[circid] = [d[1], d[2]]
      circuit_count += 1
      box_circuits[d[1]] = circid
      box_circuits[d[2]] = circid
    elsif box_circuits[d[1]] == box_circuits[d[2]]
      # Already in same circuit.  Nothing happens.
    elsif !box_circuits[d[1]].nil? && !box_circuits[d[2]].nil? && box_circuits[d[1]] != box_circuits[d[2]]
      # Joining two different circuits.
      circ = box_circuits[d[1]]
      othercirc = box_circuits[d[2]]
      circuits[othercirc].each do |box|
        box_circuits[box] = circ
      end
      circuits[circ] += circuits[othercirc]
      circuits[othercirc] = []
      circuit_count -= 1
    elsif !box_circuits[d[1]].nil? && box_circuits[d[2]].nil?
      circ = box_circuits[d[1]]
      box_circuits[d[2]] = circ
      circuits[circ].push(d[2])
    elsif box_circuits[d[1]].nil? && !box_circuits[d[2]].nil?
      circ = box_circuits[d[2]]
      box_circuits[d[1]] = circ
      circuits[circ].push(d[1])
    else
      fail "wtf"
    end

    if box_circuits.size == boxes.size && circuit_count == 1
      p "All connected after joining #{d}.  x product is #{d[1][0] * d[2][0]}."
      return
    end
  end
end

main()
