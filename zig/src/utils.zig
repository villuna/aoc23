const std = @import("std");
const ArrayList = std.ArrayList;
const File = std.fs.File;

// Set up the allocator
var gpa = std.heap.GeneralPurposeAllocator(.{}){};
var alloc = gpa.allocator();

pub fn open_day_file(day: usize) !File {
    // Create a string to hold the filename and write the formatted path to it
    var path = ArrayList(u8).init(alloc);
    defer path.deinit();
    _ = try std.fmt.format(path.writer(), "../input/day{d}.txt", .{day});

    return std.fs.cwd().openFile(path.items, .{});
}
