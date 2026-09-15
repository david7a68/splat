---
splat_schema: 1
splat_id: 62806d64-6e3b-5709-9112-2ec2ac050032
splat_kind: spell
splat_key: dnd2024:spells/gust_of_wind
splat_name: Gust of Wind
splat_origin:
  key: dnd2024:reference/spell/gust_of_wind
splat_sources:
- title: SRD 5.2.1
  locator: pp. 138
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: evocation
  classes:
  - druid
  - ranger
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    mode: self
  material:
    description: a legume seed
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/gust_of_wind
  implementation: dnd2024:spell_gust_of_wind
  parameters:
    area:
      shape: line
      length_feet: 60
      width_feet: 10
      origin: caster
      direction_choice: caster
      duration: spell_duration
      wind_strength: strong
    save:
      ability: strength
      dc:
        binding: save_dc
      failure:
        push_feet: 15
        direction: away_from_caster_along_line
      success:
        effect: none
    save_triggers:
    - initial_creature_in_line
    - creature_turn_ends_in_line
    movement:
      moving_closer_to_caster_in_line:
        cost_feet_per_foot: 2
    environment:
      disperse:
      - gas
      - vapor
      unprotected_flames: extinguish
      protected_flames:
        flicker_wildly: true
        extinguish_chance_percent: 50
    redirect:
      cost: bonus_action
      timing: caster_later_turns
splat_contracts:
- id: dnd2024:spell_gust_of_wind
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    save_dc:
      type: integer
      required: true
    choices:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - spell.cast_completed
  - movement.completed
  - turn.end
  - save.before
  - save.after
  capabilities:
  - spellcasting
  - effects
  - dice
  - spatial_relationships
  - saving_throws
splat_conversion:
  status: specified
  category: spell
  source_sha256: cbc2b85164d19a72d972932f6bbb9cb4a91b7544028f73cb4cb618920ce70615
  source_path: builtin/srd-5.2.1/spell/gust_of_wind.md
  pages:
  - 138
  engine_gaps: &id001
  - dnd2024:spell_gust_of_wind
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 5ed20867ded86c83bcaff730551ceca893dd1d2aad7850d5c741d19199b160c2
splat_clauses:
- key: source_rules
  text: '# Gust of Wind Level 2 Evocation (Druid, Ranger, Sorcerer, Wizard) Casting Time: Action Range:
    Self Components: V, S, M (a legume seed) Duration: Concentration, up to 1 minute A Line of strong
    wind 60 feet long and 10 feet wide blasts from you in a direction you choose for the duration. Each
    creature in the Line must succeed on a Strength saving throw or be pushed 15 feet away from you in
    a direction following the Line. A creature that ends its turn in the Line must make the same save.
    Any creature in the Line must spend 2 feet of movement for every 1 foot it moves when moving closer
    to you. The gust disperses gas or vapor, and it extinguishes candles and similar unprotected flames
    in the area. It causes protected flames, such as those of lanterns, to dance wildly and has a 50 percent
    chance to extinguish them. As a Bonus Action on your later turns, you can change the direction in
    which the Line blasts from you.'
  sha256: ab67b453a2d8ae73f6ab336e2d0ddc0cecf652c25c4d9b2e8bd113604e5f49a4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gust of Wind

<!-- SRD page 138 -->

Level 2 Evocation (Druid, Ranger, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a legume seed)\
Duration: Concentration, up to 1 minute\
A Line of strong wind 60 feet long and 10 feet wide\
blasts from you in a direction you choose for the\
duration. Each creature in the Line must succeed on\
a Strength saving throw or be pushed 15 feet away\
from you in a direction following the Line. A crea-\
ture that ends its turn in the Line must make the\
same save.\
Any creature in the Line must spend 2 feet of\
movement for every 1 foot it moves when moving\
closer to you.\
The gust disperses gas or vapor, and it extin-\
guishes candles and similar unprotected flames in\
the area. It causes protected flames, such as those\
of lanterns, to dance wildly and has a 50 percent\
chance to extinguish them.\
As a Bonus Action on your later turns, you can\
change the direction in which the Line blasts from\
you.\

Original layout: [[../../srd-5.2.1/pages/138|page 138]].
