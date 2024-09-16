/// Options tells the program how some of it's functionality is going to work
struct EngineOptions{
    use_gpu: bool,
    save_photo : bool,
    rgb_render : bool,
    specular_render : bool
}

const BASIC_OPTIONS: EngineOptions = EngineOptions{
    use_gpu: false,
    save_photo: true,
    rgb_render: false,
    specular_render: false,
};