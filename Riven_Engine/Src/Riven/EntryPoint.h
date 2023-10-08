#pragma once
#include "Application.h"

#ifdef RV_PLATFORM_WINDOWS

// extern will allow our function Create Application to be gloval function
extern Riven::Application* Riven::CreateApplication();

int main() {
	auto app = Riven::CreateApplication();
	app->Run();
	delete app;

	return 0;
}

#endif // RV_PLATFORM_WINDOWS
