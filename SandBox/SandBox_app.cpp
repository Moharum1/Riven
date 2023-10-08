#include "Riven.h"

// create a sandbox class which will contains the app we will run using the Engine we create
// the sandbox will inherit the application class in our engine so it could get all the built in functions
class SandBox : public Riven::Application {

public:
	SandBox() {
	}

	~SandBox() {
	}


};

// overide the create application function in "Riven.h" and replace it with our own
// it will return an Riven application pointer because we will do heap allocation
Riven::Application* Riven::CreateApplication() {
	return new SandBox();
}