using Test

include("soln.jl")

@testset "line segment iteration" begin
    p1 = Point(1, 2)
    p2 = Point(1, 5)
    ls = LineSegment(p1, p2)
    ls_rev = LineSegment(p2, p1)
    expected_result = [p1, Point(1, 3), Point(1, 4), p2]
    @test collect(ls) == expected_result
    @test collect(ls_rev) == reverse(expected_result)
    @test collect(LineSegment(p1, p1)) == [p1]
    @test collect(LineSegment(p2, p2)) == [p2]

    p3 = Point(2, 1)
    p4 = Point(5, 1)
    expected_result2 = [p3, Point(3, 1), Point(4, 1), p4]
    @test collect(LineSegment(p3, p4)) == expected_result2
    @test collect(LineSegment(p4, p3)) == reverse(expected_result2)
    @test collect(LineSegment(p3, p3)) == [p3]
    @test collect(LineSegment(p4, p4)) == [p4]

    pd1 = Point(1, 1)
    pd2 = Point(3, 3)
    pd3 = Point(9, 7)
    pd4 = Point(7, 9)
    diag_expected_1 = [pd1, Point(2, 2), pd2]
    diag_expected_2 = [pd3, Point(8, 8), pd4]
    @test collect(LineSegment(pd1, pd2)) == diag_expected_1
    @test collect(LineSegment(pd2, pd1)) == reverse(diag_expected_1)
    @test collect(LineSegment(pd3, pd4)) == diag_expected_2
    @test collect(LineSegment(pd4, pd3)) == reverse(diag_expected_2)
end
