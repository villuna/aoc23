const std = @import("std");
const ArrayList = std.ArrayList;
const File = std.fs.File;
const expect = std.testing.expect;
const print = std.debug.print;

// Set up the allocator
var gpa = std.heap.GeneralPurposeAllocator(.{}){};
var alloc = gpa.allocator();

// I can't believe zig automatically formats this to be one big line
// what the fuck is going on
const replacements = [_]struct { before: []const u8, after: []const u8 }{ .{ .before = "one", .after = "1" }, .{ .before = "two", .after = "2" }, .{ .before = "three", .after = "3" }, .{ .before = "four", .after = "4" }, .{ .before = "five", .after = "5" }, .{ .before = "six", .after = "6" }, .{ .before = "seven", .after = "7" }, .{ .before = "eight", .after = "8" }, .{ .before = "nine", .after = "9" } };

fn find(haystack: []const u8, needle: []const u8) ?usize {
    if (needle.len > haystack.len)
        return null;

    var i: usize = 0;
    while (i < haystack.len - needle.len + 1) : (i += 1) {
        const found = haystack[i .. i + needle.len];

        var all = true;
        for (found, 0..) |char, j| {
            if (char != needle[j]) {
                all = false;
                break;
            }
        }

        if (all)
            return i;
    }

    return null;
}

fn find_last(haystack: []const u8, needle: []const u8) ?usize {
    if (needle.len > haystack.len)
        return null;

    var i: usize = haystack.len - needle.len;
    while (true) {
        const found = haystack[i .. i + needle.len];

        var all = true;
        for (found, 0..) |char, j| {
            if (char != needle[j]) {
                all = false;
                break;
            }
        }

        if (all)
            return i;

        if (i > 0) {
            i -= 1;
        } else {
            break;
        }
    }

    return null;
}

fn open_day_file(day: usize) !File {
    // Create a string to hold the filename and write the formatted path to it
    var path = ArrayList(u8).init(alloc);
    defer path.deinit();
    _ = try std.fmt.format(path.writer(), "../input/day{d}.txt", .{day});

    return std.fs.cwd().openFile(path.items, .{});
}

fn part1(line: []const u8) i32 {
    var first_num: i32 = -1;
    var last_num: i32 = -1;

    for (line) |ch| {
        if (std.ascii.isDigit(ch)) {
            if (first_num == -1)
                first_num = ch - '0';

            last_num = ch - '0';
        }
    }

    return first_num * 10 + last_num;
}

fn part2(line: []const u8) !i32 {
    var first_newline = ArrayList(u8).init(alloc);
    defer first_newline.deinit();

    var first_rep: ?usize = null;
    var first_pos: ?usize = null;
    for (replacements, 0..) |rep, i| {
        const mpos = find(line, rep.before);

        if (mpos) |pos| {
            if (first_rep == null or first_pos.? > pos) {
                first_rep = i;
                first_pos = pos;
            }
        }
    }

    if (first_rep != null) {
        try first_newline.appendSlice(line[0..first_pos.?]);
        try first_newline.appendSlice(replacements[first_rep.?].after);
        try first_newline.appendSlice(line[first_pos.? + replacements[first_rep.?].before.len ..]);
    } else {
        try first_newline.appendSlice(line);
    }

    var last_newline = ArrayList(u8).init(alloc);
    defer last_newline.deinit();

    var last_rep: ?usize = null;
    var last_pos: ?usize = null;
    for (replacements, 0..) |rep, i| {
        const mpos = find_last(line, rep.before);

        if (mpos) |pos| {
            if (last_rep == null or last_pos.? < pos) {
                last_rep = i;
                last_pos = pos;
            }
        }
    }

    if (last_rep != null) {
        try last_newline.appendSlice(line[0..last_pos.?]);
        try last_newline.appendSlice(replacements[last_rep.?].after);
        try last_newline.appendSlice(line[last_pos.? + replacements[last_rep.?].before.len ..]);
    } else {
        try last_newline.appendSlice(line);
    }

    var first_num: i32 = undefined;
    var last_num: i32 = undefined;

    for (first_newline.items) |char| {
        if (std.ascii.isDigit(char)) {
            first_num = char - '0';
            break;
        }
    }

    for (0..last_newline.items.len) |i| {
        const char = last_newline.items[last_newline.items.len - i - 1];
        if (std.ascii.isDigit(char)) {
            last_num = char - '0';
            break;
        }
    }

    return first_num * 10 + last_num;
}

fn day1() !void {
    const f = try open_day_file(1);
    var reader = f.reader();
    var p1: i32 = 0;
    var p2: i32 = 0;

    while (true) {
        var line = ArrayList(u8).init(alloc);
        defer line.deinit();

        var writer = line.writer();
        reader.streamUntilDelimiter(writer, '\n', null) catch |err| {
            try expect(err == error.EndOfStream);
            break;
        };

        p1 += part1(line.items);
        p2 += try part2(line.items);
    }

    print("part 1: {d}\npart 2: {d}\n", .{ p1, p2 });
}

pub fn main() !void {
    try day1();
}
