
# numbers called
# list of boards

# iterate through boards. build an index -> number to board, board index.
# iterate through numbers...

# Board type - has a matrix, and a bitmatrix
# bitmatrix shows which thingies are marked.

struct Board
    raw_board::Matrix{Int64}
    numbers::Dict{Int64, Tuple{Int64, Int64}}
    filled::BitMatrix
    Board(raw_board) = begin
        d = Dict{Int64, Tuple{Int64, Int64}}()
        for j = 1:size(raw_board, 2)
            for i = 1:size(raw_board, 1)
                d[raw_board[i, j]] = (i, j)
            end
        end
        new(raw_board, d, BitMatrix(undef, size(raw_board)))
    end
end

# returns true if we got a bingo
function mark_board(b::Board, number)
    idx = get(b.numbers, number, nothing)
    if idx != nothing
        i, j = idx
        b.filled[i, j] = 1
        row = @view b.filled[i, :]
        col = @view b.filled[:, j]
        if count(row) == length(row) || count(col) == length(col)
            return true
        end
    end
    false
end

function calculate_score(b::Board, number)
    unmarked_ids = map(!, b.filled)
    @show sum(b.raw_board[unmarked_ids])
    sum(b.raw_board[unmarked_ids]) * number
end

function play_bingo(boards, numbers)
    for number in numbers
        for board in boards
            bingo = mark_board(board, number)
            if bingo
                return calculate_score(board, number)
            end
        end
    end
    error("No winner found")
end

function play_bingo_to_last_board(boards, numbers)
    my_boards = copy(boards)
    for number in numbers
        boards_to_remove = Set{Board}()
        for board in my_boards
            bingo = mark_board(board, number)
            if bingo && length(my_boards) == 1
                return calculate_score(board, number)
            elseif bingo
                push!(boards_to_remove, board)
            end
        end
        for board_to_remove in boards_to_remove
            delete!(my_boards, board_to_remove)
        end
    end
end

struct Problem
    numbers::Vector{Int64}
    boards::Set{Board}
end

function parse_input(inputf)
    open(inputf, "r") do io
        numbers_line = readline(io)

        numbers = map(x -> parse(Int64, x), split(numbers_line, ','))

        readline(io)

        boards = Set{Board}()
        first_row = readline(io)
        while first_row != ""
            raw_board = Matrix{Int64}(undef, 5, 5)
            first_row_numbers = map(x -> parse(Int64, x), split(first_row))
            raw_board[1, :] = first_row_numbers
            for i in 2:5
                curr_line = readline(io)
                # @show curr_line
                curr_line_numbers = map(x -> parse(Int64, x), split(curr_line))
                raw_board[i, :] = curr_line_numbers
            end
            # @show raw_board
            push!(boards, Board(raw_board))

            readline(io)
            first_row = readline(io)
        end
        return Problem(numbers, boards)
    end
end

function problem_one(inputf)
    problem = parse_input(inputf)
    @show play_bingo(problem.boards, problem.numbers)
end

function problem_two(inputf)
    problem = parse_input(inputf)
    @show play_bingo_to_last_board(problem.boards, problem.numbers)
end

function main(args)
    problem = args[1]
    input = args[2]

    if problem == "1"
        problem_one(input)
    elseif problem == "2"
        problem_two(input)
    end
end

main(ARGS)
