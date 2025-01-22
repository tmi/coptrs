import fire
from time import perf_counter_ns
import sys
from coptrs import floyd_warshall as rs_floyd_warshall
from concurrent.futures import ThreadPoolExecutor, wait

Matrix = dict[tuple[int, int], int]
def py_floyd_warshall(edges: Matrix) -> Matrix:
    vertices = set()
    for (a, b) in edges.keys():
        vertices.add(a)
        vertices.add(b)

    dist = {(a, a): 0 for a in vertices}
    for ((a, b), w) in edges.items():
        dist[(a, b)] = w

    for c in vertices:
        for a in vertices:
            for b in vertices:
                if (a, c) in dist and (c, b) in dist:
                    x = dist[(a, c)] + dist[(c, b)]
                    if (a, b) in dist:
                        if dist[(a, b)] > x:
                            dist[(a, b)] = x
                    else:
                        dist[(a, b)] = x

    return dist

def get_graph(n: int) -> Matrix:
    return {**{
        (i, i+1): 1
        for i in range(n)
    },**{
        (i+1, i): 1
        for i in range(n)
    }}

def main(e: int, m: str, t: int = 1) -> None:
    g = get_graph(2**e)
    tp = ThreadPoolExecutor(max_workers=t/2)
    if m == 'eq':
        py = py_floyd_warshall(g)
        rs = rs_floyd_warshall(g)
        # print(sorted(list(py.items())))
        # print(sorted(list(rs.items())))
        assert py == rs
    elif m == 'py':
        start = perf_counter_ns()
        if t == 1:
            py = py_floyd_warshall(g)
        else:
            futs = [tp.submit(py_floyd_warshall, g) for _ in range(t)]
            wait(futs)
        end = perf_counter_ns()
        print(f"py took {(end-start)/1e6:.3f}ms")
    elif m == 'rs':
        start = perf_counter_ns()
        if t == 1:
            rs = rs_floyd_warshall(g)
        else:
            futs = [tp.submit(rs_floyd_warshall, g) for _ in range(t)]
            wait(futs)
        end = perf_counter_ns()
        print(f"rs took {(end-start)/1e6:.3f}ms")
    else:
        raise NotImplementedError(sys.argv[1])

if __name__ == "__main__":
    fire.Fire(main)
