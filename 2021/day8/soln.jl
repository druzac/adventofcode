function word_to_bvector(word)
    bv = BitVector(0 for _ in 1:7)
    for char in word
        bv[(char - 'a') + 1] = true
    end
    bv
end

function parse_problem(inputf)
    signal_patterns = Vector{Tuple{Vector{BitVector}, Vector{BitVector}}}()
    for line in readlines(inputf)
        words = split(line)
        all_patterns = word_to_bvector.(words[1:10])
        output_words = words[12:15]
        output_patterns = word_to_bvector.(output_words)
        push!(signal_patterns, (all_patterns, output_patterns))
    end
    return signal_patterns
end

function clear_bits(bv1, bv2)
    map((x, y) -> x && !y, bv1, bv2)
end

function merge_bits(bv1, bv2)
    map((x, y) -> x || y, bv1, bv2)
end

function problem_one(problem)
    cnt_1_3_7_8 = 0
    for subp in problem
        d = Dict{Int64, BitVector}()
        for pattern in subp[1]
            cnt = count(pattern)
            if cnt == 2
                d[1] = pattern
            elseif cnt == 4
                d[4] = pattern
            elseif cnt == 3
                d[7] = pattern
            elseif cnt == 7
                d[8] = pattern
            end
        end
        for output_pattern in subp[2]
            if output_pattern == d[1] || output_pattern == d[4] || output_pattern == d[7] || output_pattern == d[8]
                cnt_1_3_7_8 += 1
            end
        end
    end
    cnt_1_3_7_8
end

function d_to_assoc_list(d::Dict{Int64, BitVector})
    [(v, k) for (k, v) in d]
end

function assoc_lookup(assoc, search_key)
    for (k, v) in assoc
        if k == search_key
            return v
        end
    end
    @show assoc, k
    error("key not found")
end

function get_digit_map(all_patterns)
    d = Dict{Int64, BitVector}()
    five_cnts = Vector{BitVector}()
    six_cnts = Vector{BitVector}()
    for pattern in all_patterns
        cnt = count(pattern)
        if cnt == 2
            d[1] = pattern
        elseif cnt == 4
            d[4] = pattern
        elseif cnt == 3
            d[7] = pattern
        elseif cnt == 7
            d[8] = pattern
        elseif cnt == 5
            push!(five_cnts, pattern)
        elseif cnt == 6
            push!(six_cnts, pattern)
        else
            @show pattern, cnt
            error("unexpected count")
        end
    end
    if length(d) != 4
        error("Didn't find the unique line segment count guys")
    end
    for _ in 1:3
        if count(xor.(five_cnts[1], five_cnts[2])) == 4
            d[3] = pop!(five_cnts)
            break
        else
            five_cnts = circshift(five_cnts, 1)
        end
    end
    if !haskey(d, 3)
        @show five_cnts
        error("3 invariant violated")
    end
    nine = merge_bits(d[3], d[4])
    for five_cnt in five_cnts
        cnt = count(clear_bits(five_cnt, nine))
        if cnt == 1
            d[2] = five_cnt
        elseif cnt == 0
            d[5] = five_cnt
        else
            @show five_cnt, nine, cnt
            error("invariant violated")
        end
    end
    for six_cnt in six_cnts
        if six_cnt == nine
            d[9] = six_cnt
        elseif count(clear_bits(six_cnt, d[5])) == 2
            d[0] = six_cnt
        elseif count(clear_bits(six_cnt, d[3])) == 2
            d[6] = six_cnt
        else
            @show six_cnt, nine, d[5], d[3]
            error("invariant violated for six counts")
        end
    end
    if length(d) != 10
        error("not done")
    end
    d_to_assoc_list(d)
end

function output_patterns_to_number(digit_assoc, output_patterns)
    n = 0
    for output_pattern in output_patterns
        n = n * 10 + assoc_lookup(digit_assoc, output_pattern)
    end
    n
end

function problem_two(problem)
    sum = 0
    for subp in problem
        digit_assoc = get_digit_map(subp[1])
        sum += output_patterns_to_number(digit_assoc, subp[2])
    end
    sum
end
