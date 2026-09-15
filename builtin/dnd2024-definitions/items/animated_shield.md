---
splat_schema: 1
splat_id: 032e1ba9-a324-5547-aae0-6c6793e87061
splat_kind: item
splat_key: dnd2024:items/animated_shield
splat_name: Animated Shield
splat_origin:
  key: dnd2024:reference/magic_item/animated_shield
splat_sources:
- title: SRD 5.2.1
  locator: pp. 209
  url: https://www.dndbeyond.com/srd
splat_item:
  category: armor
  rarity: very_rare
  equipment:
    attunement: required
splat_rule:
  mechanic: dnd2024:items/animated_shield
  implementation: dnd2024:item_animated_shield
  parameters:
    activation:
      action: bonus_action
      requires: holding
      duration:
        value: 1
        unit: minute
      shield_state: animated
      location: wielder_space
      protection: as_wielded
      hands_required: 0
      end_on:
      - duration
      - bonus_action_end
      - wielder_death
      - wielder_incapacitated
      on_end:
        if_free_hand: return_to_hand
        otherwise: fall_to_ground
    attunement:
      required: true
splat_contracts:
- id: dnd2024:item_animated_shield
  inputs:
    user:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      required: false
      type: choice
  outputs:
  - effects
  - item_state
  events:
  - item.used
  capabilities:
  - equipment
  - item_resources
  - effects
splat_conversion:
  status: specified
  category: magic_item
  source_sha256: 472df42858740309aff24683609712a2167e9e7c21ef5559a6541c83f85dc986
  source_path: builtin/srd-5.2.1/magic_item/animated_shield.md
  pages:
  - 209
  engine_gaps: &id001
  - dnd2024:item_animated_shield
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/items_1.md
  specification_sha256: 62e44a646ba0b4e79768ea30e5f3d9e3032e8312cb198b0bc62c90a868e77cce
splat_clauses:
- key: source_rules
  text: '# Animated Shield Armor (Shield), Very Rare (Requires Attunement) While holding this Shield,
    you can take a Bonus Action to cause it to animate. The Shield leaps into the air and hovers in your
    space to protect you as if you were wielding it, leaving your hands free. The Shield remains animate
    for 1 minute, until you take a Bonus Action to end this effect, or until you die or have the Incapacitated
    condition, at which point the Shield falls to the ground or into your hand if you have one free.'
  sha256: 52b8bbf5a2095acacd19a485b80ec00187bc108b895cd96a1b82f680b8e04e96
  status: contract_specified
splat_requires_mechanics: *id001
---

# Animated Shield

<!-- SRD page 209 -->

Armor (Shield), Very Rare (Requires Attunement)\
While holding this Shield, you can take a Bonus\
Action to cause it to animate. The Shield leaps into\
the air and hovers in your space to protect you as if\
you were wielding it, leaving your hands free. The\
Shield remains animate for 1 minute, until you take\
a Bonus Action to end this effect, or until you die or\
have the Incapacitated condition, at which point the\
Shield falls to the ground or into your hand if you\
have one free.\

Original layout: [[../../srd-5.2.1/pages/209|page 209]].
