---
splat_schema: 1
splat_id: 0375c0dd-6a44-4459-88ad-f23d9f156e07
splat_kind: item
splat_key: "my_homebrew:items/backpack"
splat_name: Training Backpack
splat_item:
  category: container
  mass: {value: 5, unit: lb}
  stackable: false
  equipment: {allowed: [carried, stored, worn], attunement: none}
  containment:
    capacity: {mass: {value: 30, unit: lb}}
    carried_mass: self_and_contents
    access: {mechanic: "dnd2024:container_access", state: closed}
---

The fixture constrains contents by mass only. No volume limit is inferred.
