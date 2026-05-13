#!/usr/bin/env ruby

def maybe_push(joltages, button)
  #p "joltages #{joltages}: trying to push #{button}"
  button.each do |j|
    if joltages[j] == 0
      return false
    end
  end

  button.each do |j|
    joltages[j] -= 1
  end
  true
end

def unpush(joltages, button)
  button.each do |j|
    joltages[j] += 1
  end
end

def choose(joltages, buttons)
  steps = 0
  chosen = []
  m = 0
  loop do
    steps += 1
    #p "top of loop, m=#{m}, buttons.size=#{buttons.size}"
    chosen_size = chosen.size
    (m...(buttons.size)).each do |i|
      if maybe_push(joltages, buttons[i])
        chosen.push(i)
        break
      end
    end

    if chosen_size == chosen.size
      # No forward progress.  Need to backtrack.
      p "backtracking: joltages #{joltages}, chosen #{chosen}"
      i = chosen.pop
      unpush(joltages, buttons[i])
      m = i + 1
      next
    end

    if joltages.count(0) == joltages.size
      p "solution in #{steps} steps and #{chosen.size} button pushes: #{chosen}"
      return chosen.size
    end
  end
end

def main
  f = open("input.txt")
  total = 0
  f.each_line do |line|
    parts = line.split

    buttons = []
    parts.each do |part|
      next if part[0] != '('
      button = part[1...-1].split(',').map { |x| x.to_i }
      buttons.push(button)
    end
    buttons = buttons.sort_by(&:size).reverse
    fail if parts[-1][0] != '{'
    joltages = parts[-1][1...-1].split(',').map { |x| x.to_i }
    p "machine: #{buttons.size} buttons: #{buttons}, and joltages #{joltages}"

    total += choose(joltages, buttons)
  end
  p "Total: #{total}"
end

main()
