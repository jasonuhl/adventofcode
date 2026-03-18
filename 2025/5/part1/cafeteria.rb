#!/usr/bin/env ruby

def unoverlap_ranges(ranges)
  ranges.sort!
  ranges_out = []

  nextidx = 0
  ranges.each_with_index do |r, i|
    next if i < nextidx

    nextidx += 1
    while nextidx < ranges.size && ranges[nextidx][0] <= r[1]
      r[1] = [r[1], ranges[nextidx][1]].max
      nextidx += 1
    end

    ranges_out.push(r)
  end

  ranges_out
end

def main
  ranges = []
  freshcount = 0

  f = open("input.txt")
  doing_ranges = true
  f.each_line do |line|
    line = line.chomp
    if line.empty?
      doing_ranges = false
      ranges = unoverlap_ranges(ranges)
      p "----- #{ranges.size} Non-overlapping ranges:"
      ranges.each { |r| p r }
      p "-----"
    end

    if doing_ranges
      low, high = line.split('-').map { |x| x.to_i }
      ranges.push([low, high])
    else
      id = line.to_i
      range = ranges.bsearch { |r| r[1] >= id }
      if !range.nil? && range[0] <= id
        puts "#{id} is fresh because of range #{range}"
        freshcount += 1
      else
        puts "#{id} is spoiled"
      end
    end
  end
  puts "Total fresh: #{freshcount}"
end

main()
