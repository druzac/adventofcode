import Test

abstract type SnailfishNumber end

@enum Direction left right

const ParentLink = Union{Nothing, Tuple{Direction, SnailfishNumber}}

mutable struct SnailfishPair <: SnailfishNumber
    left::SnailfishNumber
    right::SnailfishNumber
    parent::ParentLink
end

function add(sfn1::SnailfishNumber, sfn2::SnailfishNumber)
    new_root = SnailfishPair(sfn1, sfn2, nothing)
    set_parent!(sfn1, new_root, left)
    set_parent!(sfn2, new_root, right)
    new_root
end

function add_and_fully_reduce!(sfn1::SnailfishNumber, sfn2::SnailfishNumber)
    new_root = add(sfn1, sfn2)
    while reduce!(new_root) end
    new_root
end

function magnitude(sfp::SnailfishPair)
    lchild, rchild = get_children(sfp)
    3 * magnitude(lchild) + 2 * magnitude(rchild)
end

function set_parent!(pair::SnailfishPair, parent::SnailfishPair, dir::Direction)
    pair.parent = (dir, parent)
end

function set_lchild!(pair::SnailfishPair, sfn::SnailfishNumber)
    pair.left = sfn
end

function set_rchild!(pair::SnailfishPair, sfn::SnailfishNumber)
    pair.right = sfn
end

function get_children(pair::SnailfishPair)
    (pair.left, pair.right)
end

function get_parent(pair::SnailfishPair)
    pair.parent
end

function get_value(sfp::SnailfishPair)
    nothing
end

function ispair(sfp::SnailfishPair)
    true
end

function set_value!(sfp::SnailfishPair)
    error("trying to set the value of a pair")
end

function explode!(sfp::SnailfishPair)
    # go up while it's a left child
    left_val = get_value(sfp.left)
    right_val = get_value(sfp.right)
    if left_val == nothing || right_val == nothing
        error("pair to explode doesn't have literal children")
    end
    maybe_parent = sfp.parent
    if maybe_parent == nothing
        error("trying to explode the root pair")
    end
    while maybe_parent != nothing && maybe_parent[1] == left
        maybe_parent = get_parent(maybe_parent[2])
    end
    if maybe_parent != nothing
        # we just went up a right link, so go down a left link
        neighbour = get_children(maybe_parent[2])[1]
        while ispair(neighbour)
            # go down a right link
            neighbour = get_children(neighbour)[2]
        end
        # we should have a literal now
        set_value!(neighbour, left_val + get_value(neighbour))
    end
    maybe_parent = sfp.parent
    while maybe_parent != nothing && maybe_parent[1] == right
        maybe_parent = get_parent(maybe_parent[2])
    end
    if maybe_parent != nothing
        # just went up a left link, go down a right link
        neighbour = get_children(maybe_parent[2])[2]
        while ispair(neighbour)
            neighbour = get_children(neighbour)[1]
        end
        set_value!(neighbour, right_val + get_value(neighbour))
    end
    # now remove this node, and replace it with
    new_node = SnailfishLiteral(0, sfp.parent)
    immediate_parent = sfp.parent[2]
    if sfp.parent[1] == left
        set_lchild!(immediate_parent, new_node)
    elseif sfp.parent[1] == right
        set_rchild!(immediate_parent, new_node)
    else
        error("unrecognized dir: ", sfp.parent[1])
    end
end

function split!(sfp::SnailfishPair)
    error("splitting a pair")
end

# just ignore the parent links
function Base.:(==)(sfp1::SnailfishPair, sfp2::SnailfishPair)
    sfp1.left == sfp2.left && sfp1.right == sfp2.right
end

function pretty_print(sfn::SnailfishPair)
    l = pretty_print(sfn.left)
    r = pretty_print(sfn.right)
    join(["[", l, ",", r, "]"])
end

mutable struct SnailfishLiteral <: SnailfishNumber
    n::Int64
    parent::ParentLink
end

function magnitude(sfl::SnailfishLiteral)
    get_value(sfl)
end

function set_parent!(literal::SnailfishLiteral, parent::SnailfishPair, dir::Direction)
    literal.parent = (dir, parent)
end

function set_lchild!(literal::SnailfishLiteral, sfn::SnailfishNumber)
    error("trying to add a left child to a literal")
end

function set_rchild!(literal::SnailfishLiteral, sfn::SnailfishNumber)
    error("trying to add a right child to a literal")
end

function get_children(sfl::SnailfishLiteral)
    nothing
end

function get_parent(sfl::SnailfishLiteral)
    sfl.parent
end

function get_value(sfl::SnailfishLiteral)
    sfl.n
end

function set_value!(sfl::SnailfishLiteral, new_val::Int64)
    sfl.n = new_val
end

function ispair(sfl::SnailfishLiteral)
    false
end

