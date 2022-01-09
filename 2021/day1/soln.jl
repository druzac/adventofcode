import DelimitedFiles

function parse_problem(inputf)
    DelimitedFiles.readdlm(inputf, Int64)
end

function problem_one(v)
    firsts = @view v[1: end - 1]
    seconds = @view v[2: end]
    count(firsts .< seconds)
end

function problem_two(v)
    v0 = @view v[1: end - 2]
    v1 = @view v[2: end - 1]
    v2 = @view v[3: end]

    windows = v0 + v1 + v2

    firsts = @view windows[1: end - 1]
    seconds = @view windows[2: end]
    count(firsts .< seconds)
end
