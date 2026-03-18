#!/usr/bin/env ruby

def main
  total = 0
  a=[]
  f = open("input.txt")
  f.each_line do |line|
    a.push(line.chomp.chars)
  end

  b = a.transpose
  cols_for_number = []
  operator = ''
  b.push([' '])
  b.each do |col|
    p "col is #{col}"
    if col.map { |c| c==' ' }.all?
      p "found a boundary, cols_for_number are #{cols_for_number} and operator is #{operator}"
      vals_for_number = cols_for_number.map { |col| col.join.to_i }
      result = if operator == '+'
        vals_for_number.reduce(:+)
      elsif operator == '*'
        vals_for_number.reduce(:*)
      end
      p "Result: #{result}"
      total += result
      cols_for_number = []
      next
    end

    if col.map { |c| c=='+' }.any?
      operator = '+'
    elsif col.map { |c| c=='*' }.any?
      operator = '*'
    end

    cols_for_number.push(col.reject { |c| c=='+' || c=='*' || c==' '} )
  end
  p "Total: #{total}"
end

main()
