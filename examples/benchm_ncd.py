import sys
from coptrs import nearest_common_descendant_u32 as rs_ncd_impl
from time import perf_counter_ns

ftime = lambda s, e: f"{(e-s)/1e6:.3f} ms"
Distances = dict[[int, int], int]
def py_ncd_cubic(distances: Distances) -> Distances:
    result = {}
    D = max(distances.values())
    vertices = list({a for (a, _) in distances.keys()})

    for a in vertices:
        for b in vertices:
            result[(a, b)] = D + 1
            result[(b, a)] = D + 1

    for c in vertices:
        outer_loop_start = perf_counter_ns()
        for a in vertices:
            inner_loop_start = perf_counter_ns()
            for b in vertices:
                m = result[(a, b)]
                n = max(distances.get((a, c), D+1), distances.get((b, c), D+1))
                if n < m:
                    result[(a, b)] = n
                    result[(b, a)] = n
            inner_loop_end = perf_counter_ns()
        outer_loop_end = perf_counter_ns()

    return result

def py_ncd_quad(distances: Distances) -> Distances:
    # we actually assume here that all edges have weight 1
    vertices = list({a for (a, _) in distances.keys()})
    D = max(distances.values())
    children = {
        v: [u for u in vertices if distances.get((v, u), 0) == 1]
        for v in vertices
    }
    parents = {
        v: [u for u in vertices if distances.get((u, v), 0) == 1]
        for v in vertices
    }
    remaining_children = {v: len(children[v]) for v in vertices}
    queue = [v for v in vertices if remaining_children[v] == 0] # start from sinks

    visiting_order = {}
    i = -1
    result = {}
    while queue:
        cycle_start = perf_counter_ns()
        v = queue.pop(0)
        i += 1
        visiting_order[v] = i
        result[v] = {}
        # for each u, do we have a common ancestor with it?
        for u in vertices:
            # we are their ancestor, nothing can be better
            if (u, v) in distances:
                result[v][u] = (v, i)
            # some of our children may have a common ancestor with it
            else:
                for c in children[v]:
                    if u in result[c]:
                        if u not in result[v]:
                            result[v][u] = result[c][u]
                        else:
                            _, maybe_i = result[c][u]
                            _, current_i = result[v][u]
                            if maybe_i > current_i:
                                result[v][u] = result[c][u]
        for p in parents[v]:
            remaining_children[p] -= 1
            if remaining_children[p] == 0:
                queue.append(p)
        cycle_end = perf_counter_ns()

    final_result = {}
    for v in vertices:
        for u in vertices:
            if u in result[v]:
                d = result[v][u][0]
                d = max(distances[v, d], distances[u, d])
                final_result[(v, u)] = d
                final_result[(u, v)] = d
            else:
                final_result[(v, u)] = D + 1
                final_result[(u, v)] = D + 1
    return final_result

def py_ncd_quad_nonunit(distances: Distances) -> Distances:
    # we dont assume anymore that edges have distance 1, at the cost of doing
    # more comparisons. Not 100% sure it is correct :)
    vertices = list({a for (a, _) in distances.keys()})
    D = max(distances.values())
    # actually here we still assume distance==1 <-> direct edge, but thats a detail
    children = {
        v: [u for u in vertices if distances.get((v, u), 0) == 1]
        for v in vertices
    }
    parents = {
        v: [u for u in vertices if distances.get((u, v), 0) == 1]
        for v in vertices
    }
    remaining_children = {v: len(children[v]) for v in vertices}
    queue = [v for v in vertices if remaining_children[v] == 0] # start from sinks

    result = {}
    while queue:
        cycle_start = perf_counter_ns()
        v = queue.pop(0)
        result[v] = {}
        # for each u, do we have a common ancestor with it?
        for u in vertices:
            # if we are their ancestor then we are a common ancestor, though not necessarily the nearest one
            if (u, v) in distances:
                result[v][u] = (v, distances[(u,v)])
            # some of our children may have a common ancestor with u
            for c in children[v]:
                if u in result[c]:
                    d = result[c][u][0]
                    dist = max(distances[(v, d)], distances[(u, d)])
                    if u not in result[v] or result[v][u][1] > dist:
                        result[v][u] = (d, dist)
        for p in parents[v]:
            remaining_children[p] -= 1
            if remaining_children[p] == 0:
                queue.append(p)
        cycle_end = perf_counter_ns()

    final_result = {}
    for v in vertices:
        for u in vertices:
            if u in result[v]:
                final_result[(v, u)] = result[v][u][1]
            else:
                final_result[(v, u)] = D + 1
    return final_result

def get_grid_graph(d: int, w: int) -> Distances:
    graph = {}
    # we built a grid graph of D layers, each having W vertices
    # we index by tuples (a,b) corresponding to (within-layer, which-layer)
    # adjacency is (a,b) edging to (a, b+1) and (a+1, b)
    idx = lambda a, b: a + b*w # linearize the indexing
    # nei = lambda a, b: ([(a, b+1)] + ([(a+1, b+1)] if a < w else [])) if b < d else []
    for layer_from in range(d):
        for within_from in range(w):
            frum = idx(within_from, layer_from)
            graph[(frum, frum)] = 0
            for layer_to in range(layer_from+1, d):
                for within_to in range(within_from, min(within_from+(layer_to-layer_from)+1, w)):
                    to = idx(within_to, layer_to)
                    graph[(frum, to)] = layer_to - layer_from
    return graph

def edgesFromDistances(d: Distances) -> Distances:
    return {
        (a, b): e
        for ((a, b), e) in d.items()
        if e == 1
    }

