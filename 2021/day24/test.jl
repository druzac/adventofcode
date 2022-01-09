include("soln.jl")

import Test

function createiobuffer(lines)
    IOBuffer(join(lines, '\n'))
end

function parse_string_vector(lines)
    parse_problem(createiobuffer(lines))
end

Test.@testset "parsing" begin
    tc1 = ["inp x", "mul x -1"]
    e1 = [InputInstruction(:x), BinaryOperationInstruction(:x, -1, mult)]
    Test.@test parse_string_vector(tc1) == e1

    tc2 = ["inp z", "inp x", "mul z 3", "eql z x"]
    e2 = [InputInstruction(:z), InputInstruction(:x), BinaryOperationInstruction(:z, 3, mult), BinaryOperationInstruction(:z, :x, eql)]
    Test.@test parse_string_vector(tc2) == e2
end

Test.@testset "negation program" begin
    program = parse_string_vector(["inp x", "mul x -1"])
    inputs = Inputs([10])
    output1 = run_program(program, inputs)
    result = resolve(output1, :x)
    Test.@test result == -10

    output2 = resolve(run_program(program, Inputs([-10])), :x)
    Test.@test output2 == 10
end

Test.@testset "comparison program" begin
    program = parse_string_vector(["inp z", "inp x", "mul z 3", "eql z x"])
    function run_p(arg1, arg2)
        resolve(run_program(program, Inputs([arg1, arg2])), :z)
    end

    Test.@test run_p(1, 1) == 0
    Test.@test run_p(1, 3) == 1
    Test.@test run_p(1, 5) == 0
end

Test.@testset "bitify program" begin
    program = parse_string_vector(["inp w","add z w","mod z 2","div w 2","add y w","mod y 2","div w 2","add x w","mod x 2","div w 2","mod w 2"])
    function run_p(arg)
        state = run_program(program, Inputs([arg]))
        [resolve(state, :w), resolve(state, :x), resolve(state, :y), resolve(state, :z)]
    end

    Test.@test run_p(4) == [0, 1, 0, 0]
    Test.@test run_p(7) == [0, 1, 1, 1]
end
