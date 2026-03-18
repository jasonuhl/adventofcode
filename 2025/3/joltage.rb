#!/usr/bin/env ruby

def main
  f = open("input.txt")
  sum = 0
  f.each_line do |line|
    chars = line.chomp.chars
    digits = []

    (0..11).each do |i|
      digit = if i == 11
        chars.max
      else
        chars[...-(11-i)].max
      end

      digits.push(digit)
      chars = chars[chars.index(digit) + 1..]
    end

    s = digits.join
    p "#{s}"
    sum += s.to_i
  end

  p "Sum: #{sum}"
end

main()
