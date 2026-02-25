mod sdl;

fn main() {
    unsafe {
        use sdl::*;
        SDL_Init(SDL_INIT_VIDEO);
        let win_ptr = SDL_CreateWindow((c"cmakr-demo").as_ptr(), 800, 600, 0);
        SDL_SetWindowPosition(win_ptr, 100, 100);
        SDL_ShowWindow(win_ptr);
        SDL_Delay(3000);
        SDL_DestroyWindow(win_ptr);
        SDL_Quit();
    }
}
