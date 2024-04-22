use std::{
    fs::File, 
    io::Error, 
    sync::Arc
};
use maths::{Matrix4, Vector3};
use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder}
};
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer}, command_buffer::{
        allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo}, AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferToImageInfo, PrimaryCommandBufferAbstract
    }, descriptor_set::allocator::{
        StandardDescriptorSetAllocator,
        StandardDescriptorSetAllocatorCreateInfo
    }, device::{
        physical::PhysicalDeviceType,
        Device, DeviceCreateInfo,
        DeviceExtensions,
        Queue,
        QueueCreateInfo,
        QueueFlags
    }, format::Format, image::{sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo}, view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage}, instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, memory::allocator::{
        AllocationCreateInfo,
        MemoryTypeFilter,
        StandardMemoryAllocator
    }, pipeline::graphics::viewport::Viewport, swapchain::{acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo}, sync::{self, GpuFuture}, DeviceSize, Validated, VulkanError, VulkanLibrary
};


pub struct Renderer {
    swapchain: Arc<Swapchain>,
    images: Vec<Arc<ImageView>>,
    queue: Arc<Queue>,
    device: Arc<Device>,
    window: Arc<Window>,
    

    previous_frame_end: Option<Box<dyn GpuFuture>>,
    recreate_swapchain: bool
}

impl Renderer {
    pub fn new(
        print_device: bool,
        window_size: [u32; 2]
    ) -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new();


        let library = VulkanLibrary::new().unwrap();
        let required_extensions = Surface::required_extensions(&event_loop);
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions: required_extensions,
                ..Default::default()
            },
        ).unwrap();

        let window = Arc::new(WindowBuilder::new()
            .with_title("Erosion Simulation")
            .with_inner_size(PhysicalSize::new(window_size[0], window_size[1]))
            .with_resizable(false)
            .build(&event_loop)
            .unwrap());
        let surface = Surface::from_window(instance.clone(), window.clone()).unwrap();

        let (device, queue) = Self::get_device(instance, &surface, print_device);
        let (swapchain, images) = Self::get_swapchain(device.clone(), surface, window.clone());
        // let render_pass = Self::get_renderpass(device.clone(), &swapchain);
        // let pipeline = Self::get_pipeline(device.clone(), render_pass.clone());
        // let mut viewport = Viewport {
        //     offset: [0.0, 0.0],
        //     extent: [0.0, 0.0],
        //     depth_range: 0.0..=1.0
        // };

        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

        // let framebuffers = Self::get_framebuffers(&images, &render_pass, &mut viewport, memory_allocator.clone());


        let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
            device.clone(),
            StandardDescriptorSetAllocatorCreateInfo::default()
        ));
        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            StandardCommandBufferAllocatorCreateInfo {
                secondary_buffer_count: 32,
                ..Default::default()
            }
        ));

        let previous_frame_end = Some(sync::now(device.clone()).boxed());

        (Renderer {

            device,
            swapchain,
            images,
            queue,
            window,

            recreate_swapchain: false,
            previous_frame_end,
        },
        event_loop)
    }

    fn get_device(
        instance: Arc<Instance>,
        surface: &Surface,
        print_device: bool
    ) -> (Arc<Device>, Arc<Queue>){
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };

        let (physical_device, queue_family_index) = instance
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| {
                p.supported_extensions().contains(&device_extensions)
            })
            .filter_map(|p| {
                p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.intersects(QueueFlags::GRAPHICS)
                        && p.surface_support(i as u32, surface).unwrap_or(false)
                })
                .map(|i| (p, i as u32))
            })
            .min_by_key(|(p, _)| {
                match p.properties().device_type {
                    PhysicalDeviceType::DiscreteGpu => 0,
                    PhysicalDeviceType::IntegratedGpu => 1,
                    PhysicalDeviceType::VirtualGpu => 2,
                    PhysicalDeviceType::Cpu => 3,
                    PhysicalDeviceType::Other => 4,
                    _ => 5,
                }
            })
            .expect("No Suitable Physical Device Found");

        if print_device {
            println!(
                "Using device: {} (type: {:?})",
                physical_device.properties().device_name,
                physical_device.properties().device_type,
            );
        }

        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                enabled_extensions: device_extensions,
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            }
        ).unwrap();

        let queue = queues.next().unwrap();

        (device, queue)
    }

    fn get_swapchain(
        device: Arc<Device>,
        surface: Arc<Surface>,
        window: Arc<Window>
    ) -> (Arc<Swapchain>, Vec<Arc<ImageView>>) {
        let surface_capabilities = device
        .physical_device()
        .surface_capabilities(&surface, Default::default())
        .unwrap();

        let image_format = device
            .physical_device()
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0;

        let (swapchain, images) = Swapchain::new(
            device.clone(),
            surface,
            SwapchainCreateInfo {
                min_image_count: surface_capabilities.min_image_count.max(2),
                image_format,
                image_extent: window.inner_size().into(),
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha: surface_capabilities
                    .supported_composite_alpha
                    .into_iter()
                    .next()
                    .unwrap(),
                ..Default::default()
            }
        ).unwrap();

        let images = images
            .into_iter()
            .map(|image| ImageView::new_default(image).unwrap())
            .collect::<Vec<_>>();

        (swapchain, images)
    }

    pub fn recreate_swapchain(&mut self) {
        self.recreate_swapchain = true;
    }

    pub fn draw() {

    }
}



mod vs {
    vulkano_shaders::shader!{
        ty: "vertex",
        path: "assets/heightmap_vert.glsl"
    }
}

mod fs {
    vulkano_shaders::shader!{
        ty: "fragment",
        path: "assets/heightmap_frag.glsl"
    }
}