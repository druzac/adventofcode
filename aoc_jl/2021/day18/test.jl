using Test

include("soln.jl")

function test_reduce(input, expected, reduce_action)
    arg = read_snailfishnumber(input)
    expected = read_snailfishnumber(expected)
    @test reduce!(arg) == reduce_action
    @test arg == expected
end

@testset "reduce" begin
    test_reduce("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]", true)
    test_reduce("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]", true)
    test_reduce("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]", true)
    test_reduce("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", true)
    test_reduce("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]", "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]", true)
    test_reduce("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]", "[[[[0,7],4],[15,[0,13]]],[1,1]]", true)
    test_reduce("[[[[0,7],4],[15,[0,13]]],[1,1]]", "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", true)
    test_reduce("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]", true)
    test_reduce("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", true)
    test_reduce("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", false)
end

@testset "add" begin
    arg1 = read_snailfishnumber("[[[[4,3],4],4],[7,[[8,4],9]]]")
    arg2 = read_snailfishnumber("[1,1]")
    after_addition = read_snailfishnumber("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")
    actual = add(arg1, arg2)
    @test actual == after_addition
end
