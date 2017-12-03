(0..($*.pop.to_i)).each do |i|
    j = Math.sqrt(i).round
    k = (j ** 2 - i).abs - j
    p = [k, -k].map {|l| (l + j ** 2 - i - (j % 2)) * 0.5 * (-1) ** j}.map(&:to_i)
    puts "p => #{p[0]}, #{p[1]}"
end
