fn main() {
    println!("gpu scratch!");

    let desc = wgpu::InstanceDescriptor::new_without_display_handle();

    let instance = wgpu::Instance::new(desc);

    // instance.request_adapter()

    println!("instance = {:?}", instance);

    // instance.cmp

    // request_adapter(&self, options)

    // wgpu::Device::
}
