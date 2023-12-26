function parse_problem(inputf)
    readlines(inputf)
end

function character_score(char)
    if char == ')'
        3
    elseif char == ']'
        57
    elseif char == '}'
        1197
    elseif char == '>'
        25137
    else
        error("unknown character")
    end
end

CLOSERS = Set{Char}([')', ']', '}', '>'])
OPENERS = Set{Char}(['(', '[', '{', '<'])
PAIRS = Set{Tuple{Char, Char}}([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')])

function closer(c)
    c in CLOSERS
end

function opener(c)
    c in OPENERS
end

function match_chars(left, right)
    (left, right) in PAIRS
end

function score_line(line)
    stack = Vector{Char}()
    for c in line
        if opener(c)
            push!(stack, c)
        elseif closer(c)
            if length(stack) == 0 || !match_chars(pop!(stack), c)
                return character_score(c)
            end
        else
            error("unknown character")
        end
    end
    0
end


function character_complete_score(char)
    if char == '('
        1
    elseif char == '['
        2
    elseif char == '{'
        3
    elseif char == '<'
        4
    else
        error("unknown character")
    end
end

function complete_line(line)
    stack = Vector{Char}()
    for c in line
        if opener(c)
            push!(stack, c)
        elseif closer(c)
            if length(stack) == 0 || !match_chars(pop!(stack), c)
                return 0
            end
        else
            error("unknown character")
        end
    end
    score = 0
    while !isempty(stack)
        score = score * 5 + character_complete_score(pop!(stack))
    end
    score
end

function problem_one(problem)
    score = 0
    for line in problem
        line_score = score_line(line)
        score += line_score
    end
    score
end

function problem_two(problem)
    scores = Vector{Int64}()
    for line in problem
        line_score = complete_line(line)
        if line_score > 0
            push!(scores, line_score)
        end
    end
    if length(scores) % 2 != 1
        error("bad length")
    end
    sort(scores)[length(scores) ÷ 2 + 1]
end
