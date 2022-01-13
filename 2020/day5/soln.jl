function parse_problem(inputf)
    readlines(inputf)
end

function toseatnumber(line)
    row = 0
    for ch in line[1:7]
        bit = if ch == 'F'
            0
        elseif ch == 'B'
            1
        else
            error("bad char for row: $(ch)")
        end
        row = (row << 1) + bit
    end
    col = 0
    for ch in line[8:10]
        bit = if ch == 'R'
            1
        elseif ch == 'L'
            0
        else
            error("bad char for column: $(ch)")
        end
        col = (col << 1) + bit
    end
    [row, col]
end

function seatid(seat)
    seat' * [8, 1]
end

function problem_one(lines)
    maximum(seatid(toseatnumber(line)) for line in lines)
end

function problem_two(lines)
    seatids = Set(seatid(toseatnumber(line)) for line in lines)
    # 128 rows (0 to 127)
    # 8 columns (0 to 7)
    candidates = Set{Int64}()
    for row_n in 0:127
        for col_n in 0:7
            sid = seatid([row_n, col_n])
            if !(sid in seatids) && ((sid + 1) in seatids) && ((sid - 1) in seatids)
                push!(candidates, sid)
            end
        end
    end
    if length(candidates) == 1
        return pop!(candidates)
    else
        error("bad candidates: $(candidates)")
    end
end
