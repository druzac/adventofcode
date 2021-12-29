import LinearAlgebra

function permutations()
    arrangements = [[1, 2, 3], [1, 3, 2],
                    [2, 1, 3], [2, 3, 1],
                    [3, 1, 2], [3, 2, 1]]
    id = [1 0 0; 0 1 0; 0 0 1]
    permutation_matrices = Vector{Matrix{Int64}}()
    for arrange in arrangements
        push!(permutation_matrices, id[:, arrange])
    end
    permutation_matrices
    sign_matrices = Vector{Matrix{Int64}}()
    for i in 0:7
        new_matrix = zeros(Int64, 3, 3)
        for bit in 0:2
            val = ((1 << bit) & i) > 0 ? 1 : -1
            new_matrix[bit + 1, bit + 1] = val
        end
        push!(sign_matrices, new_matrix)
    end
    result_matrices = Vector{Matrix{Int64}}()
    for perm in permutation_matrices
        for sgn in sign_matrices
            maybe = perm * sgn
            if LinearAlgebra.det(maybe) > 0
                push!(result_matrices, maybe)
            end
        end
    end
    result_matrices
end

const PERMS = permutations()

function parse_problem(inputf)
    open(inputf, "r") do io
        beacons = Dict{Int64, Vector{Vector{Int64}}}()
        title_row = readline(io)
        while title_row != ""
            scanner_num = parse(Int64, split(title_row)[3])
            if haskey(beacons, scanner_num)
                error("repeated scanner: $(scanner_num), $(title_row), $(beacons)")
            end
            beacons[scanner_num] = []
            beacon_row = readline(io)
            while beacon_row != ""
                beacon_coords = map(x -> parse(Int64, x), split(beacon_row, ','))
                push!(beacons[scanner_num], beacon_coords)
                beacon_row = readline(io)
            end
            title_row = readline(io)
        end
        beacons
    end
end

# apply this to translated beacons.
function compare_and_find_match(s1, s2, perms)
    st1 = Set(s1)
    for perm in perms
        permuted_s2 = [perm * beacon for beacon in s2]
        if length(intersect(st1, permuted_s2)) >= 12
            return (permuted_s2, perm)
        end
    end
    return ([], Matrix{Int64}(undef, 3, 3))
end

function find_overlap(abeacons, bbeacons)
    for abeacon in abeacons
        a_transformed = [other - abeacon for other in abeacons]
        for bbeacon in bbeacons
            b_transformed = [other - bbeacon for other in bbeacons]
            b_rotated, perm = compare_and_find_match(a_transformed, b_transformed, PERMS)
            if !isempty(b_rotated)
                return ([beacon + abeacon for beacon in b_rotated], perm * (-bbeacon) + abeacon)
            end
        end
    end
    ([], [])
end

function get_first_scanner(scanners)
    for i in 0:100
        if haskey(scanners, i)
            return (pop!(scanners, i), i)
        end
    end
    error("didn't find a first scanner")
end

function dedup_beacons(scanners)
    first_beacons, first_scanner_num = get_first_scanner(scanners)
    normalized_beacons = Set(first_beacons)
    scanner_positions = Dict(first_scanner_num => zeros(Int64, 3))
    while !isempty(scanners)
        found_match = false
        for (scanner_num, beacons) in scanners
            transformed_beacons, scanner_pos = find_overlap(normalized_beacons, beacons)
            if !isempty(transformed_beacons)
                normalized_beacons = union(normalized_beacons, transformed_beacons)
                pop!(scanners, scanner_num)
                scanner_positions[scanner_num] = scanner_pos
                found_match = true
            end
        end
        if !found_match
            error("never found a match!")
        end
    end
    (normalized_beacons, scanner_positions)
end

function problem_one(scanners)
    length(dedup_beacons(scanners)[1])
end

function manhattan_distance(v1, v2)
    sum(abs.(v1 - v2))
end

function problem_two(scanners)
    scanners = dedup_beacons(scanners)[2]
    scanner_positions = [scanner_pos for scanner_pos in values(scanners)]
    maximum(manhattan_distance(scanner_positions[i], scanner_positions[j])
            for i in 1:length(scanner_positions)
                for j in i + 1:length(scanner_positions))
end

function main(args)
    problem_number = args[1]
    inputf = args[2]

    problem = parse_problem(inputf)
    if problem_number == "1"
        @show problem_one(problem)
    elseif problem_number == "2"
        @show problem_two(problem)
    else
        error("Need to put in 1 or 2")
    end
end

if PROGRAM_FILE != "" && realpath(@__FILE__) == realpath(PROGRAM_FILE)
    main(ARGS)
end
