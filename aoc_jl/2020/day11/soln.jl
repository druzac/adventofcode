@enum Place floor free occupied

function isfree(place::Place)
    place == free
end

function isoccupied(place::Place)
    place == occupied
end

function parsechar(ch)
    if ch == 'L'
        free
    elseif ch == '#'
        occupied
    elseif ch == '.'
        floor
    else
        error("unrecognized char: $ch")
    end
end

function placetochar(place::Place)
    if place == floor
        '.'
    elseif place == free
        'L'
    elseif place == occupied
        '#'
    else
        error("invalid place: $place")
    end
end

function parse_problem(inputf)
    lines = readlines(inputf)
    n_rows = length(lines)
    n_cols = length(lines[1])
    board = Matrix{Place}(undef, n_rows, n_cols)
    for row in 1:n_rows
        for col in 1:n_cols
            board[row, col] = parsechar(lines[row][col])
        end
    end
    board
end

function neighbours(board, i, j)
    n_rows, n_cols = size(board)
    (board[i1, j1]
     for i1 in i - 1:i + 1
         for j1 in j - 1:j + 1
             if i1 >= 1 && i1 <= n_rows && j1 >= 1 && j1 <= n_cols && (i1 != i || j1 != j))
end

function line(board, i, j, di, dj)
    n_rows, n_cols = size(board)
    new_i, new_j = i, j
    while true
        new_i += di
        new_j += dj
        if 1 <= new_i && new_i <= n_rows && 1 <= new_j && new_j <= n_cols
            cell = board[new_i, new_j]
            if isfree(cell) || isoccupied(cell)
                return cell
            end
        else
            return nothing
        end
    end
end

function visiblechairs(board, i, j)
    gen = (line(board, i, j, di, dj)
           for di in -1:1
               for dj in -1:1
                   if di != 0 || dj != 0)
    Iterators.filter(x -> !isnothing(x), gen)
end

function flip(board, i, j, nf, maxoccupied)
    if isfree(board[i, j]) && all(x -> !isoccupied(x), nf(board, i, j))
        occupied
    elseif isoccupied(board[i, j]) && count(isoccupied, nf(board, i, j)) >= maxoccupied
        free
    else
        nothing
    end
end

function tick(board, nf, maxoccupied)
    new_board = Matrix{Place}(undef, size(board))
    n_rows, n_cols = size(board)
    flipped_cell = false
    for j in 1:n_cols
        for i in 1:n_rows
            result = flip(board, i, j, nf, maxoccupied)
            if !isnothing(result)
                flipped_cell = true
                new_board[i, j] = result
            else
                new_board[i, j] = board[i, j]
            end
        end
    end
    (new_board, flipped_cell)
end

function display(board::Matrix{Place})
    n_rows, n_cols = size(board)
    for i in 1:n_rows
        println(join(placetochar.(board[i, :])))
    end
end

function run(board, nf, maxoccupied)
    while true
        board, flipped = tick(board, nf, maxoccupied)
        if !flipped
            return count(isoccupied, board)
        end
    end
end

function problem_one(board)
    run(board, neighbours, 4)
end

function problem_two(board)
    run(board, visiblechairs, 5)
end
