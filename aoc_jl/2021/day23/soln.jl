import .Common

@enum Amphipod amber bronze copper desert none

struct State
    corridor::Vector{Amphipod}
    rooms::Vector{Vector{Amphipod}}
    big_rooms::Bool
    parent_state::Union{Nothing, State}
    move_cost::Int64
end

function initial_state(rooms, big_rooms)
    corridor = [none for _ in 1:11]
    State(corridor, deepcopy(rooms), big_rooms, nothing, 0)
end

function new_state(corridor, rooms, parent_state, move_cost)
    State(corridor, rooms, parent_state.big_rooms, parent_state, move_cost)
end

const ENTITY_TO_CHAR = Dict{Amphipod, Char}(amber => 'A', bronze => 'B', copper => 'C', desert => 'D', none => '.')
const CHAR_TO_ENTITY = Dict{Char, Amphipod}('A' => amber, 'B' => bronze, 'C' => copper, 'D' => desert, '.' => none)

function entity_to_char(entity)
    ENTITY_TO_CHAR[entity]
end

function char_to_entity(char)
    CHAR_TO_ENTITY[char]
end

function room_char(room, pos)
    entity_to_char(get(room, pos, none))
end

function pprint(state::State)
    line_length = 13
    println(repeat('#', line_length))
    corridor_line = string('#', join(entity_to_char(entity) for entity in state.corridor), '#')
    println(corridor_line)
    rn = max_room_size(state):-1:1
    for i in rn
        is_first = i == max_room_size(state)
        prefix = is_first ? repeat('#', 3) : string(repeat(' ', 2), '#')
        println(string(prefix,
                       join((room_char(room, i) for room in state.rooms), '#'),
                       reverse(prefix)))
    end
    println(string(repeat(' ', 2), repeat('#', 9)))
end

const ROOM_TO_DOORWAY = Dict{Int64, Int64}(1 => 3, 2 => 5, 3 => 7, 4 => 9)
const DOORWAYS = Set(values(ROOM_TO_DOORWAY))
const AMPHIPOD_TO_ROOM = Dict{Amphipod, Int64}(amber => 1, bronze => 2, copper => 3, desert => 4)
const ROOM_TO_AMPIHPOD = Dict{Int64, Amphipod}(1 => amber, 2 => bronze, 3 => copper, 4 => desert)
const AMPHIPOD_TO_COST = Dict{Amphipod, Int64}(amber => 1, bronze => 10, copper => 100, desert => 1000)

function to_canonical(state::State)
    amphipods = [[], [], [], []]
    for (corridor_pos, entity) in enumerate(state.corridor)
        if is_amphipod(entity)
            push!(amphipods[amphipod_to_room(entity)], (0, corridor_pos))
        end
    end
    for (room_n, room) in enumerate(state.rooms)
        for (room_pos, amphipod) in enumerate(room)
            if is_amphipod(amphipod)
                push!(amphipods[amphipod_to_room(amphipod)], (room_n, room_pos))
            end
        end
    end
    (tuple(amphipods[1]...), tuple(amphipods[2]...), tuple(amphipods[3]...), tuple(amphipods[4]...))
end

function Base.hash(state::State, h::UInt)
    hash(to_canonical(state), h)
end

function Base.:(==)(state1::State, state2::State)
    to_canonical(state1) == to_canonical(state2)
end

function room_to_doorway(room_number)
    ROOM_TO_DOORWAY[room_number]
end

function is_amphipod(entity)
    entity != none
end

function has_amphipod(v::Vector{Amphipod}, i)
    is_amphipod(v[i])
end

function is_end_state(state::State)
    if any(is_amphipod, state.corridor)
        return false
    end
    for (room_n, room) in enumerate(state.rooms)
        if length(room) != max_room_size(state) || any(entity -> entity != ROOM_TO_AMPIHPOD[room_n], room)
            return false
        end
    end
    return true
end

function move_cost(amphipod::Amphipod)
    AMPHIPOD_TO_COST[amphipod]
end

function max_room_size(state::State)
    state.big_rooms ? 4 : 2
end

function hallway_to_room_slot_distance(state::State, room_n)
    max_room_size(state) - length(state.rooms[room_n]) + 1
end

function move_amphipod_to_corridor(state, room_n, corridor_pos)
    new_corridor = copy(state.corridor)
    new_rooms = deepcopy(state.rooms)
    spaces_moved = abs(corridor_pos - room_to_doorway(room_n)) + hallway_to_room_slot_distance(state, room_n)
    amphipod = pop!(new_rooms[room_n])
    new_corridor[corridor_pos] = amphipod
    mcost = spaces_moved * move_cost(amphipod)
    (new_state(new_corridor, new_rooms, state, mcost), mcost)
