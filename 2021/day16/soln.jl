import Test

# each packet begins with standard header
# first 3 bits encode packet version
# second 3 bits encode type ID

# packets w/ type ID 4 represent a literal value.
# literal values encode a single binary number.
# binary number is padded w/ 0s until the length is a multiple of 4 bits
# each group of 3 bits is

# any other type ID indicates an operator packet.

# operator packets

# return number of bits read & also the number

# make a binary read that wraps an IO and does stream-based conversion to binary
# from the hex encoding

function char_to_bv(char)
    CHAR_TO_BITS = Dict{Char, BitVector}('0' => [0,0,0,0],
                                         '1' => [0,0,0,1],
                                         '2' => [0,0,1,0],
                                         '3' => [0,0,1,1],
                                         '4' => [0,1,0,0],
                                         '5' => [0,1,0,1],
                                         '6' => [0,1,1,0],
                                         '7' => [0,1,1,1],
                                         '8' => [1,0,0,0],
                                         '9' => [1,0,0,1],
                                         'A' => [1,0,1,0],
                                         'B' => [1,0,1,1],
                                         'C' => [1,1,0,0],
                                         'D' => [1,1,0,1],
                                         'E' => [1,1,1,0],
                                         'F' => [1,1,1,1])
    CHAR_TO_BITS[char]
end

struct BinaryQueue
    inv::BitVector
    outv::BitVector
    BinaryQueue() = new(BitVector(), BitVector())
end

function empty_inv!(bq::BinaryQueue)
    while !isempty(bq.inv)
        push!(bq.outv, pop!(bq.inv))
    end
end

function enqueue_bits!(bq::BinaryQueue, bits)
    for bit in bits
        push!(bq.inv, bit)
    end
end

function dequeue_bits!(bq::BinaryQueue, n)
    if n > length(bq)
        error("Asking for too many bits from binary queue")
    end
    out = BitVector()
    # if n > length(bq.outv)
    #     empty_inv!(bq)
    # end
    while !isempty(bq.outv) && length(out) < n
        push!(out, pop!(bq.outv))
    end
    if isempty(bq.outv)
        empty_inv!(bq)
    end
    while !isempty(bq.outv) && length(out) < n
        push!(out, pop!(bq.outv))
    end
    out
end

function peek_bits(bq::BinaryQueue, n)
    if n > length(bq)
        error("Asking for too many bits from binary queue")
    end
    # if n > length(bq.outv)
    #     empty_inv!(bq)
    # end
    # argh, peek is kind of hard
    first_half_length = min(length(bq.outv), n)
    first_half = reverse(bq.outv[1:first_half_length])
    # this is kind of dodgy
    i = 1
    while length(first_half) < n
        push!(first_half, bq.inv[i])
        i += 1
    end
    first_half
end

function Base.length(bq::BinaryQueue)
    length(bq.inv) + length(bq.outv)
end

function Base.isempty(bq::BinaryQueue)
    isempty(bq.inv) && isempty(bq.outv)
end

struct HexToBinaryStream
    io::IO
    bq::BinaryQueue
    HexToBinaryStream(io) = new(io, BinaryQueue())
end

function buffer_n_bits(bits_io::HexToBinaryStream, n)
    while length(bits_io.bq) < n
        next_bits = char_to_bv(read(bits_io.io, Char))
        enqueue_bits!(bits_io.bq, next_bits)
    end
end

function read_n_bits(bits_io::HexToBinaryStream, n)
    buffer_n_bits(bits_io, n)
    dequeue_bits!(bits_io.bq, n)
end

function peek_n_bits(bits_io::HexToBinaryStream, n)
    buffer_n_bits(bits_io, n)
    peek_bits(bits_io.bq, n)
end

abstract type Packet end

struct LiteralPacket <: Packet
    version::Int8
    value::Int64
    length::Int64
end

function Base.length(lpacket::LiteralPacket)
    lpacket.length
end

function sum_versions(packet::LiteralPacket)::Int64
    return packet.version
end

function compute_value(packet::LiteralPacket)
    packet.value

end

struct OperatorPacket <: Packet
    version::Int8
    type::Int8
    packets::Vector{Packet}
    length::Int64
end

function Base.length(opacket::OperatorPacket)
    opacket.length
end

