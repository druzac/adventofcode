include(Base.Filesystem.joinpath("common", "Common.jl"))

year = ARGS[1]
day = ARGS[2]
part_number = ARGS[3]

path = Base.Filesystem.joinpath(year, day, "soln.jl")

inputf = if ARGS[4] == "-e"
    Base.Filesystem.joinpath(year, day, "example.txt")
elseif ARGS[4] == "-i"
    Base.Filesystem.joinpath(year, day, "input.txt")
else
    ARGS[4]
end

include(path)

if part_number == "1"
    println(problem_one(parse_problem(inputf)))
elseif part_number == "2"
    println(problem_two(parse_problem(inputf)))
end
