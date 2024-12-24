def bellman_ford(vertices, edges, source):
    distances = [float('inf')] * vertices
    distances[source] = 0

    for _ in range(vertices - 1):
        for u, v, weight in edges:
            if distances[u] != float('inf') and distances[v] > distances[u] + weight:
                distances[v] = distances[u] + weight

    for u, v, weight in edges:
        if distances[u] != float('inf') and distances[v] > distances[u] + weight:
            print("Negative weight cycle detected.")
            return [-1]

    return distances

def adjacent_list_to_edges(adj_list):
    edges = []
    for u in adj_list:
        for v, weight in adj_list[u]:
            edges.append((u, v, weight))
    return edges


# Main

adj_list = {
    0: [(1, -1), (2, 4)],
    1: [(2, 3), (3, 2), (4, 2)],
    2: [],
    3: [(2, 5), (1, 1)],
    4: [(3, -3)]
}

vertices = len(adj_list)

# Convert adjacency list to edge list
edges = adjacent_list_to_edges(adj_list)

source = 0

result = bellman_ford(vertices, edges, source)

if result != [-1]:
    print("Shortest distances from source: " ,source)
    print(result)
