#!/usr/bin/env ruby

def find_reflection(a, direction)
  for rowidx in 0...a.size - 1
    if a[rowidx] == a[rowidx + 1]
      mirrortop = rowidx
      mirrorsize = 1
      while mirrortop > 0 && a[mirrortop - 1] == a[rowidx + mirrorsize + 1]
        mirrortop -= 1
        mirrorsize += 1
      end

      # It only counts if it goes up to an edge.
      if mirrortop == 0 || rowidx + mirrorsize + 1 == a.size
        puts "found #{direction} reflection: #{rowidx} - #{rowidx + 1} of size #{mirrorsize}"
        val = rowidx + 1
        val *= 100 if direction == "horizontal"
        puts "value: #{val}"
        return val
      end
    end
  end
  0
end

def row_edit_distance(a, b)
  fail if a.size != b.size

  edit_distance = 0
  for i in 0...a.size
    if a[i] != b[i]
      edit_distance += 1
    end
  end

  edit_distance
end

def find_horizontal_reflection(a)
  find_reflection(a, "horizontal")
end

def find_smudged_reflection(a)
  for rowidx in 0...a.size - 1
    mirrorsize = 0
    mirror_edit_distance = 0
    x = 0
    while rowidx - mirrorsize >= 0 && rowidx + mirrorsize + 1 < a.size
      x = row_edit_distance(a[rowidx - mirrorsize], a[rowidx + mirrorsize + 1])
      break if mirror_edit_distance + x > 1
      mirror_edit_distance += x
      mirrorsize += 1
    end

    next if mirrorsize == 0
    next if mirror_edit_distance != 1
    # It only counts if it goes up to an edge.
    if rowidx - mirrorsize == -1 || rowidx + mirrorsize + 1 == a.size
      #p "found smudged reflection at #{rowidx} of size #{mirrorsize}"
      return rowidx
    end
  end
  nil
end

def find_smudged_horizontal_reflection(a)
  x = find_smudged_reflection(a)
  if x
    puts "found smudged horizontal reflection, value: #{100 * (x + 1)}"
    return 100 * (x + 1)
  end
  0
end

def find_vertical_reflection(a)
  # rotate the whole array
  cols = a[0].zip(*a[1..])

  find_reflection(cols, "vertical")
end

def find_smudged_vertical_reflection(a)
  # rotate the whole array
  cols = a[0].zip(*a[1..])

  x = find_smudged_reflection(cols)
  if x
    puts "found smudged vertical reflection, value: #{x + 1}"
    return x + 1
  end
  0
end

def get_pattern(f)
  a = []
  while(!f.eof? && line = f.readline)
    break if line == "\n"
    a.push(line.chomp.chars)
  end

  a
end

def main
  totalvalue = 0
  f = open("input.txt")
  aidx = 0
  while(a = get_pattern(f); !a.empty?)
    puts "pattern #{aidx}"
    #totalvalue += find_horizontal_reflection(a)
    #totalvalue += find_vertical_reflection(a)
    totalvalue += find_smudged_horizontal_reflection(a)
    totalvalue += find_smudged_vertical_reflection(a)

    aidx += 1
  end

  puts "total value: #{totalvalue}"
end

main()
