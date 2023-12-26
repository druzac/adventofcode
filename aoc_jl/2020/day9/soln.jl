function parse_problem(inputf)
    [parse(Int64, line) for line in eachline(inputf)]
end

function increment_count!(d::Dict{Int64, Int64}, n)
    curr = get!(d, n, 0)
    d[n] = curr + 1
end

function decrement_count!(d::Dict{Int64, Int64}, n)
    curr = get(d, n, nothing)
    if !isnothing(curr)
        if curr <= 0
            error("oops: count too low: $d, $n")
        elseif curr == 1
            delete!(d, n)
        else
            d[n] = curr - 1
        end
    end
end

function prefix_achieves_next(v::Vector{Int64}, prefix_length)
    if prefix_length <= 0
        error("prefix length too short: $prefix_length")
    end
    d = Dict{Int64, Int64}()
    for n in v[1:prefix_length]
        increment_count!(d, n)
    end
    for (idx, n) in Iterators.drop(enumerate(v), prefix_length)
        found = any(summand * 2 != n && haskey(d, n - summand)
                    for summand in v[idx - prefix_length:idx - 1])
        if !found
            return n
        end
        increment_count!(d, n)
        decrement_count!(d, v[idx - prefix_length])
    end
    error("not found!")
end

function problem_one(v::Vector{Int64})
    prefix_achieves_next(v, 25)
end

function problem_two(v::Vector{Int64})
    target = prefix_achieves_next(v, 25)
    i, j = 1, 1
    curr = v[1]
    while j <= length(v)
        if curr == target
            return reduce(+, extrema(v[i:j]))
        elseif curr < target
            j += 1
            if j <= length(v)
                curr += v[j]
            end
        else
            if i < j
                curr -= v[i]
                i += 1
            else
                i += 1
                j += 1
                if j < length(v)
                    curr = v[i]
                end
            end
        end
    end
    error("not found!")
end
