struct Interval
    a::Int64
    b::Int64
    Interval(a, b) = new(min(a, b), max(a, b))
end

function width(i::Interval)
    i.b - i.a + 1
end

struct Cube
    x::Interval
    y::Interval
    z::Interval
end

function Base.contains(i::Interval, v::Int64)
    i.a <= v && v <= i.b
end

function hasintersection(i1::Interval, i2::Interval)
    !(i1.b < i2.a || i2.b < i1.a)
end

function hasintersection(c1::Cube, c2::Cube)
    return hasintersection(c1.x, c2.x) && hasintersection(c1.y, c2.y) && hasintersection(c1.z, c2.z)
end

function Base.contains(i1::Interval, i2::Interval)
    i1.a <= i2.a && i2.b <= i1.b
end

function Base.contains(c1::Cube, c2::Cube)
    contains(c1.x, c2.x) && contains(c1.y, c2.y) && contains(c1.z, c2.z)
end


function split_interval(i1::Interval, i2::Interval)
    if !hasintersection(i1, i2)
        (i1, nothing)
    elseif i1.a < i2.a
        (Interval(i1.a, i2.a - 1), Interval(i2.a, i1.b))
    elseif i1.b > i2.b
        (Interval(i2.b + 1, i1.b), Interval(i1.a, i2.b))
    else
        (nothing, i1)
    end
end

function update_interval(cube1::Cube, cube2::Cube, interval_selector, cube_creator)
    (truncated, rest) = split_interval(interval_selector(cube1), interval_selector(cube2))
    if !isnothing(truncated)
        good_subcube = cube_creator(cube1, truncated)
        rest_subcube = cube_creator(cube1, rest)
        return (good_subcube, rest_subcube)
    else
        return nothing
    end
end

# function subtract
# returns a list of cubes that are contained in cube1 and don't intersect cube2.
function subtract(cube1::Cube, cube2::Cube)
    if !(hasintersection(cube1, cube2))
        return [cube1]
    end
    x_selector(c::Cube) = c.x
    x_creator(c::Cube, i::Interval) = Cube(i, c.y, c.z)
    y_selector(c::Cube) = c.y
    y_creator(c::Cube, i::Interval) = Cube(c.x, i, c.z)
    z_selector(c::Cube) = c.z
    z_creator(c::Cube, i::Interval) = Cube(c.x, c.y, i)
    for (selector, creator) in zip((x_selector, y_selector, z_selector),
                                   (x_creator, y_creator, z_creator))
        result = update_interval(cube1, cube2, selector, creator)
        if !isnothing(result)
            good_subcube, rest_subcube = result
            rest = subtract(rest_subcube, cube2)
            push!(rest, good_subcube)
            return rest
        end
    end
    []
end

struct Command
    toggle::Bool
    cube::Cube
end

function parse_line(line)
    toggle_s, coords = split(line)
    
    toggle = if toggle_s == "on"
        true
    elseif toggle_s == "off"
        false
    else
        error("unrecognized switch: $(toggle_s)")
    end
    coords = split(coords, ',')
    extract_coords(coords_s) = map(x -> parse(Int64, x), split(coords_s[3:end], ".."))
    to_interval(pair) = Interval(pair...)
    intervals = map(to_interval ∘ extract_coords, coords)
    Command(toggle, Cube(intervals...))
end

# parse to a list of commands
function parse_problem(inputf)
    (parse_line(line) for line in eachline(inputf))
end

function update_on_cubes(on_cubes, command)
    cube_to_sub = command.cube
    subbed_cubes = collect(Iterators.flatten((subtract(cube, cube_to_sub) for cube in on_cubes)))
    if command.toggle
        push!(subbed_cubes, command.cube)
    end
    subbed_cubes
end

function volume(cube::Cube)
    width(cube.x) * width(cube.y) * width(cube.z)
end

function process_commands(commands, predicate)
    on_cubes = []
    for command in commands
        if predicate(command.cube)
            on_cubes = update_on_cubes(on_cubes, command)
        end
    end
    sum(volume(cube) for cube in on_cubes)
end

function problem_one(commands)
    initialize_area = Cube(Interval(-50, 50),
                           Interval(-50, 50),
                           Interval(-50, 50))
    process_commands(commands, cube -> contains(initialize_area, cube))
end

function problem_two(commands)
    process_commands(commands, cube -> true)
end
