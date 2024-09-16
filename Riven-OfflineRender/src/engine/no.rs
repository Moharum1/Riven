// // Stages to build a PBRT system
// //TODO 1. Convert command line argument to a vector of string
// let args : Vec<String> = get_command_line_arguments();
// //TODO 2. Declare variable for parsed command line
// //TODO 3. Process command line argument
// let options = PBRTOptions::new(args);
// let files = GetSDFFiles();
// //TODO 4. Initialize the PBRT System
// InitPBRT(&options);
// //TODO 5. Parse the provided SDF ( Scene description file)
// let scene = Scene::new();
// let builder = SceneBuilder::new(&scene);
// ParseSDF(&builder, files.as_str());
// //TODO 6. Render the Scene
// if (options.use_gpu || options.wavefront){
//     RenderWaveFont(&scene) // Gpu based render
// }else {
//     RenderCpu(&scene)
// }
// //TODO 7. clean up after rendering the scene
// CleanUpPBRT()