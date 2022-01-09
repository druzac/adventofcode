include("soln.jl")

using Test

@testset "intervals" begin
    @test contains(Interval(-1, 3), -1)
    @test contains(Interval(-1, 3), 3)
    @test !contains(Interval(-1, 3), 5)
    @test !contains(Interval(-1, 3), -2)
    @test hasintersection(Interval(-4, 2), Interval(-1, 3))
    @test hasintersection(Interval(-4, 2), Interval(-9, -1))
    @test hasintersection(Interval(-4, 2), Interval(2, 3))
    @test !hasintersection(Interval(-2, 2), Interval(3, 5))
    @test contains(Interval(-8, 8), Interval(-8, 4))
    @test contains(Interval(-8, 8), Interval(-4, 8))
    @test !contains(Interval(-8, 8), Interval(-4, 9))
    @test split_interval(Interval(-2, 4), Interval(0, 3)) == (Interval(-2, -1), Interval(0, 4))
    @test split_interval(Interval(0, 4), Interval(0, 3)) == (Interval(4, 4), Interval(0, 3))
    @test split_interval(Interval(1, 2), Interval(0, 3)) == (nothing, Interval(1, 2))
end

@testset "cubes" begin
    @test subtract(Cube(Interval(1, 3), Interval(1, 3), Interval(1, 3)),
                   Cube(Interval(2, 3), Interval(1, 3), Interval(1, 3))) ==
                       [Cube(Interval(1, 1), Interval(1, 3), Interval(1, 3))]
    @test subtract(Cube(Interval(2, 5), Interval(1, 3), Interval(1, 3)),
                   Cube(Interval(2, 3), Interval(1, 3), Interval(1, 3))) ==
                       [Cube(Interval(4, 5), Interval(1, 3), Interval(1, 3))]
end
