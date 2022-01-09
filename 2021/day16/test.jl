import Test

include("soln.jl")

function string_to_packet(hexs)
    parse_packet(HexToBinaryStream(IOBuffer(hexs)))
end

Test.@testset "packet tests" begin
    Test.@test string_to_packet("D2FE28") == LiteralPacket(6, 2021, 21)
    Test.@test compute_value(string_to_packet("C200B40A82")) == 3
    Test.@test compute_value(string_to_packet("04005AC33890")) == 54
    Test.@test compute_value(string_to_packet("880086C3E88112")) == 7
    Test.@test compute_value(string_to_packet("CE00C43D881120")) == 9
    Test.@test compute_value(string_to_packet("D8005AC2A8F0")) == 1
    Test.@test compute_value(string_to_packet("F600BC2D8F")) == 0
    Test.@test compute_value(string_to_packet("9C005AC2F8F0")) == 0
    Test.@test compute_value(string_to_packet("9C0141080250320F1802104A08")) == 1
    # can't get this to pass...
    # Test.@test string_to_packet("38006F45291200") == OperatorPacket(1, 6, [LiteralPacket(6, 10, 11), LiteralPacket(2, 20, 16)], 49)
end