function explode!(sfl::SnailfishLiteral)
    error("exploding a literal")
end

# if there is no parent, need to return a new tree.
function split!(sfl::SnailfishLiteral)
    leftv = trunc(Integer, floor(sfl.n / 2))
    rightv = trunc(Integer, ceil(sfl.n / 2))
    if sfl.parent == nothing
        error("oops, trying to split a number with no parent")
    end
    dir, sfn = sfl.parent
    leftc = SnailfishLiteral(leftv, nothing)
    rightc = SnailfishLiteral(rightv, nothing)
    new_node = SnailfishPair(leftc, rightc, (dir, sfn))
    set_parent!(leftc, new_node, left)
    set_parent!(rightc, new_node, right)
    if dir == left
        set_lchild!(sfn, new_node)
    elseif dir == right
        set_rchild!(sfn, new_node)
    else
        error("bad dir: ", dir)
    end
end

function Base.:(==)(sfl1::SnailfishLiteral, sfl2::SnailfishLiteral)
    sfl1.n == sfl2.n
end

function pretty_print(sfl::SnailfishLiteral)
    string(sfl.n)
end

DIGITS = Set(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])

# returns true if we took an action, false otherwise.
function reduce!(sfn::SnailfishNumber)
    to_expand = Vector{Tuple{SnailfishNumber, Int64}}()
    push!(to_expand, (sfn, 0))
    to_split = nothing
    while !isempty(to_expand)
        current_node, containing_pairs = pop!(to_expand)
        if ispair(current_node) && containing_pairs >= 4
            if containing_pairs != 4
                error("too many containing pairs")
            end
            # println("explode")
            explode!(current_node)
            return true
        end
        children = get_children(current_node)
        if children != nothing
            push!(to_expand, (children[2], containing_pairs + 1))
            push!(to_expand, (children[1], containing_pairs + 1))
        end
        if to_split == nothing
            curr_value = get_value(current_node)
            if curr_value != nothing && curr_value > 9
                to_split = current_node
            end
        end
    end
    if to_split != nothing
        # println("split")
        split!(to_split)
        return true
    end
    false
end

function parse_literal(chars)
    last_digit_pos = findfirst(ch -> !(ch in DIGITS), chars) - 1
    if last_digit_pos == nothing
        last_digit_pos = length(chars)
    end
    n = parse(Int64, chars[1:last_digit_pos])
    (SnailfishLiteral(n, nothing), chars[last_digit_pos + 1:end])
end

function parse_pair(chars)
    if first(chars) != '['
        error("invalid argument: ", first(chars))
    end
    leftc, rest1 = parse_snailfishnumber(chars[2:end])
    if rest1[1] != ','
        error("invalid pair: expected ',' but found: ", rest1[1])
    end
    rightc, rest2 = parse_snailfishnumber(rest1[2:end])
    if rest2[1] != ']'
        error("invalid pair: expected ']' but found: ", rest2[1])
    end
    pair = SnailfishPair(leftc, rightc, nothing)
    set_parent!(leftc, pair, left)
    set_parent!(rightc, pair, right)
    pair, rest2[2:end]
end

function parse_snailfishnumber(line)
    ch = first(line)
    if ch == '['
        return parse_pair(line)
    elseif ch in DIGITS
        return parse_literal(line)
    else
        error("unexpected character: ", ch)
    end
end

function read_snailfishnumber(s)
    parse_snailfishnumber(s)[1]
end

function parse_problem(inputf)
    sf_numbers = Vector{SnailfishNumber}()
    for line in eachline(inputf)
        push!(sf_numbers, read_snailfishnumber(line))
    end
    sf_numbers
end

function problem_one(problem)
    l = problem[1]
    r = problem[2]
    result = add_and_fully_reduce!(l, r)
    for summand in problem[3:end]
        result = add_and_fully_reduce!(result, summand)
    end
    magnitude(result)
end

function problem_two(problem)
    maximum(Iterators.flatten((magnitude(add_and_fully_reduce!(deepcopy(problem[i]), deepcopy(problem[j]))),
                               magnitude(add_and_fully_reduce!(deepcopy(problem[j]), deepcopy(problem[i]))))
                              for i in 1:length(problem)
                                  for j in i + 1:length(problem)))
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

function test_reduce(input, expected, reduce_action)
    arg = read_snailfishnumber(input)
    expected = read_snailfishnumber(expected)
    Test.@test reduce!(arg) == reduce_action
    Test.@test arg == expected
end

Test.@testset "reduce" begin
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

Test.@testset "add" begin
    arg1 = read_snailfishnumber("[[[[4,3],4],4],[7,[[8,4],9]]]")
    arg2 = read_snailfishnumber("[1,1]")
    after_addition = read_snailfishnumber("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")
    actual = add(arg1, arg2)
    Test.@test actual == after_addition
end
