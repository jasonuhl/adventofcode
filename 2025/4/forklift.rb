#!/usr/bin/env ruby

def count_neighbors_on_line(a, i, j, include_self: false)
  return 0 if i < 0
  return 0 if i >= a.size

  #i %= a.size
  #j %= a[i].size

  n = 0
  n += 1 if j > 0 and a[i][j-1] == '@'
  n += 1 if include_self and a[i][j] == '@'
  n += 1 if j < a[i].size and a[i][j+1] == '@'

  n
end

def count_neighbors(a, i, j)
  n = 0
  n += count_neighbors_on_line(a, i-1, j, include_self: true)
  n += count_neighbors_on_line(a, i, j, include_self: false)
  n += count_neighbors_on_line(a, i+1, j, include_self: true)

  n
end

def doit(a)
  removed = []
  sum = 0
  for i in 0...a.size
    for j in 0...a[i].size
      next if a[i][j] != '@'

      n = count_neighbors(a, i, j)
      p "#{i} #{j} has #{n}"
      if n < 4
        sum += 1
        a[i][j] = 'x'
        #removed.push([i, j])
      end
    end
  end

#  removed.each do |i,j|
#    a[i][j] = 'x'
#  end

  sum
end

def main
  f = open("input.txt")
  sum = 0
  a = []
  f.each_line do |line|
    chars = line.chomp.chars
    a.push(chars)
  end

  while true
    n = doit(a)
    sum += n
    break if n == 0
  end

  p "Sum: #{sum}"
end

main()
