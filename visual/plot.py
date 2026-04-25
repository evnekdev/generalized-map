# plotgraph.py

import sys

import networkx as nx
import matplotlib.pyplot as plt

def parse_nodes_and_edges(filename):
    with open(filename) as f:
        nnodes_, nedges_ = f.readline().split(' ');
        nnodes = int(nnodes_)
        nedges = int(nedges_)
        nodes = []
        edges = []
        for k in range(0,nnodes):
            nodes.append(f.readline().strip())
        for k in range(0,nedges):
            node1_, node2_ = f.readline().split(' ')
            edges.append((node1_.strip(), node2_.strip()))
    return nodes, edges
    
    
def plot_nodes_and_edges(nodes, edges):
    g =  nx.Graph();
    g.add_nodes_from(nodes)
    g.add_edges_from(edges)
    #pos = nx.spectral_layout(g)
    pos = nx.spring_layout(g)
    nx.draw(g, pos=pos, arrows=True, with_labels=True, node_color="salmon", edge_color="navy", font_color="purple")
    #nx.draw_networkx_edge_labels(g, pos=pos, edge_labels=edge_labels)
    plt.show()

def main():
    if len(sys.argv) < 2:
        print("Enter a filename as an input argument")
        exit()
    filename = sys.argv[1]
    nodes, edges = parse_nodes_and_edges(filename)
    plot_nodes_and_edges(nodes, edges)
        
if __name__ == "__main__":
    main()