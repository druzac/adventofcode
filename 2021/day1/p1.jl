import DelimitedFiles

function main(args)
    input = args[1]
    @show input
    v = DelimitedFiles.readdlm(input, Int64)

    firsts = @view v[1: end - 1]
    seconds = @view v[2: end]
    @show count(firsts .< seconds)
    # prev = nothing
    # count = 0
    # for line in eachline(input)
    #     n = tryparse(Int64, line)
    #     if (prev != nothing && prev < n)
    #         count += 1
    #     end
    #     @show prev, n
    #     prev = n
    # end
    # @show count
end

main(ARGS)
