
NEW_LANTERNFISH_START = 9
POST_BIRTH_LANTERNFISH = 7

# array of lanternfish
function tick(lanternfish)
    births = lanternfish[1]
    new_lanternfish = zeros(Int128, size(lanternfish))
    new_lanternfish[NEW_LANTERNFISH_START] += births
    new_lanternfish[POST_BIRTH_LANTERNFISH] += births
    rotated = [@view lanternfish[2:end]; 0]
    new_lanternfish + rotated
end

function parse_problem(inputf)
    line = readline(inputf)
    raw_counts = map(x -> parse(Int128, x), split(line, ',')) .+ 1
    arr_length = max(NEW_LANTERNFISH_START, maximum(raw_counts))
    @show arr_length
    lanternfish = zeros(Int128, arr_length)
    for count in raw_counts
        lanternfish[count] += 1
    end
    @show lanternfish
end

function problem_one(inputf)
    # lanternfish = [0, 1, 1, 2, 1, 0, 0, 0, 0]
    lanternfish = parse_problem(inputf)
    for i in 1:80
        lanternfish = tick(lanternfish)
    end
    sum(lanternfish)
end

function problem_two(inputf)
    lanternfish = parse_problem(inputf)
    for i in 1:256
        lanternfish = tick(lanternfish)
    end
    sum(lanternfish)
end

function main(args)
    problem = args[1]
    input = args[2]

    if problem == "1"
        @show problem_one(input)
    elseif problem == "2"
        @show problem_two(input)
    end
end

if PROGRAM_FILE != "" && realpath(@__FILE__) == realpath(PROGRAM_FILE)
    main(ARGS)
end
