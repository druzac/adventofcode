struct Point
    x::Int64
    y::Int64
end

struct LineSegment
    a::Point
    b::Point
end

function Base.:+(p1::Point, p2::Point)
    return Point(p1.x + p2.x, p1.y + p2.y)
end

function Base.isless(p1::Point, p2::Point)
    return (p1.x, p1.y) < (p2.x, p2.y)
end

function Base.iterate(ls::LineSegment)
    a, b = ls.a, ls.b
    delta =
        if a.x == b.x
            a.y <= b.y ? Point(0, 1) : Point(0, -1)
        elseif a.y == b.y
            a.x <= b.x ? Point(1, 0) : Point(-1, 0)
        else
            xdelta = a.x < b.x ? 1 : -1
            ydelta = a.y < b.y ? 1 : -1
            Point(xdelta, ydelta)
        end
    start, last = a, b
    (start, (start, delta, last, start == last))
end

function Base.iterate(ls::LineSegment, state)
    curr, delta, last, done = state
    if done
        nothing
    else
        next = curr + delta
        (next, (next, delta, last, next == last))
    end
end

function Base.length(ls::LineSegment)
    a, b = ls.a, ls.b
    if a.x == b.x
        abs(a.y - b.y) + 1
    elseif a.y == b.y
        abs(a.x - b.x) + 1
    else
        xdist = abs(a.x - b.x) + 1
        ydist = abs(a.y - b.y) + 1
        if xdist != ydist
            error("Not a 45 degree diagonal")
        end
        xdist
    end
end

function horizontal(ls::LineSegment)
    ls.a.y == ls.b.y
end

function vertical(ls::LineSegment)
    ls.a.x == ls.b.x
end

function parse_vector(t, v)
    map(x -> parse(t, x), v)
end

function parse_line(line)
    rp1, rp2 = split(line, " -> ")
    p1 = parse_vector(Int64, split(rp1, ','))
    p2 = parse_vector(Int64, split(rp2, ','))
    (p1, p2)
end

function parse_problem(inputf)
    segments = Vector{LineSegment}()
    for line in eachline(inputf)
        p1, p2 = parse_line(line)
        push!(segments, LineSegment(Point(p1[1], p1[2]), Point(p2[1], p2[2])))
    end
    segments
end

function max_coords(segments)
    maxx = 0
    maxy = 0
    for segment in segments
        a, b = segment.a, segment.b
        maxx = max(a.x, b.x, maxx)
        maxy = max(a.y, b.y, maxy)
        if a.x < 0 || a.y < 0 || b.x < 0 || b.y < 0
            error("Negative number encountered")
        end
    end
    (maxx, maxy)
end

function count_intersections(segments, predicate)
    maxx, maxy = max_coords(segments)
    grid = zeros(Int64, maxx + 1, maxy + 1)
    for segment in segments
        if predicate(segment)
            for point in segment
                x, y = point.x, point.y
                grid[x + 1, y + 1] += 1
            end
        end
    end
    result = count(grid .> 1)
    result
end

function problem_one(segments)
    count_intersections(segments, ls -> horizontal(ls) || vertical(ls))
end

function problem_two(segments)
    count_intersections(segments, ls -> true)
end
