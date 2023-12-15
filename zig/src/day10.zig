const std = @import("std");
const utils = @import("utils.zig");

const File = std.fs.File;
const ArrayList = std.ArrayList;
const expect = std.testing.expect;
const openDayFile = utils.openDayFile;

pub const Vec2 = struct {
    x: isize,
    y: isize,

    fn eq(self: *const Vec2, other: *const Vec2) bool {
        return self.x == other.x and self.y == other.y;
    }
};

pub fn vec2(x: isize, y: isize) Vec2 {
    return Vec2{ .x = x, .y = y };
}

fn HashSet(comptime K: type) type {
    return std.AutoHashMap(K, void);
}

const Dir = enum(u8) { right = 1 << 0, down = 1 << 1, left = 1 << 2, up = 1 << 3 };

const connection_map = [_]struct { key: u8, connections: u8 }{
    .{ .key = '|', .connections = @intFromEnum(Dir.up) | @intFromEnum(Dir.down) },
    .{ .key = '-', .connections = @intFromEnum(Dir.left) | @intFromEnum(Dir.right) },
    .{ .key = 'L', .connections = @intFromEnum(Dir.up) | @intFromEnum(Dir.right) },
    .{ .key = 'J', .connections = @intFromEnum(Dir.up) | @intFromEnum(Dir.left) },
    .{ .key = 'F', .connections = @intFromEnum(Dir.down) | @intFromEnum(Dir.right) },
    .{ .key = '7', .connections = @intFromEnum(Dir.down) | @intFromEnum(Dir.left) },
    .{ .key = 'S', .connections = @intFromEnum(Dir.right) | @intFromEnum(Dir.down) | @intFromEnum(Dir.left) | @intFromEnum(Dir.up) },
    .{ .key = '.', .connections = 0 },
};

// If you are at a pipe of type given by character `char`, can you move in direction `dir`?
fn charConnects(char: u8, dir: Dir) bool {
    for (connection_map) |c| {
        if (c.key != char) {
            continue;
        }
        return (c.connections & @intFromEnum(dir)) != 0;
    }
    unreachable;
}

fn oppositeDirection(dir: Dir) Dir {
    return switch (dir) {
        Dir.up => Dir.down,
        Dir.left => Dir.right,
        Dir.right => Dir.left,
        Dir.down => Dir.up,
    };
}

// Returns the position as a result of moving in direction `dir` from position `pos`
fn adjacentPosition(pos: Vec2, dir: Dir) Vec2 {
    return switch (dir) {
        Dir.right => vec2(pos.x + 1, pos.y),
        Dir.down => vec2(pos.x, pos.y + 1),
        Dir.left => vec2(pos.x - 1, pos.y),
        Dir.up => vec2(pos.x, pos.y - 1),
    };
}

// A struct representing the state of the world
// contains the map of pipes (stores as a list of lines)
// and the starting position
const Environment = struct {
    map: ArrayList(ArrayList(u8)),
    start: Vec2,
    start_type: u8,

    fn isInBounds(self: *const Environment, pos: Vec2) bool {
        return pos.y >= 0 and pos.y < self.map.items.len and pos.x >= 0 and pos.x < self.map.items[@intCast(pos.y)].items.len;
    }

    // Returns the character at position `pos` if it exists and null otherwise
    fn charAt(self: *const Environment, pos: Vec2) ?u8 {
        if (self.isInBounds(pos)) {
            return self.map.items[@intCast(pos.y)].items[@intCast(pos.x)];
        } else {
            return null;
        }
    }

    // If the position `pos` is connected to a pipe via direction `dir`
    // in particular, this pipe must lead in that direction, and the pipe in that direction must lead
    // here.
    fn connects(self: *const Environment, pos: Vec2, dir: Dir) bool {
        const char = self.charAt(pos) orelse return false;
        const other_pos = adjacentPosition(pos, dir);

        if (self.charAt(other_pos)) |*other_char| {
            return charConnects(char, dir) and charConnects(other_char.*, oppositeDirection(dir));
        } else {
            return false;
        }
    }
};

// parses the input file
// `input` must be a reader type
fn parseInput(input: anytype, alloc: std.mem.Allocator) !Environment {
    var result = Environment{
        .map = ArrayList(ArrayList(u8)).init(alloc),
        .start = undefined,
        .start_type = undefined,
    };

    var y: isize = 0;
    while (true) {
        var line = ArrayList(u8).init(alloc);
        const writer = line.writer();

        input.streamUntilDelimiter(writer, '\n', null) catch |err| {
            try expect(err == error.EndOfStream);
            break;
        };

        if (utils.find(line.items, "S")) |*x| {
            result.start = vec2(@intCast(x.*), y);
        }

        try result.map.append(line);
        y += 1;
    }

    // Figure out what type of bend the start is supposed to be
    var directions: usize = undefined;
    for ([_]Dir{ Dir.up, Dir.down, Dir.left, Dir.right }) |d| {
        if (result.connects(result.start, d)) {
            directions |= @intFromEnum(d);
        }
    }

    for (connection_map) |info| {
        if (info.connections == directions) {
            result.start_type = info.key;
            break;
        }
    }

    return result;
}

