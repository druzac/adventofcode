import Base.Filesystem

include(Filesystem.joinpath("common", "Common.jl"))

year = ARGS[1]
day = ARGS[2]
part_number = ARGS[3]

if part_number == "t"
    include(Filesystem.joinpath(year, day, "test.jl"))
else
    inputf = if  ARGS[4] == "-e"
        Filesystem.joinpath(year, day, "example.txt")
    elseif ARGS[4] == "-i"
        Filesystem.joinpath(year, day, "input.txt")
    else
        Filesystem.joinpath(year, day, ARGS[4])
    end
    include(Filesystem.joinpath(year, day, "soln.jl"))

    if part_number == "1"
        println(problem_one(parse_problem(inputf)))
    elseif part_number == "2"
        println(problem_two(parse_problem(inputf)))
    end
end