def rs_ncd(distances: Distances) -> Distances:
    D = max(distances.values())
    edges = edgesFromDistances(distances)
    return rs_ncd_impl(edges, distances, D+1)

def test_get_grid_graph():
    G1 = get_grid_graph(2, 2)
    assert G1 == {(0, 0): 0, (0, 2): 1, (0, 3): 1, (1, 1): 0, (1, 3): 1, (2, 2): 0, (3, 3): 0}
    G2 = get_grid_graph(3, 3)
    assert G2 == {(0, 0): 0, (0, 3): 1, (0, 4): 1, (0, 6): 2, (0, 7): 2, (0, 8): 2, (1, 1): 0, (1, 4): 1, (1, 5): 1, (1, 7): 2, (1, 8): 2, (2, 2): 0, (2, 5): 1, (2, 8): 2, (3, 3): 0, (3, 6): 1, (3, 7): 1, (4, 4): 0, (4, 7): 1, (4, 8): 1, (5, 5): 0, (5, 8): 1, (6, 6): 0, (7, 7): 0, (8, 8): 0}

def test_ncd():
    G1 = get_grid_graph(2, 2)
    R1pC = py_ncd_cubic(G1)
    assert R1pC == {(0, 0): 0, (0, 1): 1, (1, 0): 1, (0, 2): 1, (2, 0): 1, (0, 3): 1, (3, 0): 1, (1, 1): 0, (1, 2): 2, (2, 1): 2, (1, 3): 1, (3, 1): 1, (2, 2): 0, (2, 3): 2, (3, 2): 2, (3, 3): 0}
    R1pQ = py_ncd_quad(G1)
    assert R1pC == R1pQ
    R1pQn = py_ncd_quad_nonunit(G1)
    assert R1pC == R1pQn
    R1rC = rs_ncd(G1)
    assert R1rC == R1pC
    G2 = get_grid_graph(3, 3)
    R2pC = py_ncd_cubic(G2)
    assert R2pC == {(0, 0): 0, (0, 1): 1, (1, 0): 1, (0, 2): 2, (2, 0): 2, (0, 3): 1, (3, 0): 1, (0, 4): 1, (4, 0): 1, (0, 5): 2, (5, 0): 2, (0, 6): 2, (6, 0): 2, (0, 7): 2, (7, 0): 2, (0, 8): 2, (8, 0): 2, (1, 1): 0, (1, 2): 1, (2, 1): 1, (1, 3): 2, (3, 1): 2, (1, 4): 1, (4, 1): 1, (1, 5): 1, (5, 1): 1, (1, 6): 3, (6, 1): 3, (1, 7): 2, (7, 1): 2, (1, 8): 2, (8, 1): 2, (2, 2): 0, (2, 3): 3, (3, 2): 3, (2, 4): 2, (4, 2): 2, (2, 5): 1, (5, 2): 1, (2, 6): 3, (6, 2): 3, (2, 7): 3, (7, 2): 3, (2, 8): 2, (8, 2): 2, (3, 3): 0, (3, 4): 1, (4, 3): 1, (3, 5): 3, (5, 3): 3, (3, 6): 1, (6, 3): 1, (3, 7): 1, (7, 3): 1, (3, 8): 3, (8, 3): 3, (4, 4): 0, (4, 5): 1, (5, 4): 1, (4, 6): 3, (6, 4): 3, (4, 7): 1, (7, 4): 1, (4, 8): 1, (8, 4): 1, (5, 5): 0, (5, 6): 3, (6, 5): 3, (5, 7): 3, (7, 5): 3, (5, 8): 1, (8, 5): 1, (6, 6): 0, (6, 7): 3, (7, 6): 3, (6, 8): 3, (8, 6): 3, (7, 7): 0, (7, 8): 3, (8, 7): 3, (8, 8): 0}
    R2pQ = py_ncd_quad(G2)
    assert R2pC == R2pQ
    R2pQn = py_ncd_quad_nonunit(G2)
    assert R2pC == R2pQn
    R2rC = rs_ncd(G2)
    assert R2rC == R2pC

def bench_ncd(n, run_cubic: bool = True):
    s1 = perf_counter_ns()
    Gb = get_grid_graph(n, n)
    s2 = perf_counter_ns()
    if run_cubic:
        RbpC = py_ncd_cubic(Gb)
    s3 = perf_counter_ns()
    RbrC = rs_ncd(Gb)
    s4 = perf_counter_ns()
    RbpQ = py_ncd_quad(Gb)
    s5 = perf_counter_ns()
    RbpQn = py_ncd_quad_nonunit(Gb)
    s6 = perf_counter_ns()
    assert RbpQ == RbpQn
    if run_cubic:
        assert RbpQ == RbpC
    assert RbpQ == RbrC
    s7 = perf_counter_ns()
    print(f"generation took {ftime(s1, s2)}, pyC took {ftime(s2, s3)}, rsC took {ftime(s3, s4)}, pyQ took {ftime(s4, s5)}, pyQn took {ftime(s5, s6)}, compare took {ftime(s6, s7)}")


if __name__ == "__main__":
    if sys.argv[1] == "test":
        test_get_grid_graph()
        test_ncd()
    elif sys.argv[1] == "benchS":
        bench_ncd(13)
    elif sys.argv[1] == "benchM":
        # pyC took 18407.704 ms, rsC took 2742.516 ms, pyQ took 137.379 ms
        bench_ncd(20)
    elif sys.argv[1] == "benchL":
        bench_ncd(35, False)
