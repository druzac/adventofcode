struct Interval
    a::Int64
    b::Int64
end

function nonempty_intersect(interval1::Interval, interval2::Interval)
    contains(interval1, interval2.a) || contains(interval1, interval2.b) || contains(interval2, interval1.a) || contains(interval2, interval1.a)
end

struct CoordSolution
    startvel::Int64
    interval::Interval
end

struct Patch
    x::Interval
    y::Interval
end

function parse_problem(inputf)
    line = readline(inputf)
    x_vals = []
    y_vals = []
    for word in split(line)
        if word[1] == 'x'
            first, last = split(word[3:length(word) - 1], "..")
            push!(x_vals, parse(Int64, first))
            push!(x_vals, parse(Int64, last))
        elseif word[1] == 'y'
            first, last = split(word[3:length(word)], "..")
            push!(y_vals, parse(Int64, first))
            push!(y_vals, parse(Int64, last))
        end
    end
    x_int = Interval(x_vals[1], x_vals[2])
    y_int = Interval(y_vals[1], y_vals[2])
    Patch(x_int, y_int)
end

function problem_one(problem)
    # i solved this part using math on the subway.

    # the probe passes its starting location at the same speed going
    # downward it has when initially fired with a positive y
    # velocity. given the submarine is above the target region, find
    # the largest y that is still within the box.
    # to get the maximum height achieved, use the summation identity:
    # \sum(i=1..n)(i) == n * (n + 1) / 2
end

function x_pos_alt(xv0, t)
    if xv0 < 0
        error("negative starting velocity not implemented")
    end
    t = t >= xv0 ? xv0 : t
    (2 * xv0 + 1 - t) * t ÷ 2
end

function y_pos_alt(yv0, t)
    (2 * yv0 + 1 - t) * t ÷ 2
end

function Base.contains(in::Interval, x)
    in.a <= x && x <= in.b
end

function x_time_interval(p::Patch, xvel)
    furthest = x_pos_alt(xvel, xvel)
    if furthest < p.x.a
        error("never reaches goal")
    end
    last = if furthest <= p.x.b
        typemax(Int64)
    else
        current = furthest
        last = xvel
        while current > p.x.b
            last -= 1
            current = x_pos_alt(xvel, last)
        end
        last
    end
    first = xvel
    current = x_pos_alt(xvel, first)
    while current >= p.x.a
        first -= 1
        current = x_pos_alt(xvel, first)
    end
    first += 1
    if contains(p.x, x_pos_alt(xvel, last))
        Interval(first, last)
    else
        Interval(-1, -1)
    end
end

function find_x_values(p::Patch)
    xmin = p.x.a
    xmax = p.x.b
    xmin_vel = trunc(Int64, ceil((-1 + sqrt(1 + 8*xmin)) / 2))
    current_xvel = xmin_vel
    x_values = Vector{CoordSolution}()
    for x_cand_vel in xmin_vel:p.x.b
        interval = x_time_interval(p, x_cand_vel)
        if interval.a != -1 && interval.b != -1
            push!(x_values, CoordSolution(x_cand_vel, interval))
        end
    end
    x_values
end

function find_y_interval(interval::Interval, yvel)
    t = 1
    first = -1
    last = -1
    while true
        pos = y_pos_alt(yvel, t)
        if contains(interval, pos)
            last = t
            if first == -1
                first = t
            end
        elseif pos < interval.a
            break
        end
        t += 1
    end
    Interval(first, last)
end

function find_y_values(p::Patch)
    ymin = p.y.a
    ymax = p.y.b
    if p.y.a > 0 || p.y.b > 0
        error("only supports negative y")
    end
    # assume values are negative
    # negative end is p.y.a, positive end is -p.y.a
    coords = Vector{CoordSolution}()
    for y_cand_vel in ymin:-ymin
        interval = find_y_interval(p.y, y_cand_vel)
        if interval.a != -1 && interval.b != -1
            push!(coords, CoordSolution(y_cand_vel, interval))
        end
    end
    coords
end

function problem_two(patch)
    x_values = find_x_values(patch)
    y_values = find_y_values(patch)
    cnt = 0
    for xval in x_values
        for yval in y_values
            if nonempty_intersect(xval.interval, yval.interval)
                cnt += 1
            end
        end
    end
    cnt
end
