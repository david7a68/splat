---
splat_schema: 1
splat_id: 0375c0dd-6a44-4459-88ad-f23d9f156e05
splat_kind: item
splat_key: "my_homebrew:items/spear"
splat_name: Training Spear
splat_item:
  category: weapon
  mass: {value: 3, unit: lb}
  stackable: false
  equipment: {allowed: [carried, stored, wielded], hands: 1, attunement: none}
  weapon:
    category: simple_melee
    damage: {expression: "1d6", type: piercing}
    versatile: {expression: "1d8", hands: 2}
    thrown: {normal_feet: 20, long_feet: 60}
    mastery: sap
---

Ordinary weapon resolution uses these values and applicable proficiency rules.
The Warden's separate homebrew Strike supplies its own damage expression.
Possession of the spear alone does not grant mastery access.
