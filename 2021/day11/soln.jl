function parse_problem(inputf)
    lines = readlines(inputf)
    n_rows = length(lines)
    n_cols = length(lines[1])
    m = Matrix{Int64}(undef, n_rows, n_cols)
    for j in 1:n_cols
        for i in 1:n_rows
            m[i, j] = parse(Int64, lines[i][j])
        end
    end
    m
end

function tick(m)
    flashers = Vector{Tuple{Int64, Int64}}()
    cnt_flashes = 0
    for j = 1:size(m, 2)
        for i = 1:size(m, 1)
            m[i, j] += 1
            if m[i, j] == 10
                push!(flashers, (i, j))
                cnt_flashes += 1
            end
        end
    end
    while !isempty(flashers)
        i, j = pop!(flashers)
        for neighbour_i = i - 1:1:i + 1
            for neighbour_j = j - 1:1:j + 1
                if (neighbour_i >= 1 && neighbour_i <= size(m, 1) && neighbour_j >= 1 && neighbour_j <= size(m, 2) && (neighbour_i, neighbour_j) != (i, j))
                    m[neighbour_i, neighbour_j] += 1
                    if m[neighbour_i, neighbour_j] == 10
                        push!(flashers, (neighbour_i, neighbour_j))
                        cnt_flashes += 1
                    end
                end
            end
        end
    end
    for j = 1:size(m, 2)
        for i = 1:size(m, 1)
            if m[i, j] >= 10
                m[i, j] = 0
            end
        end
    end
    cnt_flashes
end

function problem_one(m)
    num_steps = 100
    # two passes
    # first pass - increment numbers. if while incrementing, the
    # number is 10, then set that octopus to flash too.
    # use a stack for flashing octopodes - vector
    num_flashes = 0
    for k = 1:num_steps
        num_flashes += tick(m)
        @show m
        @show num_flashes
    end
    num_flashes
end

function problem_two(m)
    num_octopodes = length(m)
    step_number = 1
    while true
        num_flashes = tick(m)
        if num_flashes == num_octopodes
            return step_number
        end
        step_number += 1
    end
end

function main(args)
    problem_number = args[1]
    inputf = args[2]

    problem = parse_problem(inputf)
    if problem_number == "1"
        @show problem_one(problem)
    elseif problem_number == "2"
        @show problem_two(problem)
    else
        error("Need to put in 1 or 2")
    end
end

if PROGRAM_FILE != "" && realpath(@__FILE__) == realpath(PROGRAM_FILE)
    main(ARGS)
end
