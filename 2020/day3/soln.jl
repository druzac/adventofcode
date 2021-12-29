
function parse_problem(inputf)
    function parse_char(char)
        if char == '#'
            1
        elseif char == '.'
            0
        else
            error("invalid character: ", char)
        end
    end
    function parse_line(line)
        
    end
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
