const std = @import("std");
const ArrayList = std.ArrayList;
const File = std.fs.File;

pub fn openDayFile(day: usize, alloc: std.mem.Allocator) !File {
    // Create a string to hold the filename and write the formatted path to it
    var path = ArrayList(u8).init(alloc);
    defer path.deinit();
    _ = try std.fmt.format(path.writer(), "../input/day{d}.txt", .{day});

    return std.fs.cwd().openFile(path.items, .{});
}

pub fn find(haystack: []const u8, needle: []const u8) ?usize {
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

pub fn findLast(haystack: []const u8, needle: []const u8) ?usize {
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

pub fn abs(x: anytype) @TypeOf(x) {
    if (x >= 0) {
        return x;
    } else {
        return -x;
    }
}