// Since what's on your left and what's on your right depends on the direction you're moving in the
// pipe, I've decided that each tile will be assigned an arbitrary "forward direction", from which
// the adjacent tiles on the left and right can be identified
const adjacent_map = [_]struct { key: u8, fdir: Dir, left: []const Vec2, right: []const Vec2 }{
    .{ .key = '|', .fdir = Dir.up, .left = &[_]Vec2{vec2(-1, 0)}, .right = &[_]Vec2{vec2(1, 0)} },
    .{ .key = '-', .fdir = Dir.right, .left = &[_]Vec2{vec2(0, -1)}, .right = &[_]Vec2{vec2(0, 1)} },
    .{ .key = 'L', .fdir = Dir.left, .left = &[_]Vec2{ vec2(0, 1), vec2(-1, 1), vec2(-1, 0) }, .right = &[_]Vec2{} },
    .{ .key = 'J', .fdir = Dir.down, .left = &[_]Vec2{ vec2(1, 0), vec2(1, 1), vec2(0, 1) }, .right = &[_]Vec2{} },
    .{ .key = 'F', .fdir = Dir.up, .left = &[_]Vec2{ vec2(-1, 0), vec2(-1, -1), vec2(0, -1) }, .right = &[_]Vec2{} },
    .{ .key = '7', .fdir = Dir.right, .left = &[_]Vec2{ vec2(0, -1), vec2(1, -1), vec2(1, 0) }, .right = &[_]Vec2{} },
};

// Takes in a position and direction and pushes the adjacent empty spaces to two lists
// the empty spaces on the left of the creature will go to the `left` list, and right to `right`
// this way we separate the interior from the exterior
fn pushAdjacentEmpties(env: Environment, pos: Vec2, dir: Dir, left: *ArrayList(Vec2), right: *ArrayList(Vec2)) !void {
    var char = env.charAt(pos);

    if (char == 'S') {
        char = env.start_type;
    }

    for (adjacent_map) |info| {
        if (char == info.key) {
            var real_left = info.left;
            var real_right = info.right;

            if (info.fdir != dir) {
                real_left = info.right;
                real_right = info.left;
            }

            for (real_left) |adj_pos| {
                const transformed_pos = vec2(pos.x + adj_pos.x, pos.y + adj_pos.y);
                if (env.isInBounds(transformed_pos)) {
                    try left.append(transformed_pos);
                }
            }

            for (real_right) |adj_pos| {
                const transformed_pos = vec2(pos.x + adj_pos.x, pos.y + adj_pos.y);
                if (env.isInBounds(transformed_pos)) {
                    try right.append(transformed_pos);
                }
            }
            break;
        }
    }
}

fn part1(env: Environment, left: *ArrayList(Vec2), right: *ArrayList(Vec2), loop: *HashSet(Vec2)) !void {
    try loop.put(env.start, {});
    var direction = Dir.up;

    // I just want to find one direction that connects to the pipe and follow that
    for ([_]Dir{ Dir.up, Dir.down, Dir.left, Dir.right }) |d| {
        if (env.connects(env.start, d)) {
            direction = d;
            break;
        }
    }

    var position = adjacentPosition(env.start, direction);
    var distance: usize = 1;

    var j: usize = 0;
    // Keep moving along the pipe until we reach the start again
    // the length of the loop divided by two will be the furthest point
    // i believe the length will always be even! I think thats a graph theory thing or something
    while (!position.eq(&env.start)) {
        try loop.put(position, {});
        try pushAdjacentEmpties(env, position, direction, left, right);
        for ([_]Dir{ Dir.up, Dir.down, Dir.left, Dir.right }) |d| {
            if (d == oppositeDirection(direction)) {
                continue;
            }

            if (env.connects(position, d)) {
                position = adjacentPosition(position, d);
                direction = d;
                break;
            }
        }

        distance += 1;
        j += 1;
    }

    std.debug.print("part 1: {d}\n", .{distance / 2});
}

// Depth first search
// or as I knew it as when I discovered it as a teenager, "the minesweeper fill algorithm"
fn fill(env: Environment, loop: HashSet(Vec2), pos: Vec2, visited: *HashSet(Vec2)) !void {
    try visited.put(pos, {});

    for ([_]Dir{ Dir.right, Dir.down, Dir.left, Dir.up }) |d| {
        const next_pos = adjacentPosition(pos, d);

        if (env.isInBounds(next_pos) and !loop.contains(next_pos) and !visited.contains(next_pos)) {
            try fill(env, loop, next_pos, visited);
        }
    }
}

fn part2(env: Environment, lists: [2]ArrayList(Vec2), loop: HashSet(Vec2), alloc: std.mem.Allocator) !void {
    var visited = [2]HashSet(Vec2){ HashSet(Vec2).init(alloc), HashSet(Vec2).init(alloc) };

    for (0..2) |i| {
        for (lists[i].items) |pos| {
            if (!loop.contains(pos)) {
                try fill(env, loop, pos, &visited[i]);
            }
        }
    }

    // whichever one contains (0,0) is the exterior. the interior cannot contain border points.
    const interior: usize = if (visited[0].contains(vec2(0, 0)))
        1
    else if (visited[1].contains(vec2(0, 0)))
        0
    else
        @panic("one of the regions must contain (0,0)");

    std.debug.print("part 2: {d}\n", .{visited[interior].count()});
}

pub fn day10() !void {
    // Create an allocator
    // I'm using an arena allocator! because this makes memory management a lot easier
    // when you have a lot of little allocations that have to live a long time
    // for example, the list of strings that make up the map
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    const alloc = arena.allocator();
    defer arena.deinit();

    const file = try openDayFile(10, alloc);
    defer file.close();
    const env = try parseInput(file.reader(), alloc);

    // to keep track of where the loop is, and which tiles are on the lhs and rhs of the loop
    var loop = HashSet(Vec2).init(alloc);
    var left_list = ArrayList(Vec2).init(alloc);
    var right_list = ArrayList(Vec2).init(alloc);

    try part1(env, &left_list, &right_list, &loop);
    try part2(env, [_]ArrayList(Vec2){ left_list, right_list }, loop, alloc);
}
