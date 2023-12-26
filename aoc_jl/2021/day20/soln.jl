struct Image
    bm::BitMatrix
    infinite_pixel::Bool
end

function Base.getindex(image::Image, i::Int64, j::Int64)
    n_rows, n_cols = size(image.bm)
    if i >= 1 && i <= n_rows && j >= 1 && j <= n_cols
        image.bm[i, j]
    else
        image.infinite_pixel
    end
end

function Base.size(image::Image)
    size(image.bm)
end

function sumpixels(image::Image)
    if image.infinite_pixel
        error("infinite pixels are on")
    end
    sum(image.bm)
end

function parse_problem(inputf)
    chartobit(ch) = begin
        if ch == '#'
            true
        elseif ch == '.'
            false
        else
            error("unrecognized char: $ch")
        end
    end
    open(inputf, "r") do io
        bv = BitVector(map(chartobit, collect(readline(io))))

        readline(io)
        first_line = readline(io)
        l = length(first_line)
        bm = BitMatrix(undef, l, l)
        bm[1, :] = BitVector(map(chartobit, collect(first_line)))
        for i in 2:l
            bm[i, :] = BitVector(map(chartobit, collect(readline(io))))
        end
        lastline = readline(io)
        if lastline != ""
            error("should be eof, instead is: $lastline")
        end
        (bv, Image(bm, false))
    end
end

function zero_frame(bm)
    if any(bm[1:2, :]) || any(bm[end-1:end, :]) || any(bm[:, 1:2]) || any(bm[:, end-1:end])
        new_bm = falses(4 .+ size(bm))
        n_rows, n_cols = size(new_bm)
        new_bm[3:end-2, 3:end-2] = bm
        new_bm
    else
        bm
    end
end

function display_image(bm)
    n_rows, n_cols = size(bm)
    for i in 1:n_rows
        for j in 1:n_cols
            char = bm[i, j] ? '#' : '.'
            print(char)
        end
        println()
    end
end

function bv_to_integer(bv)
    sum(val << (length(bv) - i) for (i, val) in enumerate(bv))
end

function lookupbit(image_lookup, i, j, image::Image)
    bv = [image[ip, jp] for ip in i - 1:i + 1
              for jp in j - 1:j + 1]
    value = bv_to_integer(bv)
    image_lookup[value + 1]
end

function image_enhance(image_lookup, image::Image)
    output_bm = BitMatrix(undef, size(image) .+ 2)
    n_rows, n_cols = size(output_bm)
    for j in 1:n_cols
        for i in 1:n_rows
            output_bm[i, j] = lookupbit(image_lookup, i - 1, j - 1, image)
        end
    end
    infinite_idx = image.infinite_pixel ? length(image_lookup) : 1
    Image(output_bm, image_lookup[infinite_idx])
end

function image_enhance(image_lookup, image::Image, cnt::Int64)
    output_image = image
    for _ in 1:cnt
        output_image = image_enhance(image_lookup, output_image)
    end
    output_image
end

function problem_one(problem)
    image_lookup, input_image = problem
    output_image = image_enhance(image_lookup, input_image, 2)
    sumpixels(output_image)
end

function problem_two(problem)
    image_lookup, input_image = problem
    output_image = image_enhance(image_lookup, input_image, 50)
    sumpixels(output_image)
end
