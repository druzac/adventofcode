function parse_problem(inputf)
    current_calories::Vector{Int64} = []
    all_calories::Vector{Vector{Int64}} = []
    for line in eachline(inputf)
        if isempty(line)
            push!(all_calories, current_calories)
            current_calories = []
        else
            push!(current_calories, parse(Int64, line))
        end
    end
    push!(all_calories, current_calories)
    all_calories
end

function problem_one(v)
    maximum(group -> sum(group), v)
end

function problem_two(v)
    sum(sort(sum.(v), rev=true)[1:3])
end
