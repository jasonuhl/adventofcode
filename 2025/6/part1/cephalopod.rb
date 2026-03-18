#!/usr/bin/env ruby

def main
  a=[]
  f = open("input.txt")
  f.each_line do |line|
    a.push(line.chomp.split)
  end
  p a
  b = a.transpose

  total = 0
  b.map do |arr|
    linetotal = if arr[-1] == '*'
      arr[0..-2].map { |x| x.to_i }.reduce(:*)
    elsif arr[-1] == '+'
      arr[0..-2].map { |x| x.to_i }.reduce(:+)
    else
      fail 'bad operator'
    end
    p linetotal
    total += linetotal
  end
  p "Total: #{total}"
end

main()
