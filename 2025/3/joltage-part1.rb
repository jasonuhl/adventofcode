#!/usr/bin/env ruby

def main
  f = open("input.txt")
  sum = 0
  f.each_line do |line|
    chars = line.chomp.chars
    first = chars[...-1].max
    second = chars[chars.index(first) + 1..].max
    m = "#{first}#{second}"
    p "line #{line} max #{m}"
    sum += m.to_i
  end

  p "Sum: #{sum}"
end

main()
