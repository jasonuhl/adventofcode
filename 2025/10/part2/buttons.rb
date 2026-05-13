#!/usr/bin/env ruby

# Inspired by:
# https://old.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
# https://aoc.winslowjosiah.com/solutions/2025/day/10/

def compute_combo_parities(n, joltages, buttons, paritycache, chosen=[])
  if n == 0
    parities = joltages.map { '.' }.join
    chosen.each do |button|
      button.each do |i|
        if parities[i] == '.'
          parities[i] = '#'
        elsif parities[i] == '#'
          parities[i] = '.'
        else
          fail "wtf"
        end
      end
    end

    paritycache[parities] ||= []
    paritycache[parities].push(chosen)

    return
  end

  combos = []
  (0..(buttons.size - n)).each do |i|
    compute_combo_parities(
      n - 1, joltages, buttons[i+1..], paritycache, chosen + [buttons[i]]
    )
  end
  return
end

def choose(joltages, buttons, paritycache)
  return 0 if joltages.count(0) == joltages.size

  parities = joltages.map { |j| j.odd? ? '#' : '.' }.join
  return 999999 if !paritycache.key?(parities)

  combos = []
  paritycache[parities].each do |combo|
    new_joltages = joltages.clone
    combo.each do |button|
      button.each do |i|
        new_joltages[i] -= 1
      end
    end
    next if new_joltages.any? { |j| j < 0 }
    combos.push([combo.size, new_joltages])
  end
  return 999999 if combos.empty?

  bestv = nil
  combos.each do |combo|
    v, new_joltages = combo
    joltages = new_joltages.map { |x| x / 2 }
    v2 = choose(joltages, buttons, paritycache) * 2
    bestv = v + v2 if bestv.nil?
    bestv = [bestv, v + v2].min
  end
  bestv
end

def main
  f = open("input.txt")
  total_buttons = 0
  f.each_line do |line|
    parts = line.split

    buttons = []
    parts.each do |part|
      next if part[0] != '('
      button = part[1...-1].split(',').map { |x| x.to_i }
      buttons.push(button)
    end
    fail if parts[-1][0] != '{'
    joltages = parts[-1][1...-1].split(',').map { |x| x.to_i }
    p "machine: #{buttons.size} buttons: #{buttons}, and joltages #{joltages}"

    paritycache = {}
    (0..(buttons.size)).each do |n|
      compute_combo_parities(n, joltages, buttons, paritycache)
    end
    #p "made #{paritycache.keys.size} keys in paritycache"

    v = choose(joltages, buttons, paritycache)
    p "machine: #{v} pushes"
    total_buttons += v
  end

  p "Total: #{total_buttons}"
end

main()
