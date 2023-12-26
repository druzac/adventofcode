
abstract type Instruction end

struct Acc <: Instruction
    operand::Int64
end

struct Jump <: Instruction
    relativeoffset::Int64
end

function getoperand(instr::Jump)
    instr.relativeoffset
end

struct Noop <: Instruction
    operand::Int64
end

function getoperand(instr::Noop)
    instr.operand
end

function parse_instruction(line)
    words = split(line)
    if length(words) != 2
        error("line doesn't contain two words: $line")
    end
    arg = parse(Int64, words[2])
    if words[1] == "acc"
        Acc(arg)
    elseif words[1] == "jmp"
        Jump(arg)
    elseif words[1] == "nop"
        Noop(arg)
    else
        error("unrecognized command: $(words[1])")
    end
end

mutable struct CpuState
    pc::Int64
    acc::Int64
end

function execute!(state::CpuState, accinstr::Acc)
    state.pc += 1
    state.acc += accinstr.operand
end

function execute!(state::CpuState, jumpinstr::Jump)
    state.pc += jumpinstr.relativeoffset
end

function execute!(state::CpuState, noopinstr::Noop)
    state.pc += 1
end

function isacc(accinstr::Acc)
    true
end

function isacc(jumpinstr::Jump)
    false
end

function isacc(noopinstr::Noop)
    false
end

function isjmp(jmpinstr::Jump)
    true
end

function isjmp(instr::Noop)
    false
end

function isjmp(instr::Acc)
    false
end

function parse_problem(inputf)
    [parse_instruction(line) for line in eachline(inputf)]
end

function swapjmpnop!(program, linenum)
    if isjmp(program[linenum])
        program[linenum] = Noop(getoperand(program[linenum]))
    else
        program[linenum] = Jump(getoperand(program[linenum]))
    end
end

function run_program(program)
    executed_instructions = Set{Int64}()
    state = CpuState(1, 0)
    while state.pc <= length(program)
        if state.pc in executed_instructions
            return (state.acc, executed_instructions, false)
        end
        push!(executed_instructions, state.pc)
        execute!(state, program[state.pc])
    end
    (state.acc, executed_instructions, true)
end

function problem_one(program)
    run_program(program)[1]
end

function problem_two(program)
    visited_instructions = [linenum for linenum in run_program(program)[2] if !isacc(program[linenum])]
    visited_instructions, length(visited_instructions)
    for linenum in visited_instructions
        swapjmpnop!(program, linenum)
        result = run_program(program)
        if result[3]
            return result[1]
        end
        swapjmpnop!(program, linenum)
    end
    error("didn't find a solution!")
end
