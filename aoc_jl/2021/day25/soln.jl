struct State
    right_cucumbers::Vector{Vector{Int64}}
    down_cucumbers::Vector{Vector{Int64}}
    board::BitMatrix
end

function pprint(st::State)
    out_board = Matrix{Char}(undef, size(st.board))
    out_board[:, :] .= '.'
    for cuc in st.right_cucumbers
        out_board[cuc[1], cuc[2]] = '>'
    end
    for cuc in st.down_cucumbers
        out_board[cuc[1], cuc[2]] = 'v'
    end
    n_rows, n_cols = size(out_board)
    for i in 1:n_rows
        for j in 1:n_cols
            print(out_board[i, j])
        end
        println()
    end
end

function parse_problem(inputf)
    positions = readlines(inputf)
    n_rows = length(positions)
    n_cols = length(positions[1])
    right_cucumbers = Vector{Vector{Int64}}()
    down_cucumbers = Vector{Vector{Int64}}()
    board = falses(n_rows, n_cols)
    for i in 1:n_rows
        for j in 1:n_cols
            if positions[i][j] != '.'
                if positions[i][j] == '>'
                    push!(right_cucumbers, [i, j])
                elseif positions[i][j] == 'v'
                    push!(down_cucumbers, [i, j])
                else
                    error("unrecognized character: $(positions[i][j])")
                end
                board[i,j] = true
            end
        end
    end
    State(right_cucumbers, down_cucumbers, board)
end

function right_neighbour(pos, sz)
    if pos[2] < sz[2]
        [pos[1], pos[2] + 1]
    else
        [pos[1], 1]
    end
end

function down_neighbour(pos, sz)
    if pos[1] < sz[1]
        [pos[1] + 1, pos[2]]
    else
        [1, pos[2]]
    end
end

function move_cucumbers(cucumbers, board, mover_f)
    moved = false
    new_cucumbers = Vector{Vector{Int64}}(undef, length(cucumbers))
    for i in 1:length(cucumbers)
        current_pos = cucumbers[i]
        next_pos = mover_f(current_pos, size(board))
        if board[next_pos[1], next_pos[2]]
            new_cucumbers[i] = current_pos
        else
            moved = true
            new_cucumbers[i] = next_pos
        end
    end
    for (old_pos, new_pos) in zip(cucumbers, new_cucumbers)
        board[old_pos[1], old_pos[2]] = false
        board[new_pos[1], new_pos[2]] = true
    end
    cucumbers[:] = new_cucumbers
    moved
end

function take_step(state)
    right_moved = move_cucumbers(state.right_cucumbers, state.board, right_neighbour)
    down_moved = move_cucumbers(state.down_cucumbers, state.board, down_neighbour)
    right_moved || down_moved
end

function problem_one(state)
    num_moves = 0
    while true
        moved = take_step(state)
        num_moves += 1
        if !moved
            return num_moves
        end
    end
end
