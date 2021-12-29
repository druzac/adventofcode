function parse_problem(inputf)
    line = readline(inputf)
    map(x -> parse(Int64, x), split(line, ','))
end

function fuel_cost(val, arr)
    sum(abs.(val .- arr))
end

function problem_one(problem)
    sorted_p = sort(problem)
    # @show sorted_p
    if length(sorted_p) == 0
        error("Empty input array.")
    elseif length(sorted_p) % 2 == 0
        idx = length(sorted_p) ÷ 2
        val_one, val_two = sorted_p[idx], sorted_p[idx + 1]
        # @show val_one, val_two
        return min(fuel_cost(val_one, sorted_p),
                   fuel_cost(val_two, sorted_p))
    else
        val = sorted_p[(length + 1) ÷ 2]
        # @show val
        return fuel_cost(val, sorted_p)
    end
end

function quadratic_fuel_cost(val, arr)
    sum(map(x -> (x * (x + 1)) ÷ 2, abs.(val .- arr)))
end

function problem_two(problem)
    if length(problem) == 0
        error("Empty input array.")
    end
    first = minimum(problem)
    last = maximum(problem)
    curr_min = quadratic_fuel_cost(first, problem)
    for curr in first + 1:last
        curr_min = min(curr_min, quadratic_fuel_cost(curr, problem))
    end
    return curr_min
end

function main(args)
    problem_number = args[1]
    inputf = args[2]

    problem = parse_problem(inputf)
    # @show problem
    # @show inputf
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
