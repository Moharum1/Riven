#pragma once

#ifdef RV_PLATFORM_WINDOWS 
	#ifdef RV_BUILD_DLL
		#define RIVEN_API __declspec(dllexport)
	#else
		#define RIVEN_API __declspec(dllimport)
	#endif // DEBUG
#else
	#error Riven only support windows 
#endif