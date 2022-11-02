import random


class Maze:
    def init_maze(self, MAP_N: int) -> list[list[int]]:
        maze: list[list[int]] = []
        for _ in range(0, MAP_N):
            maze.append([0 for _ in range(0, MAP_N)])

        for n in range(0, MAP_N):
            maze[n][0] = maze[n][MAP_N - 1] = 1
            maze[0][n] = maze[MAP_N - 1][n] = 1
        return maze

    def place_wall_in_maze(self, maze: list[list[int]], MAP_N: int) -> list[list[int]]:
        for y in range(2, MAP_N - 2):
            for x in range(2, MAP_N - 2):
                if x % 2 == 1 or y % 2 == 1:
                    continue
                maze[y][x] = 1
                r = random.randint(0, 3)
                if r == 0:
                    maze[y - 1][x] = 1
                if r == 1:
                    maze[y + 1][x] = 1
                if r == 2:
                    maze[y][x - 1] = 1
                if r == 3:
                    maze[y][x + 1] = 1
        return maze

    def print_maze(self, maze: list[list[int]], MAP_N: int):
        tiles = ["  ", "ZZ"]
        for y in range(0, MAP_N):
            for x in range(0, MAP_N):
                print(tiles[maze[y][x]], end="")
            print("")

    def create_maze(self, MAP_N: int = 25):
        maze = self.init_maze(MAP_N)
        maze = self.place_wall_in_maze(maze, MAP_N)
        self.print_maze(maze, MAP_N)


if __name__ == "__main__":
    Maze().create_maze()