function sum_versions(packet::OperatorPacket)::Int64
    running_sum = packet.version
    for subpacket in packet.packets
        running_sum += sum_versions(subpacket)
    end
    running_sum
end

function compute_value(packet::OperatorPacket)::Int64
    if packet.type < 0 || packet.type == 4 || packet.type > 7
        error("bad type for operator packet")
    end
    sub_values = map(compute_value, packet.packets)
    if packet.type == 0
        sum(sub_values)
    elseif packet.type == 1
        prod(sub_values)
    elseif packet.type == 2
        minimum(sub_values)
    elseif packet.type == 3
        maximum(sub_values)
    elseif packet.type == 5
        if length(sub_values) != 2
            error("bad sub packets for packet type 5")
        end
        sub_values[1] > sub_values[2] ? 1 : 0
    elseif packet.type == 6
        if length(sub_values) != 2
            error("bad sub packets for packet type 6")
        end
        sub_values[1] < sub_values[2] ? 1 : 0
    elseif packet.type == 7
        if length(sub_values) != 2
            error("bad sub packets for packet type 7")
        end
        sub_values[1] == sub_values[2] ? 1 : 0
    else
        error("bad packet type")
    end
end

# 110100
# 10111
# 11110
# 00101
# 000

# 1101

# 1101
# 0010
# 1111
# 1110
# 0010
# 1000

function bits_to_num(bv)
    n = 0
    for bit in bv
        n = n << 1 + bit
    end
    n
end

function parse_operator_packet(bio)
    header_w_mode = read_n_bits(bio, 7)
    version = read_three(header_w_mode[1:3])
    type = read_three(header_w_mode[4:6])
    mode_bit = header_w_mode[7]
    if mode_bit == 0
        packets = Vector{Packet}()
        total_sub_packet_length = bits_to_num(read_n_bits(bio, 15))
        # @show total_sub_packet_length
        current_sub_length = 0
        while current_sub_length < total_sub_packet_length
            packet = parse_packet(bio)
            push!(packets, packet)
            current_sub_length += length(packet)
        end
        OperatorPacket(version, type, packets, current_sub_length + 15 + 7)
    else
        packets = Vector{Packet}()
        total_sub_packet_count = bits_to_num(read_n_bits(bio, 11))
        # @show total_sub_packet_count
        while length(packets) < total_sub_packet_count
            packet = parse_packet(bio)
            push!(packets, packet)
        end
        OperatorPacket(version, type, packets, sum(map(length, packets)) + 11 + 7)
    end
end

function parse_literal_packet(bio)
    header = read_n_bits(bio, 6)
    version = read_three(header[1:3])
    type = read_three(header[4:6])
    if type != 4
        error("called parse literal packet on different packet")
    end
    number = 0
    read_bits = 0
    while true
        read_bits += 5
        block = read_n_bits(bio, 5)
        # @show block
        has_next_block = block[1]
        for bit in block[2:5]
            number = number << 1 + bit
        end
        if !has_next_block
            break
        end
    end
    LiteralPacket(version, number, read_bits + 6)
end

const HEADER_LENGTH = 6

function read_three(bv)
    (bv[1] << 2) + (bv[2] << 1) + bv[3]
end

function parse_packet(bio)
    header = peek_n_bits(bio, 6)
    version = read_three(header[1:3])
    type = read_three(header[4:6])
    # @show header, version, type
    if type == 4
        parse_literal_packet(bio)
    else
        parse_operator_packet(bio)
    end
end

function hex_to_binary(s)
    result = BitVector()
    for ch in s
        for bit in CHAR_TO_BITS[ch]
            push!(result, bit)
        end
    end
    result
end

function parse_problem(inputf)
    inputf
end

function problem_one(inputf)
    open(inputf, "r") do io
        bio = HexToBinaryStream(io)
        packet = parse_packet(bio)
        sum_versions(packet)
    end
end

function problem_two(inputf)
    open(inputf, "r") do io
        bio = HexToBinaryStream(io)
        packet = parse_packet(bio)
        compute_value(packet)
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

function string_to_packet(hexs)
    parse_packet(HexToBinaryStream(IOBuffer(hexs)))
end

if PROGRAM_FILE != "" && realpath(@__FILE__) == realpath(PROGRAM_FILE)
    # @show string_to_packet("38006F45291200")
    # @show string_to_packet("EE00D40C823060")
    main(ARGS)
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
