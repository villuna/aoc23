const std = @import("std");
const utils = @import("utils.zig");
const ArrayList = std.ArrayList;
const print = std.debug.print;
const Vec2 = utils.Vec2;
const vec2 = utils.vec2;

const P2_CYCLE_LEN: usize = 1000000000;

fn HashSet(comptime T: type) type {
    return std.AutoHashMap(T, void);
}

fn part1(input: ArrayList(ArrayList(u8)), alloc: std.mem.Allocator) !void {
    var levels = ArrayList(usize).init(alloc);
    var load: usize = 0;
    // Not strictly necessary bc we're using an arena but good practice
    defer levels.deinit();

    const width = input.items[0].items.len;
    const height = input.items.len;
    try levels.appendNTimes(height, width);

    for (0.., input.items) |i, line| {
        const y = height - i;
        for (0.., line.items) |x, c| {
            switch (c) {
                'O' => {
                    load += levels.items[x];
                    levels.items[x] -= 1;
                },
                '#' => {
                    levels.items[x] = y - 1;
                },
                else => {},
            }
        }
    }

    print("part 1: {d}\n", .{load});
}

const Dir = enum(usize) {
    North,
    West,
    South,
    East,
};

const Grid = struct {
    width: usize,
    height: usize,
    map: ArrayList(u8),

    fn new(input: ArrayList(ArrayList(u8)), alloc: std.mem.Allocator) !Grid {
        const width = input.items[0].items.len;
        const height = input.items.len;
        var res = Grid{ .width = width, .height = height, .map = ArrayList(u8).init(alloc) };

        for (input.items) |line| {
            try res.map.appendSlice(line.items);
        }

        return res;
    }

    fn lookup(self: *Grid, x: usize, y: usize) ?*u8 {
        if (x >= 0 and x < self.width and y >= 0 and y < self.height) {
            return &self.map.items[y * self.width + x];
        } else {
            return null;
        }
    }

    fn print(self: *Grid) void {
        for (0..self.height) |j| {
            std.debug.print("{s}\n", .{self.map.items[j * self.width .. (j + 1) * self.width]});
        }
        std.debug.print("\n", .{});
    }

    fn cycle(self: *Grid) void {
        for (0..self.width) |x| {
            var level: usize = 0;
            for (0..self.height) |y| {
                const c = self.lookup(x, y).?;
                switch (c.*) {
                    '#' => {
                        level = y + 1;
                    },
                    'O' => {
                        c.* = '.';
                        self.lookup(x, level).?.* = 'O';
                        level += 1;
                    },
                    else => {},
                }
            }
        }

        for (0..self.height) |y| {
            var level: usize = 0;
            for (0..self.width) |x| {
                const c = self.lookup(x, y).?;
                switch (c.*) {
                    '#' => {
                        level = x + 1;
                    },
                    'O' => {
                        c.* = '.';
                        self.lookup(level, y).?.* = 'O';
                        level += 1;
                    },
                    else => {},
                }
            }
        }

        for (0..self.width) |x| {
            var level: usize = self.height - 1;
            for (0..self.height) |i| {
                const y = self.height - i - 1;
                const c = self.lookup(x, y).?;
                switch (c.*) {
                    '#' => {
                        if (y > 0)
                            level = y - 1;
                    },
                    'O' => {
                        c.* = '.';
                        self.lookup(x, level).?.* = 'O';
                        if (level > 0)
                            level -= 1;
                    },
                    else => {},
                }
            }
        }

        for (0..self.height) |y| {
            var level: usize = self.height - 1;
            for (0..self.width) |i| {
                const x = self.width - i - 1;
                const c = self.lookup(x, y).?;
                switch (c.*) {
                    '#' => {
                        if (x > 0)
                            level = x - 1;
                    },
                    'O' => {
                        c.* = '.';
                        self.lookup(level, y).?.* = 'O';
                        if (level > 0)
                            level -= 1;
                    },
                    else => {},
                }
            }
        }
    }
};

fn part2(input: ArrayList(ArrayList(u8)), alloc: std.mem.Allocator) !void {
    // These sorts of questions always have a trick, undoubtedly there'll be a cycle
    // we just need to find how long it takes to get to one
    var grid = try Grid.new(input, alloc);
    var visited = std.StringHashMap(usize).init(alloc);
    var order = ArrayList([]const u8).init(alloc);
    var i: usize = 0;
    var start: usize = undefined;
    while (true) : (i += 1) {
        grid.cycle();

        if (visited.get(grid.map.items)) |v| {
            start = v;
            break;
        } else {
            const map = try grid.map.clone();
            try visited.put(map.items, i);
            try order.append(map.items);
        }
    }

    const rem = (P2_CYCLE_LEN - 1 - start) % (i - start);
    const state = order.items[start + rem];

    var load: usize = 0;
    for (0..grid.height) |j| {
        const y = grid.height - j;
        for (0..grid.width) |x| {
            if (state[j * grid.width + x] == 'O') {
                load += y;
            }
        }
    }
    print("part 2: {d}\n", .{load});
}

pub fn day14() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    const alloc = arena.allocator();
    defer arena.deinit();

    const input = try utils.dayFileLines(14, alloc);
    try part1(input, alloc);
    try part2(input, alloc);
}
