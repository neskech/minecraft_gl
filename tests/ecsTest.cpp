#include "Ecs/component.hpp"
#include "Ecs/components/script.hpp"
#include "Ecs/entityComponentSystem.hpp"
#include "Ecs/entityManager.hpp"
#include "catch2/catch_test_macros.hpp"
#include "event/eventManager.hpp"
#include "pch.hpp"

//TODO
//Make ECS give out const references to children managers, e.g. entity manager
//Enhance entities with ability to modify themselves with an ECS reference

TEST_CASE("Add Entities", "[Ecs]")
{
  EntityComponentSystem ecs;
  Entity e1 = ecs.MakeEntity();
  Entity e2 = ecs.MakeEntity();
}

TEST_CASE("Add And Delete Entities", "[Ecs]")
{
  EntityComponentSystem ecs;
  Entity e1 = ecs.MakeEntity();
  ecs.DeleteEntity(e1);
  Entity e2 = ecs.MakeEntity();
  REQUIRE(e1 != e2);
}

TEST_CASE("Add Component", "[Ecs]")
{
  EntityComponentSystem ecs;
  Entity e1 = ecs.MakeEntity();
  Input i;
  ecs.AddComponent<Component::ScriptBase>(e1, ecs, i, e1);
  REQUIRE(ecs.HasComponent<Component::ScriptBase>(e1));
  Component::ScriptBase script = ecs.GetComponent<Component::ScriptBase>(e1);
}