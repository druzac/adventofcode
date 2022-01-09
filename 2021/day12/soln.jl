# can visit big caves any number of times
# so there can be no purely small cave

# it's a graph search, but.
# explored set is only for small caves.
# termination condition? no more paths to expand.

# what is a path?
# a set of small caves (`start` and `end` count),
# and the current terminal.

# d is a Dict{K, Collection{V}}
function append_or_create(d, k, v)
    values = get(d, k, nothing)
    if values == nothing
        d[k] = [v]
    else
        push!(values, v)
    end
end

struct CaveSystem
    adj_lists::Dict{String, Vector{String}}
    CaveSystem(edges) = begin
        added_edges = Set{Tuple{String, String}}()
        adj_lists = Dict{String, Vector{String}}()
        for edge in edges
            if edge in added_edges || (edge[2], edge[1]) in added_edges
                error("redundant edge")
            end
            push!(added_edges, edge)
            push!(added_edges, (edge[2], edge[1]))
            append_or_create(adj_lists, edge[1], edge[2])
            append_or_create(adj_lists, edge[2], edge[1])
        end
        new(adj_lists)
    end
end

START_CAVE = "start"
END_CAVE = "end"

function neighbours(cs::CaveSystem, cave)
    cs.adj_lists[cave]
end

function isbigcave(cave)::Bool
    all(isuppercase, cave)
end

function isendcave(cave)
    cave == END_CAVE
end

function parse_problem(inputf)
    parse_line(line) = begin
        a, b = split(line, '-')
        (a, b)
    end
    # never allocate whole thing into memory?
    edges = (parse_line(line) for line in eachline(inputf))
    CaveSystem(edges)
end

abstract type Path end

struct BasePath <: Path
    small_caves::Set{String}
    terminal::String
    BasePath(start::String) = begin
        small_caves = Set{String}()
        if !isbigcave(start)
            push!(small_caves, start)
        end
        new(small_caves, start)
    end
    BasePath(small_caves, terminal) = begin
        new(small_caves, terminal)
    end
end

function canvisit(p::BasePath, cave)
    !(cave in p.small_caves)
end

function add_step(p::BasePath, next)
    addition = isbigcave(next) ? [] : [next]
    new_small_caves = union(p.small_caves, addition)
    BasePath(new_small_caves, next)
end

function getterminal(p::BasePath)
    p.terminal
end

struct DoubleSmallCavePath <: Path
    canrevisitsmall::Bool
    bp::BasePath
    DoubleSmallCavePath(start::String) = new(true, BasePath(start))
    DoubleSmallCavePath(canrevisitsmall, bp) = new(canrevisitsmall, bp)
end

function canvisit(p::DoubleSmallCavePath, cave)
    canvisit(p.bp, cave) || (cave != START_CAVE && cave != END_CAVE && p.canrevisitsmall)
end

function add_step(p::DoubleSmallCavePath, cave)
    canrevisitsmall = p.canrevisitsmall && !(cave in p.bp.small_caves)
    DoubleSmallCavePath(canrevisitsmall, add_step(p.bp, cave))
end

function getterminal(p::DoubleSmallCavePath)
    getterminal(p.bp)
end

function take_step(path::Path, cs::CaveSystem)
    (add_step(path, neighbour) for neighbour in neighbours(cs, getterminal(path)) if canvisit(path, neighbour))
end

function iscomplete(path::Path)
    getterminal(path) == END_CAVE
end

function generic_problem_solve(cs, starter)
    paths = [starter(START_CAVE)]
    num_paths = 0
    while !isempty(paths)
        curr_path = pop!(paths)
        for new_path in take_step(curr_path, cs)
            if iscomplete(new_path)
                num_paths += 1
            else
                 push!(paths, new_path)
            end
        end
    end
    num_paths
end

function problem_one(cs)
    generic_problem_solve(cs, BasePath)
end

function problem_two(cs)
    generic_problem_solve(cs, DoubleSmallCavePath)
end
