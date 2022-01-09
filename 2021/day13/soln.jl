# first coordinate is col
# second coordinate is row

@enum Direction x y

function parse_problem(inputf)
    it = eachline(inputf)
    coords = Vector{Tuple{Int64, Int64}}()
    for line in it
        if isempty(line)
            break
        end
        a, b = map(val -> parse(Int64, val), split(line, ','))
        push!(coords, (a + 1, b + 1))
    end
    function chartodirection(char)
        if char == 'x'
            x
        elseif char == 'y'
            y
        else
            error("unexpected char: ", char)
        end
    end
    folds = Vector{Tuple{Direction, Int64}}()
    for line in it
        words = split(line)
        if words[1] != "fold" || words[2] != "along"
            error("unexpected words: ", words[1], words[2])
        end
        rawchar, rawval = split(words[3], '=')
        val = parse(Int64, rawval)
        direction = chartodirection(rawchar[1])
        push!(folds, (direction, val + 1))
    end
    n_cols = maximum(map(val -> val[1], coords))
    n_rows = maximum(map(val -> val[2], coords))
    bm = falses(n_rows, n_cols)
    for coord in coords
        bm[coord[2], coord[1]] = true
    end
    (bm, folds)
end

function foldpaper(bm, fold)
    val = fold[2]
    bm1, bm2 = if (fold[1] == x)
        tmp1 = @view bm[:, 1:val - 1]
        tmp2 = reverse(@view bm[1:end, val + 1:end]; dims=2)
        tmp1, tmp2
    else
        tmp1 = @view bm[1:val - 1, :]
        tmp2 = reverse(@view bm[val + 1:end, :]; dims=1)
        tmp1, tmp2
    end
    bm1 .| bm2
end

function displaypaper(bm)
    nrows, ncols = size(bm)
    for i in 1:nrows
        for j in 1:ncols
            char = bm[i, j] ? '#' : '.'
            print(char)
        end
        println()
    end
end

function problem_one(problem)
    bm = problem[1]
    folds = problem[2]
    bm = foldpaper(bm, folds[1])
    count(bm)
end

function problem_two(problem)
    bm = problem[1]
    folds = problem[2]
    for fold in folds
        bm = foldpaper(bm, fold)
    end
    displaypaper(bm)
end

function main(args)
    problem_number = args[1]
    inputf = args[2]

    problem = parse_problem(inputf)
    if problem_number == "1"
        @show problem_one(problem)
    elseif problem_number == "2"
        @show problem_two(problem)
    else
        error("Need to put in 1 or 2")
    end
end

if PROGRAM_FILE != "" && realpath(@__FILE__) == realpath(PROGRAM_FILE)
    main(ARGS)
end
