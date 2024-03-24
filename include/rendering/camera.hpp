#pragma once
#include "component.hpp"
#include "util/types.hpp"

class Camera : Component::Component
{
    public:
    Camera() {}

    private:
    f32 fieldOfView;

};