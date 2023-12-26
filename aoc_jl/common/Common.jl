module Common

export MinHeap

struct MinHeap{T}
    arr::Vector{Tuple{Int64, T}}
    MinHeap{T}() where {T} = new(Vector{Tuple{Int64, T}}())
end

function Base.isempty(mh::MinHeap)
    isempty(mh.arr)
end

function parentidx(i)
    i ÷ 2
end

function lchildidx(i)
    2 * i
end

function rchildidx(i)
    2 * i + 1
end

function insert!(minheap::MinHeap{T}, k::Int64, val::T) where {T}
    push!(minheap.arr, (k, val))
    current_k = k
    current_idx = length(minheap.arr)
    while true
        parenti = parentidx(current_idx)
        if parenti < 1
            break
        end
        parent = minheap.arr[parenti]
        if k < parent[1]
            minheap.arr[current_idx] = parent
            minheap.arr[parenti] = (k, val)
            current_idx = parenti
        else
            break
        end
    end
end

function extract!(minheap::MinHeap)
    if isempty(minheap.arr)
        error("empty heap")
    end
    if length(minheap.arr) == 1
        return pop!(minheap.arr)
    end
    result = minheap.arr[1]
    minheap.arr[1] = minheap.arr[end]
    pop!(minheap.arr)
    current_idx = 1
    k, val = minheap.arr[1]
    num_visits = 0
    while current_idx < length(minheap.arr)
        num_visits += 1
        lchildi = lchildidx(current_idx)
        rchildi = rchildidx(current_idx)
        if lchildi > length(minheap.arr)
            break
        end
        alternatives = [(minheap.arr[lchildi][1], lchildi)]
        if rchildi <= length(minheap.arr)
            push!(alternatives, (minheap.arr[rchildi][1], rchildi))
        end
        min_alternative_key = minimum(x -> x[1], alternatives)
        if min_alternative_key < k
            swap_idx = argmin(x -> x[1], alternatives)[2]
            minheap.arr[current_idx], minheap.arr[swap_idx] = minheap.arr[swap_idx], minheap.arr[current_idx]
            current_idx = swap_idx
        else
            break
        end
    end
    return result
end

end  # end module
