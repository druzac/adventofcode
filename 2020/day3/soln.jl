function parse_problem(inputf)
    function parse_char(char)
        if char == '#'
            1
        elseif char == '.'
            0
        else
            error("invalid character: ", char)
        end
    end
    forest = Vector{BitVector}()
    open(inputf, "r") do io
        for line in eachline(io)
            push!(forest, map(parse_char, collect(line)))
        end
    end
    forest
end

function move(forest, current, velocity)
    new_pos = current + velocity
    if new_pos[1] > length(forest[1])
        new_pos[1] = new_pos[1] % length(forest[1])
        if new_pos[1] == 0
            new_pos[1] == length(forest[1])
        end
    end
    new_pos
end

function check_slope(forest, slope)
    num_trees = 0
    current_pos = [1, 1]
    while current_pos[2] <= length(forest)
        if forest[current_pos[2]][current_pos[1]] == 1
            num_trees += 1
        end
        current_pos = move(forest, current_pos, slope)
    end
    num_trees
end

function problem_one(forest)
    check_slope(forest, [3,1])
end

function problem_two(forest)
    prod((check_slope(forest, slope) for slope in [[1, 1],
                                                   [3, 1],
                                                   [5, 1],
                                                   [7, 1],
                                                   [1, 2]]))
end
