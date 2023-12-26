use std::{mem::size_of, num::NonZeroU64};

use nom::{
    bytes::complete::tag,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult, Parser,
};
use wgpu::{naga::FastHashMap, util::DeviceExt};

use crate::{
    parsers::{int, newline},
    AOContext,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    pos: [f64; 4],
    vel: [f64; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct DataStruct {
    min: f64,
    max: f64,
    length: [u32; 4],
}

async fn setup_wgpu() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        ..Default::default()
    });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: None,
            ..Default::default()
        })
        .await
        .unwrap();

    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::default() | wgpu::Features::SHADER_F64,
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap()
}

fn triple(input: &str) -> IResult<&str, [f64; 4]> {
    tuple((int::<i64>, tag(", "), int::<i64>, tag(", "), int::<i64>))
        .map(|(x, _, y, _, z)| [x as f64, y as f64, z as f64, 0.0])
        .parse(input)
}

async fn part1(stones: &[Vertex], ctx: &mut AOContext) {
    let (device, queue) = setup_wgpu().await;

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("compute shader"),
        source: wgpu::ShaderSource::Glsl {
            shader: include_str!("day24_shader.glsl").into(),
            stage: wgpu::naga::ShaderStage::Compute,
            defines: FastHashMap::default(),
        },
    });

    let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("input buffer"),
        contents: bytemuck::cast_slice(&stones),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });

    let result_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("result buffer"),
        size: (stones.len() * stones.len() * size_of::<u32>()) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("output buffer"),
        size: (stones.len() * stones.len() * size_of::<u32>()) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let input_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("input bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU64::new(
                            (stones.len() * size_of::<Vertex>()) as u64,
                        ),
                    },
                    count: None,
                },
            ],
        });

    let result_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("result bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(
                        (stones.len() * stones.len() * size_of::<u32>()) as u64,
                    ),
                },
                count: None,
            }],
        });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("compute pipeline layout"),
        bind_group_layouts: &[&input_bind_group_layout, &result_bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("compute pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "main",
    });

    let data_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("length buffer"),
        contents: bytemuck::cast_slice(&[DataStruct {
            min: 200000000000000.0,
            max: 400000000000000.0,
            length: [stones.len() as u32, 0, 0, 0],
        }]),
        usage: wgpu::BufferUsages::UNIFORM,
    });

    let input_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("input bind group"),
        layout: &input_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: data_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: input_buffer.as_entire_binding(),
            },
        ],
    });

    let result_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("result bind group"),
        layout: &result_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: result_buffer.as_entire_binding(),
        }],
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("command encoder"),
    });

    let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
        label: Some("compute pass"),
        timestamp_writes: None,
    });

    pass.set_pipeline(&compute_pipeline);
    pass.set_bind_group(0, &input_bind_group, &[]);
    pass.set_bind_group(1, &result_bind_group, &[]);
    pass.dispatch_workgroups(stones.len() as u32, stones.len() as u32, 1);

    drop(pass);

    encoder.copy_buffer_to_buffer(
        &result_buffer,
        0,
        &output_buffer,
        0,
        (size_of::<u32>() * stones.len() * stones.len()) as u64,
    );
    queue.submit(Some(encoder.finish()));

    let buf_slice = output_buffer.slice(..);
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    buf_slice.map_async(wgpu::MapMode::Read, move |v| tx.send(v).unwrap());
    device.poll(wgpu::Maintain::Wait);

    if let Ok(Ok(())) = rx.recv() {
        let data_raw = &*buf_slice.get_mapped_range();
        let data: &[u32] = bytemuck::cast_slice(data_raw);
        ctx.submit_part1(data.iter().sum::<u32>());
    }
}

pub fn day24(input: String, ctx: &mut AOContext) {
    let stones = separated_list1(
        newline,
        separated_pair(triple, tag(" @ "), triple).map(|(pos, vel)| Vertex { pos, vel }),
    )(&input)
    .unwrap()
    .1;
    ctx.parsing_done();

    pollster::block_on(part1(&stones, ctx));
    // Part 2 in mathematica
    // im sorry, i failed
    ctx.submit_part2("Sorry, I didn't do this one in rust");
}
