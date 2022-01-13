function parse_problem(inputf)
    lines = readlines(inputf)
    map(x -> parse(Int64, x), lines)
end

function problem_one(problem)
    sort!(problem)
    idx1 = 1
    idx2 = length(problem)
    while true
        curr_sum = problem[idx1] + problem[idx2]
        if curr_sum == 2020
            return problem[idx1] * problem[idx2]
        elseif curr_sum < 2020
            idx1 += 1
        else
            idx2 -= 1
        end
    end
end

# 3 numbers that sum to 2020
function problem_two(problem)
    # x + y == 2020 - z
    subs = 2020 .- problem
    d = Dict{Int64, Int64}()
    for (idx, entry) in enumerate(subs)
        d[entry] = idx
    end
    for i in 1:length(problem)
        for j in i + 1:length(problem)
            val = problem[i] + problem[j]
            hit_idx = get(d, val, nothing)
            if hit_idx != nothing && hit_idx != i && hit_idx != j
                return problem[i] * problem[j] * problem[hit_idx]
            end
        end
    end
end
