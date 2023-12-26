@enum Player pone ptwo

function next(p::Player)
    if p == pone
        ptwo
    elseif p == ptwo
        pone
    else
        error("unrecognized player $p")
    end
end

abstract type Dice end

mutable struct D100Deterministic <: Dice
    last_roll::Int64
    num_rolls::Int64
    D100Deterministic() = new(100, 0)
end

function roll_die(det::D100Deterministic)
    next_roll = det.last_roll == 100 ? 1 : det.last_roll + 1
    det.last_roll = next_roll
    det.num_rolls += 1
    next_roll
end

function num_rolls(det::D100Deterministic)
    det.num_rolls
end

mutable struct PlayerState
    score::Int64
    position::Int8
    PlayerState(pos::Int8) = new(0, pos)
end

function move_player(ps::PlayerState, squares_to_move::Int64, num_squares::Int64)
    next_square = (ps.position + squares_to_move) % num_squares
    next_square = next_square > 0 ? next_square : num_squares
    ps.position = next_square
    ps.score += next_square
end

mutable struct DiracDice
    nextplayer::Player
    playerone::PlayerState
    playertwo::PlayerState
    dice::Dice
    goal_score::Int64
    num_squares::Int64
    rolls_per_turn::Int64
end

function take_turn(dd::DiracDice)
    player = if dd.nextplayer == pone
        dd.playerone
    elseif dd.nextplayer == ptwo
        dd.playertwo
    else
        error("unrecognized next player")
    end
    roll_sum = sum(roll_die(dd.dice) for _ in 1:dd.rolls_per_turn)
    move_player(player, roll_sum, dd.num_squares)
    dd.nextplayer = next(dd.nextplayer)
end

function player_value(ps::PlayerState, dice::Dice)
    ps.score * num_rolls(dice)
end

function check_end(dd::DiracDice)
    if dd.playerone.score >= dd.goal_score
        player_value(dd.playertwo, dd.dice)
    elseif dd.playertwo.score >= dd.goal_score
        player_value(dd.playerone, dd.dice)
    else
        nothing
    end
end

function parse_problem(inputf)
    open(inputf, "r") do io
        pone_pos = parse(Int8, split(readline(io))[end])
        ptwo_pos = parse(Int8, split(readline(io))[end])
        (PlayerState(pone_pos), PlayerState(ptwo_pos))
    end
end

function problem_one(player_states)
    dd = DiracDice(pone, player_states[1], player_states[2], D100Deterministic(), 1000, 10, 3)
    while true
        take_turn(dd)
        final_value = check_end(dd)
        if !isnothing(final_value)
            return final_value
        end
    end
end

struct QPlayerState
    score::Int64
    position::Int64
    QPlayerState(ps::PlayerState) = new(ps.score, ps.position)
    QPlayerState(score::Int64, position::Int64) = new(score, position)
end

struct UniverseState
    next_player::Player
    playerone::QPlayerState
    playertwo::QPlayerState
end

function compute_3_roll_results()
    tuples = [(i, j, k) for i in 1:3
                  for j in 1:3
                      for k in 1:3]
    d = Dict{Int64, Int64}()
    for tuple in tuples
        val = sum(tuple)
        if haskey(d, val)
            d[val] += 1
        else
            d[val] = 1
        end
    end
    d
end

const THREE_ROLL_RESULTS = compute_3_roll_results()

function roll_three_qdie()
    ((k, v) for (k, v) in THREE_ROLL_RESULTS)
end

const UniverseCache = Dict{UniverseState, Vector{Int64}}

function move_player(ps::QPlayerState, squares_to_move::Int64, num_squares::Int64)
    next_square = (ps.position + squares_to_move) % num_squares
    next_square = next_square > 0 ? next_square : num_squares
    QPlayerState(ps.score + next_square, next_square)
end

function update_state(us::UniverseState, roll::Int64)
    if us.next_player == pone
        UniverseState(ptwo, move_player(us.playerone, roll, 10), us.playertwo)
    elseif us.next_player == ptwo
        UniverseState(pone, us.playerone, move_player(us.playertwo, roll, 10))
    else
        error("invalid next player")
    end
end

function simulateuniverse(us::UniverseState, max_score::Int64)
    new_cache = UniverseCache()
    simulateuniverse(us, new_cache, max_score)
end

function simulateuniverse(us::UniverseState, uc::UniverseCache, max_score::Int64)
    if us.playerone.score >= max_score
        return [1, 0]
    elseif us.playertwo.score >= max_score
        return [0, 1]
    end

    result = [0, 0]
    for (roll, n_universes) in roll_three_qdie()
        new_us = update_state(us, roll)
        if haskey(uc, new_us)
            result += n_universes * uc[new_us]
        else
            result += n_universes * simulateuniverse(new_us, uc, max_score)
        end
    end
    uc[us] = result
    result
end

function problem_two(player_states)
    qplayer_states = QPlayerState.(player_states)
    us = UniverseState(pone, qplayer_states[1], qplayer_states[2])
    results = simulateuniverse(us, 21)
    maximum(results)
end
