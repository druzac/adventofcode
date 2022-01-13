function parse_problem(inputf)
    open(inputf, "r") do io
        groups = Vector{Vector{String}}()
        group = Vector{String}()
        for line in eachline(io)
            if line == ""
                if !isempty(group)
                    push!(groups, group)
                    group = Vector{String}()
                end
            else
                push!(group, line)
            end
        end
        if !isempty(group)
            push!(groups, group)
        end
        groups
    end
end

function problem_one(groups)
    sum(length(reduce(union, map(Set{Char}, group))) for group in groups)
end

function problem_two(groups)
    sum(length(reduce(intersect, map(Set{Char}, group))) for group in groups)
end
