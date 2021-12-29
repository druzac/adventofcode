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

function main(args)
    input = args[1]
    # horizontal; depth; aim
    position = [0; 0; 0]
    for line in eachline(input)
        position += get_displacement(line, position[3])
    end
    @show position
    @show position[1] * position[2]
end

main(ARGS)
