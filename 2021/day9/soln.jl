
function parse_problem(inputf)
    all_lines = readlines(inputf)
    cols = length(all_lines[1])
    rows = length(all_lines)
    heatmap = Matrix{Int8}(undef, rows, cols)
    for i in 1:rows
        for j in 1:cols
            heatmap[i, j] = parse(Int8, all_lines[i][j])
        end
    end
    heatmap
end

function try_compare(val, row, col, heatmap)
    try
        val < heatmap[row, col]
    catch e
        true
    end
end

function low_point(row, col, heatmap)
    val = heatmap[row, col]
    (try_compare(val, row + 1, col, heatmap) &&
        try_compare(val, row - 1, col, heatmap) &&
        try_compare(val, row, col + 1, heatmap) &&
        try_compare(val, row, col - 1, heatmap))
end

function get_low_points(heatmap)
    low_points = Vector{Tuple{Int64, Int64}}()
    rows, cols = size(heatmap)
    for j in 1:cols
        for i in 1:rows
            if low_point(i, j, heatmap)
                # @show i, j, heatmap[i, j]
                push!(low_points, (i, j))
            end
        end
    end
    low_points
end

function problem_one(heatmap)
    n = 0
    for low_point in get_low_points(heatmap)
        i, j = low_point
        n += heatmap[i, j] + 1
    end
    n
    # risk_levels = Vector{Int8}()
    # rows, cols = size(heatmap)
    # for j in 1:cols
    #     for i in 1:rows
    #         if low_point(i, j, heatmap)
    #             @show i, j, heatmap[i, j]
    #             push!(risk_levels, heatmap[i, j] + 1)
    #         end
    #     end
    # end
    # sum(risk_levels)
end

function find_basin(low_point, heatmap)
    @show low_point
    @show typeof(low_point)
    explored = Set{Tuple{Int64, Int64}}()
    push!(explored, low_point)
    frontier = Vector{Tuple{Int64, Int64}}()
    push!(frontier, low_point)
    basin = Set{Tuple{Int64, Int64}}()
    while length(frontier) != 0
        i, j = pop!(frontier)
        try
            val = heatmap[i, j]
            if val != 9
                push!(basin, (i, j))
                neighbours = [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)]
                for neighbour in neighbours
                    if !(neighbour in explored)
                        push!(explored, neighbour)
                        push!(frontier, neighbour)
                    end
                end
            end
        catch e
            nothing
        end
    end
    length(basin)
end

function problem_two(heatmap)
    @show "hello there"
    @show heatmap
    low_points = get_low_points(heatmap)
    sizes = [0, 0, 0]
    for low_point in low_points
        basin_size = find_basin(low_point, heatmap)
        if basin_size > sizes[3]
            pop!(sizes)
            push!(sizes, basin_size)
            sizes = sort(sizes; rev=true)
        end
        # basin = find_basin(low_point, heatmap)
        # @show basin, low_point, length(basin)
        # size_sums += find_basin(low_point, heatmap)
    end
    reduce(*, sizes)
end

# bfs from the low points.
# or dfs, doesn't matter

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
