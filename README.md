# Pathfinding

This project aims at comparing 3 pathfinding algorithms : Breadth first search, Heuristic search and A-star.

The program will display the length of the found path and how many steps were needed to find it on the given map.

# Folders

## src

- `src/main.rs`: the main program

## map

The folder with maps

# Usage

Create a topology of a map in a file:
- the first line contains the dimensions of the map (width and height)
- the second line contains the coordinates of the starting point and the exit point of the map (x and y coordinates of each point)
- the other lines are the layout of the map: one line for each map row, each line shall have as many space-separated cells as the map width, each cell shall be either 0 for walkable space or 1 for a wall.

Place map files into the `map` folder and run the program with the name of a map file as argument.

# Output

The program output depicts the result of each of the three algorithms as ascii art as well as the the number of steps and the length of the best found path. Finally a summury of the latter two informations is given as a table.
