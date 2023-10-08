#pragma once
#include "Core.h"
#include <iostream>

namespace Riven {

	class RIVEN_API Application
	{
	public:	
		Application() {
		
		};
		virtual ~Application() {
		
		};
		void Run();
	};

	// to be defined by the client side 
	Application* CreateApplication();

}

