#!/usr/bin/env ruby

def push_buttons(lights, buttons)
  lights = lights.clone
  buttons.each do |button|
    button.each do |i|
      if lights[i] == '.'
        lights[i] = '#'
      elsif lights[i] == '#'
        lights[i] = '.'
      else
        fail "invalid light"
      end
    end
  end
  lights.count('.') == lights.size
end

def choose(n, lights, buttons, chosen=[])
  p "        choosing #{n} from #{buttons}, chosen=#{chosen}"
  if n == 0
    if push_buttons(lights, chosen)
      p "        solution! #{chosen}"
      return chosen.size
    end
    return false
  end

  (0..(buttons.size - n)).each do |i|
    if v = choose(n-1, lights, buttons[i+1..], chosen+[buttons[i]])
      return v
    end
  end
  false
end

def main
  f = open("input.txt")
  total_buttons = 0
  f.each_line do |line|
    parts = line.split

    lights = parts[0]
    fail if lights[0] != '[' || lights[-1] != ']'
    lights = lights[1...-1]

    buttons = []
    parts.each do |part|
      next if part[0] != '('
      button = part[1...-1].split(',').map { |x| x.to_i }
      buttons.push(button)
    end
    p "machine: lights #{lights} and #{buttons.size} buttons: #{buttons}"

    (1...(buttons.size)).each do |n|
      p "    trying #{n} buttons"
      if v = choose(n, lights, buttons)
        total_buttons += v
        break
      end
    end
  end
  p "Total buttons: #{total_buttons}"
end

main()
