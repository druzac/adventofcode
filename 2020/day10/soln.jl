function parse_problem(inputf)
    [parse(Int64, line) for line in eachline(inputf)]
end

function augment_and_sort(adapters::Vector{Int64})
    sort([adapters; 0; maximum(adapters) + 3])
end

function problem_one(adapters::Vector{Int64})
    adapters_augmented = augment_and_sort(adapters)
    diffs = adapters_augmented[2:end] - adapters_augmented[1:end-1]
    counted_diffs = [0, 0, 0]
    for diff in diffs
        if diff < 1 || diff > 3
            error("diff too large! $diff")
        end
        counted_diffs[diff] += 1
    end
    counted_diffs[3] * counted_diffs[1]
end

function problem_two(adapters::Vector{Int64})
    adapters_augmented = augment_and_sort(adapters)
    d = zeros(UInt64, length(adapters_augmented))
    d[end] = 1
    d[end - 1] = 1
    for i in length(d) - 2:-1:1
        j = i + 1
        while j <= length(d) && adapters_augmented[j] <= adapters_augmented[i] + 3
            d[i] += d[j]
            j += 1
        end
    end
    d[1]
end
