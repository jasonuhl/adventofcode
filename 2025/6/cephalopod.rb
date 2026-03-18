#!/usr/bin/env ruby

def parse_numbers(a)
  nums = []
  len = a[0].length
  num = []
  operator = ''
  (0...len).each do |col|
    colvalues = a.map { |line| line[col] }.reject { |x| x=='*' || x=='+' }
    p "colvalues for #{col} are #{colvalues}"
    num.push(colvalues)
    if a.map { |line| line[col] == '+' }.any?
      operator = '+'
      p "col #{col}: found operator #{operator}"
    elsif a.map { |line| line[col] == '*' }.any?
      operator = '*'
      p "col #{col}: found operator #{operator}"
    end

    if (col == len - 1) || (a.map { |line| line[col]== ' ' }.all?)
      p "col #{col} is empty"
      num.push(operator)
      nums.push(num)
      num = []
      operator = ''
    end
  end
  nums.each do |num|
    p "a num is #{num}"
  end
  nums
end

def main
  a=[]
  f = open("input.txt")
  f.each_line do |line|
    a.push(line)
  end

  nums = parse_numbers(a)
  total = 0
  nums.each do |num|
    ints = num[...-1].map { |elem| elem.join.to_i }.reject { |x| x==0 }
    operator = num[-1]
    p "ints are #{ints} and operator is #{operator}"
    result = if operator == '+'
      ints.reduce(:+)
    elsif operator == '*'
      ints.reduce(:*)
    else
      fail 'bad operator'
    end
    p result
    total += result
  end
  p "Total: #{total}"
end

main()
