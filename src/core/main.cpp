#include "application.hpp"
#include <expected>
#include <print>

int main()
{
  Application app;
  app.Initialize();
  app.Run();
}
