
abstract type Command end

struct MoveCardinal <: Command
    delta::Vector{Int64}
end

struct MoveForward <: Command
    spaces::Int64
end

# turns are always clockwise
# turns are given in integer number of 90 degree turns
struct Turn <: Command
    rotations::Int64
end

function parseline(line)
    fchar = line[1]
    n = parse(Int64, line[2:end])
    if fchar == 'F'
        MoveForward(n)
    elseif fchar == 'N'
        MoveCardinal([0, n])
    elseif fchar == 'S'
        MoveCardinal([0, -n])
    elseif fchar == 'E'
        MoveCardinal([n, 0])
    elseif fchar == 'W'
        MoveCardinal([-n, 0])
    elseif fchar == 'L'
        if n % 90 != 0
            error("bad degrees: $line")
        end
        Turn(mod(-(n ÷ 90), 4))
    elseif fchar == 'R'
        if n % 90 != 0
            error("bad degrees: $line")
        end
        Turn(mod((n ÷ 90), 90))
    else
        error("unrecognized command: $line")
    end
end

function parse_problem(inputf)
    [parseline(line) for line in eachline(inputf)]
end

mutable struct State
    coordinates::Vector{Int64}
    # heading is a number from 0 to 3.
    # 0 - N
    # 1 - E
    # 2 - S
    # 3 - W
    heading::Int64
    State() = new([0, 0], 1)
end

function execute!(state::State, mc::MoveCardinal)
    state.coordinates += mc.delta
end

function execute!(state::State, mf::MoveForward)
    unit_vec = if state.heading == 0
        [0, 1]
    elseif state.heading == 1
        [1, 0]
    elseif state.heading == 2
        [0, -1]
    elseif state.heading == 3
        [-1, 0]
    else
        error("bad heading: $state")
    end
    state.coordinates += unit_vec * mf.spaces
end

function execute!(state::State, t::Turn)
    state.heading = mod(t.rotations + state.heading, 4)
end

function problem_one(commands)
    state = State()
    for command in commands
        execute!(state, command)
    end
    sum(abs.(state.coordinates))
end
