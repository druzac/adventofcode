include("soln.jl")

import Test

Test.@testset "intervals" begin
    Test.@test contains(Interval(-1, 3), -1)
    Test.@test contains(Interval(-1, 3), 3)
    Test.@test !contains(Interval(-1, 3), 5)
    Test.@test !contains(Interval(-1, 3), -2)
    Test.@test hasintersection(Interval(-4, 2), Interval(-1, 3))
    Test.@test hasintersection(Interval(-4, 2), Interval(-9, -1))
    Test.@test hasintersection(Interval(-4, 2), Interval(2, 3))
    Test.@test !hasintersection(Interval(-2, 2), Interval(3, 5))
    Test.@test contains(Interval(-8, 8), Interval(-8, 4))
    Test.@test contains(Interval(-8, 8), Interval(-4, 8))
    Test.@test !contains(Interval(-8, 8), Interval(-4, 9))
    Test.@test split_interval(Interval(-2, 4), Interval(0, 3)) == (Interval(-2, -1), Interval(0, 4))
    Test.@test split_interval(Interval(0, 4), Interval(0, 3)) == (Interval(4, 4), Interval(0, 3))
    Test.@test split_interval(Interval(1, 2), Interval(0, 3)) == (nothing, Interval(1, 2))
end

Test.@testset "cubes" begin
    Test.@test subtract(Cube(Interval(1, 3), Interval(1, 3), Interval(1, 3)),
                        Cube(Interval(2, 3), Interval(1, 3), Interval(1, 3))) ==
                            [Cube(Interval(1, 1), Interval(1, 3), Interval(1, 3))]
    Test.@test subtract(Cube(Interval(2, 5), Interval(1, 3), Interval(1, 3)),
                        Cube(Interval(2, 3), Interval(1, 3), Interval(1, 3))) ==
                            [Cube(Interval(4, 5), Interval(1, 3), Interval(1, 3))]

end
