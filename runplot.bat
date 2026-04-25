REM run the rust main.exe from gmap package and plot the graph using Python's networkx

cd ../
cargo run -p gmap

cd ./gmap/visual
python plot.py nodes_and_edges.dat