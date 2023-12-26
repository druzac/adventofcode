function parse_problem(inputf)
    open(inputf, "r") do io
        passports = Vector{Dict{String, String}}()
        current_passport = Dict{String, String}()
        for line in eachline(io)
            if line == "" && !isempty(current_passport)
                push!(passports, current_passport)
                current_passport = Dict{String, String}()
            end
            for (key, val) in map(x -> split(x, ':'), split(line))
                current_passport[key] = val
            end
        end
        if !isempty(current_passport)
            push!(passports, current_passport)
        end
        passports
    end
end

const REQUIRED_FIELDS = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]

function isvalid(passport)
    all(haskey(passport, rfield) for rfield in REQUIRED_FIELDS)
end

function snuminbounds(snum, a, b)
    num = tryparse(Int64, snum)
    num != nothing && num <= b && num >= a
end

function isvalidbyr(bys)
    snuminbounds(bys, 1920, 2002)
end

function isvalidiyr(iys)
    snuminbounds(iys, 2010, 2020)
end

function isvalideyr(eys)
    snuminbounds(eys, 2020, 2030)
end

function isvalidhgt(hgt)
    if length(hgt) < 3
        return false
    end
    suffix = hgt[end-1:end]
    snum = hgt[1:end-2]
    if suffix == "cm"
        snuminbounds(snum, 150, 193)
    elseif suffix == "in"
        snuminbounds(snum, 59, 76)
    else
        false
    end
end

function isvalidhcl(hcl)
    occursin(r"^#[0-9a-f]{6}$", hcl)
end

const EYE_COLOURS = Set{String}(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"])

function isvalidecl(ecl)
    ecl in EYE_COLOURS
end

function isvalidpid(pid)
    occursin(r"^[0-9]{9}$", pid)
end

const TESTS = Dict("byr" => isvalidbyr, "iyr" => isvalidiyr, "eyr" => isvalideyr, "hgt" => isvalidhgt, "hcl" => isvalidhcl, "ecl" => isvalidecl, "pid" => isvalidpid)

function isvalidstrict(passport)
    for (field, fieldvalidator) in TESTS
        fieldval = get(passport, field, "")
        if !fieldvalidator(fieldval)
            return false
        end
    end
    true
end

function problem_one(passports)
    length(filter(isvalid, passports))
end

function problem_two(passports)
    length(filter(isvalidstrict, passports))
end
