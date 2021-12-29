
struct PasswordPolicy
    char::Char
    min_cnt::Int64
    max_cnt::Int64
end

function parsepasswordpolicy(s)
    range, char = split(s)
    if length(char) != 1
        error("second part of policy is too long: ", char)
    end
    bounds = map(x -> parse(Int64, x), split(range, '-'))
    if length(bounds) != 2
        error("bounds is too long: ", bounds)
    end
    PasswordPolicy(char[1], minimum(bounds), maximum(bounds))
end

# 1-3 a: abcde
# 1-3 b: cdefg
# 2-9 c: ccccccccc
function parse_problem(inputf)
    parse_line(line) = begin
        spec, password = split(line, ':')
        (parsepasswordpolicy(spec), strip(password))
    end
    (parse_line(line) for line in eachline(inputf))
end

function isvalid(spec, password)
    cnt = count(spec.char, password)
    spec.min_cnt <= cnt && cnt <= spec.max_cnt
end

function problem_one(problem)
    sum(1 for (spec, password) in problem if isvalid(spec, password))
    # for (spec, password) in problem
    #     @show spec, password
    #     @show isvalid(spec, password)
    # end
end

function isvalidtwo(spec, password)
    positions = (spec.min_cnt, spec.max_cnt)
    # @show positions, spec.char
    sum(1 for position in positions if password[position] == spec.char; init=0) == 1
end

function problem_two(problem)
    sum(1 for (spec, password) in problem if isvalidtwo(spec, password))
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
