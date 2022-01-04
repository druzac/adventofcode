struct Inputs
    v::Vector{Int64}
    Inputs(raw) = new(reverse([ip for ip in raw]))
end

function nextinput!(inputs::Inputs)
    if isempty(inputs.v)
        error("No more inputs!")
    end
    pop!(inputs.v)
end

struct ALUState
    variables::Vector{Int64}
    ALUState() = new([0, 0, 0, 0])
end

function symtoidx(sym::Symbol)
    if sym == :w
        1
    elseif sym == :x
        2
    elseif sym == :y
        3
    elseif sym == :z
        4
    else
        error("invalid sym: $sym")
    end
end

function resolve(state::ALUState, sym::Symbol)
    state.variables[symtoidx(sym)]
end

function resolve(state::ALUState, literal::Int64)
    literal
end

function write!(state::ALUState, sym::Symbol, value::Int64)
    state.variables[symtoidx(sym)] = value
end

abstract type Instruction end

const Operand = Union{Int64, Symbol}

struct InputInstruction <: Instruction
    dest::Symbol
end

function apply!(state::ALUState, inputs::Inputs, inp::InputInstruction)
    ni = nextinput!(inputs)
    write!(state, inp.dest, ni)
end

@enum Operator plus mult dv md eql

function apply_op(op::Operator, a::Int64, b::Int64)
    if op == plus
        a + b
    elseif op == mult
        a * b
    elseif op == dv
        a ÷ b
    elseif op == md
        a % b
    elseif op == eql
        a == b ? 1 : 0
    else
        error("unknown op: $op")
    end
end

struct BinaryOperationInstruction <: Instruction
    dest::Symbol
    operand::Operand
    op::Operator
end

function apply!(state::ALUState, inputs::Inputs, binOp::BinaryOperationInstruction)
    curr_val = resolve(state, binOp.dest)
    new_val = apply_op(binOp.op, curr_val, resolve(state, binOp.operand))
    write!(state, binOp.dest, new_val)
end

function run_program(instrs, inputs::Inputs)
    alu_state = ALUState()
    for instr in instrs
        apply!(alu_state, inputs, instr)
    end
    alu_state
end

function run_program_to_nth(instrs::Vector{Instruction}, v, n::Int64)
    inputs = Inputs(v)
    run_program(Iterators.take(instrs, n), inputs)
end

struct Gate
    line_number::Int64
    unlockable::Bool
end

function unlock_gates(instrs::Vector{Instruction}, gates, args, n, rng)
    if length(gates) < n
        return args
    end
    current_gate = gates[n]
    for current_arg in rng
        catted_args = [args; current_arg]
        if current_gate.unlockable
            state = run_program_to_nth(instrs, catted_args, current_gate.line_number)
            if resolve(state, :x) != 0
                continue
            end
        end
        result = unlock_gates(instrs, gates, catted_args, n + 1, rng)
        if !isnothing(result)
            return result
        end
    end
    nothing
end

function gate_approach(instrs, max_gates, rng)
    # gates are pairs of eql instructions, both writing to x.
    # some gates unconditionally write 1 to x given the constraints on the inputs,
    # others can be made to write a 0 into x. subsequent instructions increment z if
    # x is 1, so this program uses recursive backtracking to attempt to achieve as many
    # conditions as possible to keep z as small as possible.
    # the line number of the gates and whether or not they are unlockable were found by code
    # inspection of the input program.
    gates = [
        Gate(8, false),
        Gate(26, false),
        Gate(44, false),
        Gate(62, true),
        Gate(80, false),
        Gate(98, true),
        Gate(116, false),
        Gate(134, true),
        Gate(152, false),
        Gate(170, false),
        Gate(188, true),
        Gate(206, true),
        Gate(224, true),
        Gate(242, true)
    ]
    unlock_gates(instrs, gates[1:max_gates], [], 1, rng)
end

function tovariable(s)
    sym = Symbol(s)
    if sym != :w && sym != :x && sym != :y && sym != :z
        nothing
    else
        sym
    end
end

function tovariableorfail(s)
    result = tovariable(s)
    if isnothing(result)
        error("bad variable: $s")
    end
    result
end

function getop(instruction)
    if instruction == "add"
        plus
    elseif instruction == "mul"
        mult
    elseif instruction == "div"
        dv
    elseif instruction == "mod"
        md
    elseif instruction == "eql"
        eql
    else
        error("unknown instruction: $instruction")
    end
end

function getoperand(s)
    maybe_var = tovariable(s)
    if isnothing(maybe_var)
        parse(Int64, s)
    else
        maybe_var
    end
end

function parse_instruction(line)
    words = split(line)
    type = words[1]
    dest = tovariableorfail(words[2])
    if type == "inp"
        if length(words) != 2
            error("bad input instruction: $line")
        end
        InputInstruction(dest)
    else
        if length(words) != 3
            error("bad operator instruction: $line")
        end
        op = getop(type)
        operand = getoperand(words[3])
        BinaryOperationInstruction(dest, operand, op)
    end
end

function parse_problem(inputf)
    [parse_instruction(line) for line in eachline(inputf)]
end

function problem_one(instructions)
    join(gate_approach(instructions, 14, 9:-1:1), "")
end

function problem_two(instructions)
    join(gate_approach(instructions, 14, 1:9), "")
end

function main(args)
    problem_number = args[1]
    inputf = args[2]

    problem = parse_problem(inputf)
    if problem_number == "1"
        println(problem_one(problem))
    elseif problem_number == "2"
        println(problem_two(problem))
    else
        error("Need to put in 1 or 2")
    end
end

if PROGRAM_FILE != "" && realpath(@__FILE__) == realpath(PROGRAM_FILE)
    main(ARGS)
end

function createiobuffer(lines)
    IOBuffer(join(lines, '\n'))
end

function parse_string_vector(lines)
    parse_problem(createiobuffer(lines))
end

import Test

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
