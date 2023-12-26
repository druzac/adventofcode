
function get_displacement(command)
    op, val = split(command)
    n = tryparse(Int64, val)
    if op == "forward"
        return [n; 0]
    elseif op == "up"
        return [0; -n]
    elseif op == "down"
        return [0; n]
    else
        error("unrecognized op")
    end
end

function parse_problem(inputf)
    inputf
end

function problem_one(input)
    position = [0; 0]
    for line in eachline(input)
        position += get_displacement(line)
    end
    position[1] * position[2]
end

function get_displacement(command, aim)
    op, val = split(command)
    n = tryparse(Int64, val)
    if op == "forward"
        return [n; aim * n; 0]
    elseif op == "up"
        return [0; 0; -n]
    elseif op == "down"
        return [0; 0; n]
    else
        error("unrecognized op")
    end
end

function problem_two(input)
    # horizontal; depth; aim
    position = [0; 0; 0]
    for line in eachline(input)
        position += get_displacement(line, position[3])
    end
    position[1] * position[2]
end
