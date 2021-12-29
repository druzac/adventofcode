
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

function main(args)
    input = args[1]
    @show input

    position = [0; 0]
    # prev = nothing
    # count = 0
    for line in eachline(input)
        # command, val = split(line)
        position += get_displacement(line)
        # position = position + get_displacement(line)
        # if (prev != nothing && prev < n)
        #     count += 1
        # end
        # @show prev, n
        # prev = n
    end
    @show position
    @show position[1] * position[2]
end

main(ARGS)
