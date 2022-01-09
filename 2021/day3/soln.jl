function ctobool(char)
    if char == '1'
        return true
    elseif char == '0'
        return false
    else
        error("can't parse string")
    end
end

function bitarr_to_int(arr)
    return sum(arr .* (2 .^ collect(length(arr)-1:-1:0)))
end

function lines_to_bitmatrix(lines)
    nrows = length(lines)
    ncols = length(lines[1])

    bits = BitArray(undef, nrows, ncols)
    for i in 1:nrows
        for j in 1:ncols
            bits[i, j] = ctobool(lines[i][j])
        end
    end
    bits
end

function parse_problem(inputf)
    lines_to_bitmatrix(readlines(inputf))
end

function problem_one(bits)
    ncols = size(bits, 2)
    gamma = BitArray(undef, ncols)
    for j in 1:ncols
        col = @view bits[1:end, j]
        num_ones = count(col)
        num_zeros = length(col) - num_ones
        if num_ones > num_zeros
            gamma[j] = true
        else
            gamma[j] = false
        end
    end
    epsilon = .! gamma

    int_gamma = bitarr_to_int(gamma)
    int_epsilon = bitarr_to_int(epsilon)
    int_gamma * int_epsilon
end

# the criteria takes:
#   number of 1s
#   number of 0s
#   returns which to keep - rows w/ a 1 or a 0 in that position
# f should be a function 

function oxygen_bit_criteria(num_ones, num_zeros)
    if num_ones >= num_zeros
        true
    else
        false
    end
end

function co2_scrubber_bit_criteria(num_ones, num_zeros)
    if num_zeros <= num_ones
        false
    else
        true
    end
end

function apply_criteria(bitmatrix, bit_criteria)
    apply_criteria(bitmatrix, bit_criteria, 1)
end

function apply_criteria(bitmatrix, bit_criteria, curr_col)
    num_cols = size(bitmatrix, 2)
    if num_cols < curr_col || size(bitmatrix, 1) == 1
        return bitmatrix
    end
    col = @view bitmatrix[:, curr_col]
    num_ones = count(col)
    num_zeros = length(col) - num_ones
    value_to_keep = bit_criteria(num_ones, num_zeros)
    rows_to_keep = col .== value_to_keep
    filtered = @view bitmatrix[rows_to_keep, :]
    apply_criteria(filtered, bit_criteria, curr_col + 1)
end

function problem_two(bits)
    oxygen_val = apply_criteria(bits, oxygen_bit_criteria)
    co2_scrubber_val = apply_criteria(bits, co2_scrubber_bit_criteria)
    bitarr_to_int(oxygen_val') * bitarr_to_int(co2_scrubber_val')
end
