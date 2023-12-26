function parse_problem(inputf)
    function parse_contained_bags(after_contains)
        if after_contains[1:2] != "no"
            contained_bags = Vector{Tuple{Int64, String}}()
            contained_bags_phrases = split(after_contains, ", ")
            for contained_bags_phrase in contained_bags_phrases
                words = split(contained_bags_phrase)
                if length(words) != 4
                    error("strange bag list: $(contained_bags_phrase)")
                end
                num = parse(Int64, words[1])
                bag_name = join(words[2:3], ' ')
                push!(contained_bags, (num, bag_name))
            end
            contained_bags
        else
            []
        end
    end
    
    open(inputf, "r") do io
        bags = Vector{Tuple{String, Vector{Tuple{Int64, String}}}}()
        for line in eachline(io)
            words = split(line)
            containing_bag_name = join(words[1:2], ' ')
            contained_bags = parse_contained_bags(join(words[5:end], ' '))
            push!(bags, (containing_bag_name, contained_bags))
        end
        bags
    end
end

function problem_one(bags)
    bag_to_containing_bags = Dict{String, Vector{String}}()
    for (containing_bag, contained_bags) in bags
        for (n, contained_bag) in contained_bags
            curr = get!(bag_to_containing_bags, contained_bag, [])
            push!(curr, containing_bag)
        end
    end
    solution = Set{String}()
    to_explore = ["shiny gold"]
    while !isempty(to_explore)
        current_bag = pop!(to_explore)
        for containing_bag in get(bag_to_containing_bags, current_bag, [])
            if !(containing_bag in solution)
                push!(solution, containing_bag)
                push!(to_explore, containing_bag)
            end
        end
    end
    length(solution)
end

function problem_two(bags)
    bag_to_contained_bags = Dict{String, Vector{Tuple{Int64, String}}}()
    for (containing_bag, contained_bags) in bags
        bag_to_contained_bags[containing_bag] = contained_bags
    end
    to_explore = [(1, "shiny gold")]
    bag_count = 0
    while !isempty(to_explore)
        multiplier, bag_name = pop!(to_explore)
        for (count, contained_bag) in get(bag_to_contained_bags, bag_name, [])
            total_count = count * multiplier
            bag_count += total_count
            push!(to_explore, (total_count, contained_bag))
        end
    end
    bag_count
end
