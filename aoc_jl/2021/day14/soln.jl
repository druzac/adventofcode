struct Polymer
    template::String
    rules::Dict{Tuple{Char, Char}, Char}
end

function parse_problem(inputf)
    open(inputf, "r") do io
        template = readline(io)
        readline(io)

        d = Dict{Tuple{Char, Char}, Char}()
        for line in eachline(io)
            pair, product = strip.(split(line, "->"))
            d[(pair[1], pair[2])] = product[1]
        end
        Polymer(template, d)
    end
end

function indicator_i(i, n)
    a = zeros(Int64, n)
    a[i] = 1
    a
end

function char_to_vec(problem)
    all_chars = Vector{Char}()
    s1 = Set(char for char in problem.template)
    s2 = Set(pair[1] for pair in keys(problem.rules))
    s3 = Set(val for val in values(problem.rules))
    all_chars = union(s1, s2, s3)
    Dict((val, indicator_i(index, length(all_chars))) for (index, val) in enumerate(sort([char for char in all_chars])))
end

function dp(problem, iterations)
    c2v = char_to_vec(problem)
    # rules_to_arrays is a dict from rules to a vector of vectors. the ith index of the vector
    # is the sum of all products after ith reactions starting from the key rule.

    # initialize dp lookup array with values for a single reaction for all rules.
    rules_to_arrays = Dict((rule, [c2v[product]]) for (rule, product) in problem.rules)
    # compute dp lookup array for all rules up to ith iteration.
    for i in 2:iterations
        for (rule, product) in problem.rules
            one_step = c2v[product]
            left_child = rules_to_arrays[(rule[1], product)][i - 1]
            right_child = rules_to_arrays[(product, rule[2])][i - 1]
            push!(rules_to_arrays[rule], one_step + left_child + right_child)
        end
    end
    # count molecules in template
    total = sum(c2v[char] for char in problem.template)
    # for each pair in template, add all products across all iterations
    # using last index of dp lookup array
    for i in 1:length(problem.template) - 1
        pair = (problem.template[i], problem.template[i + 1])
        total += rules_to_arrays[pair][end]
    end
    small, big = extrema(total)
    big - small
end

function problem_one(problem)
    dp(problem, 10)
end

function problem_two(problem)
    dp(problem, 40)
end
