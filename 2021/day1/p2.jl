import DelimitedFiles

function main(args)
    input = args[1]
    @show input
    v = DelimitedFiles.readdlm(input, Int64)

    v0 = @view v[1: end - 2]
    v1 = @view v[2: end - 1]
    v2 = @view v[3: end]

    windows = v0 + v1 + v2

    firsts = @view windows[1: end - 1]
    seconds = @view windows[2: end]
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
