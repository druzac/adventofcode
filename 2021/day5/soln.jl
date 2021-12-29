import Test

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
    # @show delta
    start, last = a, b
    # @show start, last
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

Test.@testset "line segment iteration" begin
    p1 = Point(1, 2)
    p2 = Point(1, 5)
    ls = LineSegment(p1, p2)
    ls_rev = LineSegment(p2, p1)
    expected_result = [p1, Point(1, 3), Point(1, 4), p2]
    Test.@test collect(ls) == expected_result
    Test.@test collect(ls_rev) == reverse(expected_result)
    Test.@test collect(LineSegment(p1, p1)) == [p1]
    Test.@test collect(LineSegment(p2, p2)) == [p2]

    p3 = Point(2, 1)
    p4 = Point(5, 1)
    expected_result2 = [p3, Point(3, 1), Point(4, 1), p4]
    Test.@test collect(LineSegment(p3, p4)) == expected_result2
    Test.@test collect(LineSegment(p4, p3)) == reverse(expected_result2)
    Test.@test collect(LineSegment(p3, p3)) == [p3]
    Test.@test collect(LineSegment(p4, p4)) == [p4]

    pd1 = Point(1, 1)
    pd2 = Point(3, 3)
    pd3 = Point(9, 7)
    pd4 = Point(7, 9)
    diag_expected_1 = [pd1, Point(2, 2), pd2]
    diag_expected_2 = [pd3, Point(8, 8), pd4]
    Test.@test collect(LineSegment(pd1, pd2)) == diag_expected_1
    Test.@test collect(LineSegment(pd2, pd1)) == reverse(diag_expected_1)
    Test.@test collect(LineSegment(pd3, pd4)) == diag_expected_2
    Test.@test collect(LineSegment(pd4, pd3)) == reverse(diag_expected_2)
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
        # @show line
        p1, p2 = parse_line(line)
        # @show p1, p2
        push!(segments, LineSegment(Point(p1[1], p1[2]), Point(p2[1], p2[2])))
    end
    segments
end

# panics if it hits a negative number
function max_coords(segments)
    maxx = 0
    maxy = 0
    for segment in segments
        a, b = segment.a, segment.b
        maxx = max(a.x, b.x, maxx)
        maxy = max(a.y, b.y, maxy)
        # @show a, b
        # @show maxx, maxy
        if a.x < 0 || a.y < 0 || b.x < 0 || b.y < 0
            error("Negative number encountered")
        end
    end
    (maxx, maxy)
end

function count_intersections(inputf, predicate)
    segments = parse_problem(inputf)
    maxx, maxy = max_coords(segments)
    grid = zeros(Int64, maxx + 1, maxy + 1)
    # @show maxx, maxy
    # @show size(grid)
    for segment in segments
        if predicate(segment)
            for point in segment
                x, y = point.x, point.y
                grid[x + 1, y + 1] += 1
            end
        end
    end
    # @show grid
    result = count(grid .> 1)
    result
end

function problem_one(inputf)
    count_intersections(inputf, ls -> horizontal(ls) || vertical(ls))
    # segments = parse_problem(inputf)
    # maxx, maxy = max_coords(segments)
    # grid = zeros(Int64, maxx + 1, maxy + 1)
    # # @show maxx, maxy
    # # @show size(grid)
    # for segment in segments
    #     if horizontal(segment) || vertical(segment)
    #         for point in segment
    #             x, y = point.x, point.y
    #             grid[x + 1, y + 1] += 1
    #         end
    #     end
    # end
    # # @show grid
    # result = count(grid .> 1)
    # result
end

function problem_two(inputf)
    count_intersections(inputf, ls -> true)
end

function main(args)
    problem = args[1]
    input = args[2]

    if problem == "1"
        @show problem_one(input)
    elseif problem == "2"
        @show problem_two(input)
    end
end

if PROGRAM_FILE != "" && realpath(@__FILE__) == realpath(PROGRAM_FILE)
    main(ARGS)
end
