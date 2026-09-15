---
splat_schema: 1
splat_id: 0375c0dd-6a44-4459-88ad-f23d9f156e08
splat_kind: item
splat_key: "my_homebrew:items/pouch"
splat_name: Training Pouch
splat_item:
  category: container
  mass: {value: 1, unit: lb}
  stackable: false
  equipment: {allowed: [carried, stored, worn], attunement: none}
  containment:
    capacity: {mass: {value: 6, unit: lb}}
    carried_mass: self_and_contents
    access: {mechanic: "dnd2024:container_access", state: closed}
---

A separate container instance, nested inside the supply pack in this example.
