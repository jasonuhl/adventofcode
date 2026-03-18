#!/usr/bin/env ruby

def pretty_print(g)
  for row in g
    next if row.nil?
    print '> '
    puts row.map { |c| c.nil? ? ' ' : c }.join[1000..]
  end
end

def main
  totalvalue = 0
  f = open("input.txt")
  a = []
  while(!f.eof? && line = f.readline)
    a.push(line.chomp.split(' '))
  end

  #p a

  marks = 1
  pos_x = 1000
  pos_y = 1000
  g = []
  g[1000] = []
  g[1000][1000] = '#'

  leftmost = 9999

  turtle_chars = {'U' => '^', 'R' => '>', 'D' => 'v', 'L' => '<'}

  for step in a
    #p "pos: #{pos_x},#{pos_y}"
    if step[0] == 'R'
      new_pos_x = pos_x + step[1].to_i
      new_pos_y = pos_y
    elsif step[0] == 'L'
      new_pos_x = pos_x - step[1].to_i
      new_pos_y = pos_y
    elsif step[0] == 'D'
      new_pos_x = pos_x
      new_pos_y = pos_y + step[1].to_i
    elsif step[0] == 'U'
      new_pos_x = pos_x
      new_pos_y = pos_y - step[1].to_i
    end

    if pos_x != new_pos_x
      (pos_x..new_pos_x).step(pos_x < new_pos_x ? 1 : -1).each do |x|
        if g[pos_y][x].nil?
          marks += 1
        end
        if g[pos_y][x].nil?
          g[pos_y][x] = turtle_chars[step[0]]
        end
        leftmost = x if x < leftmost
      end
    end
      if pos_y != new_pos_y
      (pos_y..new_pos_y).step(pos_y < new_pos_y ? 1 : -1).each do |y|
        if g[y].nil?
          g[y] = []
        end
        if g[y][pos_x].nil?
          marks += 1
        end
        g[y][pos_x] = turtle_chars[step[0]]
        leftmost = pos_x if pos_x < leftmost
      end
    end

    pos_x = new_pos_x
    pos_y = new_pos_y
  end

  p "marks before infill: #{marks}"
  p "leftmost is #{leftmost}"

  pretty_print(g)

  # infill
  p "g size is #{g.size}"
  for y in 0...g.size
    next if g[y].nil?
    pendown = false
    for x in 0...g[y].size
      if g[y][x] == '^'
        pendown = true
      elsif g[y][x] == 'v'
        pendown = false
      elsif pendown && g[y][x].nil?
        g[y][x] = 'f'
        marks += 1
      end
    end
    p "cumulative marks for row #{y}: #{marks}"
  end

  p "after infill"
  pretty_print(g)

  p "g size is #{g.size}"

  total = 0
  for row in g
    if row.nil?
      next
    end
    for col in row
      if col == '^' || col == '>' || col == 'v' || col == '<' || col == 'f'
        total += 1
      end
    end
  end
  p total
  p "marks after infill: #{marks}"
end

main()