end

function move_amphipod_to_room(state, corridor_pos, dest_room_n)
    new_corridor = copy(state.corridor)
    new_rooms = deepcopy(state.rooms)
    amphipod = new_corridor[corridor_pos]
    new_corridor[corridor_pos] = none
    push!(new_rooms[dest_room_n], amphipod)
    if length(new_rooms[dest_room_n]) > max_room_size(state)
        error("too many amphipods in a room")
    end
    spaces_moved = abs(corridor_pos - room_to_doorway(dest_room_n)) + hallway_to_room_slot_distance(state, dest_room_n) - 1
    mcost = spaces_moved * move_cost(amphipod)
    (new_state(new_corridor, new_rooms, state, mcost), mcost)
end

function amphipod_to_room(amphipod)
    AMPHIPOD_TO_ROOM[amphipod]
end

function all_amphipods_good(rooms, room_n)
    good_amphipod = ROOM_TO_AMPIHPOD[room_n]
    all(entity -> entity == good_amphipod, rooms[room_n])
end

function path_is_clear(state::State, corridor_pos::Int64, room_n::Int64)
    doorway_pos = room_to_doorway(room_n)
    start, stop = min(corridor_pos, doorway_pos), max(corridor_pos, doorway_pos)
    for (i, entity) in enumerate(state.corridor)
        if i >= start && i <= stop && i != corridor_pos && is_amphipod(entity)
            return false
        end
    end
    true
end

function neighbours(state)
    next_states::Vector{Tuple{State, Int64}} = []
    # first, try to move any amphipods in rooms into the corridor
    for (n, room) in enumerate(state.rooms)
        if !isempty(room)
            amphipod = room[end]
            if n == amphipod_to_room(amphipod) && all_amphipods_good(state.rooms, n)
                continue
            end
            doorway = room_to_doorway(n)
            iterate_corridor(it) = for corridor_pos in it
                if has_amphipod(state.corridor, corridor_pos)
                    break
                elseif !(corridor_pos in DOORWAYS)
                    push!(next_states, move_amphipod_to_corridor(state, n, corridor_pos))
                end
            end
            iterate_corridor(doorway + 1:length(state.corridor))
            iterate_corridor(doorway - 1:-1:1)
        end
    end
    # for any amphipods in the corridor, try to move them into a room
    for (corridor_pos, entity) in enumerate(state.corridor)
        if is_amphipod(entity)
            dest_room_n = amphipod_to_room(entity)
            if all_amphipods_good(state.rooms, dest_room_n) && path_is_clear(state, corridor_pos, dest_room_n)
                push!(next_states, move_amphipod_to_room(state, corridor_pos, dest_room_n))
            end
        end
    end
    next_states
end

function parse_problem(inputf)
    lines = readlines(inputf)
    rooms = Vector{Vector{Amphipod}}([[], [], [], []])
    for (room_n, line_idx) in enumerate(4:2:10)
        push!(rooms[room_n], char_to_entity(lines[4][line_idx]))
        push!(rooms[room_n], char_to_entity(lines[3][line_idx]))
    end
    initial_state(rooms, false)
end

function show_path(state)
    states = []
    current_state = state
    while !isnothing(current_state)
        push!(states, current_state)
        current_state = current_state.parent_state
    end
    for state in reverse(states)
        pprint(state)
        println(state.move_cost)
    end
end

function dijkstra(start_state)
    visited = Set{State}()
    mh = Common.MinHeap{State}()
    Common.insert!(mh, 0, start_state)
    num_iterations = 0
    while !isempty(mh)
        num_iterations += 1
        current_cost, current_state = Common.extract!(mh)
        if current_state in visited
            continue
        end
        if is_end_state(current_state)
            return current_cost
        end
        push!(visited, current_state)
        for (neighbour, mcost) in neighbours(current_state)
            new_cost = mcost + current_cost
            Common.insert!(mh, new_cost, neighbour)
        end
    end
    error("didn't find solution!")
end

function problem_one(start_state)
    dijkstra(start_state)
end

function problem_two(start_state)
    corr = start_state.corridor
    rooms = Vector{Vector{Amphipod}}()
    for (rn, room) in enumerate(start_state.rooms)
        push!(rooms, [room[1]])
    end
    push!(rooms[1], desert)
    push!(rooms[1], desert)
    push!(rooms[2], bronze)
    push!(rooms[2], copper)
    push!(rooms[3], amber)
    push!(rooms[3], bronze)
    push!(rooms[4], copper)
    push!(rooms[4], amber)
    for (rn, room) in enumerate(start_state.rooms)
        push!(rooms[rn], room[end])
    end
    st = initial_state(rooms, true)
    dijkstra(st)
end
