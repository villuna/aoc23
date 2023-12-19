const std = @import("std");
const print = std.debug.print;
const utils = @import("utils.zig");
const expect = std.testing.expect;
const Split = utils.Split;

fn hash(str: []const u8) u8 {
    var res: u8 = 0;
    for (str) |c| {
        res +%= c;
        res *%= 17;
    }
    return res;
}

test "hash" {
    try expect(hash("rn=1") == 30);
    try expect(hash("cm-") == 253);
}

fn part1(input: []const u8) !void {
    var splits = Split.new(input, ',');
    var p1: u32 = 0;

    while (splits.next()) |next| {
        p1 += @as(u32, hash(next));
    }
    print("part 1: {d}\n", .{p1});
}

const Box = struct {
    const Node = struct {
        label: []const u8,
        value: u32,
        prev: ?*Node,
        next: ?*Node,
    };

    alloc: std.mem.Allocator,
    hashmap: std.StringHashMap(*Node),
    head: ?*Node,
    tail: ?*Node,

    fn init(alloc: std.mem.Allocator) !Box {
        return Box{
            .alloc = alloc,
            .hashmap = std.StringHashMap(*Node).init(alloc),
            .head = null,
            .tail = null,
        };
    }

    fn deinit(self: *Box) void {
        var next = self.head;
        while (next) |node| {
            next = node.next;
            self.alloc.destroy(node);
        }

        self.hashmap.deinit();
    }

    fn insert(self: *Box, label: []const u8, val: u32) !void {
        if (self.hashmap.contains(label)) {
            const node = self.hashmap.get(label).?;
            node.*.value = val;
        } else {
            const new_node = try self.alloc.create(Node);
            new_node.* = Node{
                .label = label,
                .value = val,
                .next = null,
                .prev = null,
            };

            if (self.tail) |tail| {
                tail.next = new_node;
                new_node.*.prev = tail;
            } else {
                self.head = new_node;
            }

            self.tail = new_node;
            try self.hashmap.put(label, new_node);
        }
    }

    fn remove(self: *Box, label: []const u8) void {
        if (self.hashmap.get(label)) |node| {
            if (self.head == node) {
                self.head = node.*.next;
            }

            if (self.tail == node) {
                self.tail = node.*.prev;
            }

            if (node.*.prev) |prev| {
                prev.*.next = node.*.next;
            }

            if (node.*.next) |next| {
                next.*.prev = node.*.prev;
            }

            _ = self.hashmap.remove(label);
            self.alloc.destroy(node);
        }
    }

    fn weighted_sum(self: *const Box) u32 {
        var sum: u32 = 0;
        var i: u32 = 1;
        var next = self.head;

        while (next) |node| : (i += 1) {
            sum += i * node.*.value;
            next = node.*.next;
        }

        return sum;
    }
};

fn part2(input: []const u8, alloc: std.mem.Allocator) !void {
    var splits = Split.new(input, ',');
    var boxes: [256]Box = undefined;
    for (0..256) |i| {
        boxes[i] = try Box.init(alloc);
    }

    while (splits.next()) |cmd| {
        if (cmd[cmd.len - 1] == '-') {
            const label = cmd[0 .. cmd.len - 1];
            const i = hash(label);

            boxes[i].remove(label);
        } else {
            var sides = Split.new(cmd, '=');
            const label = sides.next().?;
            const val = try std.fmt.parseInt(u32, sides.next().?, 10);
            const i = hash(label);

            try boxes[i].insert(label, val);
        }
    }

    var i: u32 = 0;
    var sum: u32 = 0;
    while (i < 256) : (i += 1) {
        sum += (i + 1) * boxes[i].weighted_sum();
        boxes[i].deinit();
    }

    print("part 2: {d}\n", .{sum});
}

pub fn day15() !void {
    // set up the allocator
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    // read the file into a string (for speed)
    var file = try utils.openDayFile(15, alloc);
    defer file.close();
    var input = std.ArrayList(u8).init(alloc);
    defer input.deinit();
    try file.reader().streamUntilDelimiter(input.writer(), '\n', null);

    try part1(input.items);
    try part2(input.items, alloc);
}
