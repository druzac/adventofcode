import .Common

const Point = Tuple{Int64, Int64}

const MAX_COST = 9

struct ExtendedCave
    m::Matrix{Int64}
    extension::Int64
end

function Base.getindex(c::ExtendedCave, i, j)
    n_rows, n_cols = size(c)
    if i < 1 || j < 1 || i > n_rows || j > n_cols
        throw(BoundsError(c, (i, j)))
    end
    base_rows, base_cols = size(c.m)
    col_displacement = (j - 1) ÷ base_cols
    row_displacement = (i - 1) ÷ base_rows
    base_i, base_j = (i % base_rows, j % base_cols)
    if base_i == 0
        base_i = base_rows
    end
    if base_j == 0
        base_j = base_cols
    end
    cost = c.m[base_i, base_j] + col_displacement + row_displacement
    cost = cost % MAX_COST
    if cost == 0
        cost = MAX_COST
    end
    cost
end

function Base.size(c::ExtendedCave)
    size(c.m) .* c.extension
end

function parse_problem(inputf)
    lines = readlines(inputf)
    n_cols = length(lines[1])
    n_rows = length(lines)
    m = Matrix{Int64}(undef, n_rows, n_cols)
    for i in 1:n_rows
        for j in 1:n_cols
            m[i, j] = parse(Int64, lines[i][j])
            if m[i, j] <= 0
                error("values are too small!", i, j)
            end
        end
    end
    m
end

function neighbours(grid, point)
    n_rows, n_cols = size(grid)
    i, j = point
    neighbours = []
    for jp in j - 1:j + 1
        if jp >= 1 && jp <= n_cols
            for ip in i - 1:i + 1
                if ip >= 1 && ip <= n_rows && (abs(jp - j) + abs(ip - i)) == 1
                    push!(neighbours, (ip, jp))
                end
            end
        end
    end
    neighbours
end

function matrix_dijkstra(grid)
    current_node = (1, 1)
    costs = Dict{Tuple{Int64, Int64}, Int64}()
    costs[current_node] = 0
    unexplored = Dict{Tuple{Int64, Int64}, Int64}()
    visited = Set{Point}([current_node])
    mh = Common.MinHeap{Point}()
    num_visits = 0
    while current_node != size(grid)
        num_visits += 1
        for neighbour in neighbours(grid, current_node)
            if neighbour in visited
                continue
            end
            curr_node_val = get(unexplored, neighbour, typemax(Int64))
            new_node_val = min(costs[current_node] + grid[neighbour[1], neighbour[2]], curr_node_val)
            if new_node_val < curr_node_val
                unexplored[neighbour] = new_node_val
                Common.insert!(mh, new_node_val, neighbour)
            end
        end
        current_node = Common.extract!(mh)[2]
        push!(visited, current_node)
        while !haskey(unexplored, current_node)
            current_node = Common.extract!(mh)[2]
        end
        costs[current_node] = pop!(unexplored, current_node)
    end
    n_rows, n_cols = size(grid)
    costs[(n_rows, n_cols)]
end

function problem_one(grid)
    matrix_dijkstra(grid)
end

function problem_two(grid)
    matrix_dijkstra(ExtendedCave(grid, 5))
end
